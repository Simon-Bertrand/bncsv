use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::{
    fs::{self, File},
    io::{Read, Write},
    sync::{mpsc, Arc},
    thread::{self, JoinHandle},
};

use bncsv_core::{compr::BnCsvConverter, fmt::utf8::Utf8Converter};

use super::Cli;
use crate::cli::utils;
use glob::GlobError;
use std::io;
#[derive(Debug)]
pub(crate) struct TaskQuery {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}
//pub type Channel = (Sender<TaskQuery>, Receiver<TaskQuery>);
type Channels = Vec<(Sender<TaskQuery>, Receiver<TaskQuery>)>;
impl Cli {
    pub(crate) fn convert_multithreaded(
        &self,
        input_paths: Result<Vec<PathBuf>, GlobError>,
    ) -> io::Result<()> {
        use crate::cli::print_file_result;

        use super::utils::FormatType;
        let tasks = input_paths
            .unwrap()
            .iter()
            .map(|x| TaskQuery {
                input_path: x.to_path_buf(),
                output_path: {
                    match &self.output {
                        Some(p) if x.is_relative() => p.join(x),
                        Some(p) if !x.is_relative() => p.join(
                            x.strip_prefix(self.abs_pathbase.as_ref().expect(
                                "Absolute path base is required when input path is absolute",
                            ))
                            .expect(&format!(
                                "Absolute path base <{:?}> is not found in the absolute input path {}",
                                self.abs_pathbase,
                                x.display()
                            )),
                        ),
                        _ => x.to_path_buf(),
                    }
                }
                .with_extension({
                    match self.input_type {
                        FormatType::Csv => "bncsv",
                        FormatType::Bncsv => "csv",
                    }
                }),
            })
            .collect::<Vec<TaskQuery>>();
        let n_tasks = tasks.len();
        let n_threads = self
            .jobs
            .map(|x| x.max(1))
            .unwrap_or(std::thread::available_parallelism().map_or(1, |x| x.get()))
            .min(n_tasks);

        //Create channel for each thread
        let channels: Channels = (0..n_threads)
            .map(|_| mpsc::channel::<TaskQuery>())
            .collect::<Channels>();

        if let Err(e) = self.validate_multithreaded_tasks_paths(&tasks) {
            return Err(e);
        }
        //Distribute tasks over channels
        tasks.into_iter().enumerate().for_each(|(i, query)| {
            channels[i % n_threads].0.send(query).unwrap();
        });
        let input_format = Arc::new(self.input_type.clone());

        // Run consummers
        let handles = channels
            .into_iter()
            .map(|(_, rx)| {
                let f = input_format.clone();
                thread::spawn(move || {
                    while let Ok(data) = rx.recv() {
                        let input_bytes =
                            File::options().read(true).open(&data.input_path)?.bytes();
                        if let Some(p) = &data.output_path.parent() {
                            if !p.exists() {
                                fs::create_dir_all(&p)?;
                            }
                        }

                        let res = utils::consume_iter_in_writer(
                            {
                                match f.as_ref() {
                                    FormatType::Csv => Box::new(Utf8Converter::encode(
                                        input_bytes
                                            .map(|x| x.expect("Could not read input utf-8 byte")),
                                    ))
                                        as Box<dyn Iterator<Item = std::io::Result<u8>>>,
                                    FormatType::Bncsv => Box::new(Utf8Converter::decode(
                                        input_bytes
                                            .map(|x| x.expect("Could not read input bncsv byte")),
                                    )),
                                }
                            },
                            &mut (Box::new(
                                File::options()
                                    .write(true)
                                    .create(true)
                                    .open(&data.output_path)?,
                            ) as Box<dyn Write>),
                        );

                        print_file_result(
                            &f,
                            &data.input_path,
                            &data.output_path,
                            res.is_ok(),
                            false,
                        );
                    }
                    Ok(())
                })
            })
            .collect::<Vec<JoinHandle<io::Result<()>>>>();

        handles.into_iter().for_each(|x| match x.join() {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                eprintln!("Error in thread {:?}", e);
            }
            Err(e) => {
                eprintln!("Error in thread {:?}", e);
            }
        });
        println!("{n_tasks} files conversion finished successfully on {n_threads} threads");
        Ok(())
    }
}
