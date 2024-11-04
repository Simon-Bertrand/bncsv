use argh::FromArgValue;

use crate::utils::iterators::TryChunks;
use std::path::Path;
use std::{io::Write, path::PathBuf};
#[allow(non_upper_case_globals)]
pub const style_bold: &str = "\x1B[1m";
#[allow(non_upper_case_globals)]
pub const style_unbold: &str = "\x1B[21m";
pub const color_red: &str = "\x1B[31m";
#[allow(non_upper_case_globals)]
pub const color_green: &str = "\x1B[32m";
// #[allow(non_upper_case_globals)]
// pub const style_reset: &str = "\x1B[0m";
#[allow(non_upper_case_globals)]
pub const color_reset: &str = "\x1B[39m";
// #[allow(non_upper_case_globals)]
// pub const bg_red: &str = "\x1B[41m";
// #[allow(non_upper_case_globals)]
// pub const bg_green: &str = "\x1B[42m";
// pub const bg_reset: &str = "\x1B[49m";

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum FormatType {
    Csv,
    Bncsv,
}

impl FromArgValue for FormatType {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "csv" => Ok(FormatType::Csv),
            "bncsv" => Ok(FormatType::Bncsv),
            _ => Err(format!("Not implemented input type")),
        }
    }
}
pub(crate) fn consume_iter_in_writer(
    iter: impl Iterator<Item = std::io::Result<u8>>,
    writer: &mut Box<dyn Write>,
) -> Result<(), std::io::Error> {
    iter.try_chunks(2048).try_for_each(|x| {
        match x {
            Ok(x) => {
                writer
                    .write_all(&x)
                    .expect("Failed to write the output -> ");
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    })
}

fn shorten_path(path: &PathBuf) -> String {
    let path_str = path.to_string_lossy();
    if path_str.len() > 50 {
        format!("...{}", path_str.split_at(path_str.len() - 50).1)
    } else {
        path_str.to_string()
    }
}

pub(crate) fn print_file_result(
    input_type: &FormatType,
    path: &PathBuf,
    out_path: &PathBuf,
    success: bool,
    quiet: bool,
) {
    if quiet {
        return;
    }
    let convert_helper = match input_type {
        FormatType::Csv => "CSV->BNCSV",
        FormatType::Bncsv => "BNCSV->CSV",
    };
    match success {
        true => {
            println!("{style_bold}{color_green} ✅ [{convert_helper}] Success converting {} to {}{color_reset}{style_unbold}",
                     shorten_path(&path),
                     shorten_path(&out_path),
             );
        }
        false => {
            println!("{style_bold}{color_red} ❗ [{convert_helper}] Failed converting {} to {}{color_reset}{style_unbold}",
            shorten_path(&path),
            shorten_path(&out_path),
        );
        }
    }
}
