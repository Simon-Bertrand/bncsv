use std::io::Write;

use crate::{compr::BnCsvConverter, fmt::utf8::Utf8Converter};

use super::{utils, Cli, FormatType};

impl Cli {
    pub(crate) fn write_to_output(
        &self,
        reader: Box<dyn Iterator<Item = std::io::Result<u8>>>,
        mut writer: Box<dyn Write>,
    ) -> std::io::Result<()> {
        let input_iter = reader.map(|x| x.expect("Could not read input byte"));
        utils::consume_iter_in_writer(
            {
                match self.input_type {
                    FormatType::Csv => Box::new(Utf8Converter::encode(input_iter))
                        as Box<dyn Iterator<Item = std::io::Result<u8>>>,
                    FormatType::Bncsv => Box::new(Utf8Converter::decode(input_iter)),
                }
            },
            &mut writer,
        )
    }
}
