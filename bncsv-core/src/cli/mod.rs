mod io;
#[cfg(feature = "multithreading")]
mod multithread;
mod utils;
mod validation;
use argh::FromArgs;
use glob::GlobError;
use std::fs::File;
use std::io::Write;
use std::{io::Read, path::PathBuf};
use utils::{print_file_result, FormatType};

#[derive(FromArgs)]
#[argh(description = "BNCSV Format CLI Tool")]
pub struct Cli {
    #[argh(positional, description = "input file glob paths")]
    pub paths: Vec<String>,

    #[argh(
        option,
        short = 'i',
        description = "type of input file : ['csv', 'bncsv']"
    )]
    pub input_type: FormatType,

    #[argh(option, short = 'o', description = "output path dir")]
    pub output: Option<PathBuf>,

    #[argh(option, description = "path base for absolute glob input paths")]
    pub abs_pathbase: Option<PathBuf>,

    #[argh(switch, short = 'p', description = "use stdin as input")]
    pub pipe: bool,

    #[argh(option, short = 'j', description = "number of jobs to run in parallel")]
    pub jobs: Option<usize>,
}

impl Cli {
    pub(crate) fn new() -> Self {
        let mut strings: Vec<String> = std::env::args_os()
            .map(|s| s.into_string())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|arg| {
                eprintln!("Invalid utf8 argument char: {}", arg.to_string_lossy());
                std::process::exit(1)
            });

        if strings.is_empty() {
            eprintln!("No program name, argv is empty");
            std::process::exit(1);
        }

        if strings.len() == 1 {
            strings.push("--help".into());
        }
        let cmd = std::path::Path::new(&strings[0])
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&strings[0]);
        let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        Self::from_args(&[cmd], &strs[1..]).unwrap_or_else(|early_exit| {
            std::process::exit(match early_exit.status {
                Ok(()) => {
                    println!("{}", early_exit.output);
                    0
                }
                Err(()) => {
                    eprintln!(
                        "{}\nRun {} --help for more information.",
                        early_exit.output, cmd
                    );
                    1
                }
            })
        })
    }

    pub(crate) fn entrypoint(&self) -> std::io::Result<()> {
        if !self.pipe && self.paths.len() == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Input file glob paths are required",
            ));
        }

        let input_paths = self
            .pipe
            .then_some(&vec![]) // Pipe is true implies input_paths is empty and n_files =0
            .unwrap_or(&self.paths)
            .iter()
            .map(|input| glob::glob(&input))
            .map(|x| x.map(|y| y.collect::<Result<Vec<PathBuf>, GlobError>>()))
            .flatten()
            .collect::<Result<Vec<Vec<PathBuf>>, GlobError>>()
            .map(|x| x.into_iter().flatten().collect::<Vec<PathBuf>>());
        let n_files = input_paths.as_ref().map(|res| res.len()).unwrap_or(0); //Any error results in a len of zero
        match (n_files, self.pipe) {
            // (n_files, self.pipe) are partially redundant but this way is supposed to be more readable
            (0, true) => self.write_to_output(
                Box::new(std::io::stdin().bytes()),
                if self.output.is_none() {
                    Box::new(std::io::stdout())
                } else {
                    Box::new(
                        File::options()
                            .write(true)
                            .create(true)
                            .open(&self.output.as_ref().unwrap())?,
                    )
                },
            )?,
            (1, false) => {
                let p: &PathBuf = &input_paths.unwrap()[0];
                match &self.output {
                    Some(ref out_p) if !out_p.is_dir() => {
                        print_file_result(
                            &self.input_type,
                            p,
                            &out_p,
                            {
                                self.write_to_output(
                                    Box::new(File::options().read(true).open(p)?.bytes()),
                                    Box::new(
                                        File::options().write(true).create(true).open(&out_p)?,
                                    ),
                                )
                                .is_ok()
                            },
                            self.output.is_none(),
                        );
                    }
                    None => {
                        self.write_to_output(
                            Box::new(File::options().read(true).open(p)?.bytes()),
                            Box::new(std::io::stdout()),
                        )?;
                        std::io::stdout().flush()?;
                    }
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid output path",
                        ));
                    }
                }
            }
            (d, false) if d > 1 => {
                #[cfg(not(feature = "multithreading"))]
                {
                    // TODO : Implement a simple for loop to iterate over input_paths
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Multiple input files are not supported without multithreading build feature",
                    ));
                }
                #[cfg(feature = "multithreading")]
                self.convert_multithreaded(input_paths)?;
            }
            (0, false) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "No input files found",
                ));
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid given arguments",
                ));
            }
        }
        Ok(())
    }
}
