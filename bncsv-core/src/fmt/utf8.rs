use std::io::{self};

use crate::compr::{decode, encode, BnCsvConverter, DecodingTree, Symbol};
pub const SYMB_COMMA: Symbol = Symbol {
    bits: &[0, 0, 1],
    value: 44,
};
pub const SYMB_DOT: Symbol = Symbol {
    bits: &[0, 1, 0],
    value: 46,
};
pub const SYMB_NEWLINE: Symbol = Symbol {
    bits: &[0, 1, 1, 0, 0, 1],
    value: 10,
};
pub const SYMB_MINUS: Symbol = Symbol {
    bits: &[0, 1, 1, 0, 1],
    value: 45,
};
pub const SYMB_5: Symbol = Symbol {
    bits: &[0, 0, 0],
    value: 53,
};
pub const SYMB_0: Symbol = Symbol {
    bits: &[0, 1, 1, 1],
    value: 48,
};
pub const SYMB_1: Symbol = Symbol {
    bits: &[1, 0, 1, 0],
    value: 49,
};
pub const SYMB_2: Symbol = Symbol {
    bits: &[1, 1, 1, 0],
    value: 50,
};
pub const SYMB_3: Symbol = Symbol {
    bits: &[1, 1, 0, 1],
    value: 51,
};
pub const SYMB_4: Symbol = Symbol {
    bits: &[1, 0, 0, 1],
    value: 52,
};
pub const SYMB_6: Symbol = Symbol {
    bits: &[1, 1, 0, 0],
    value: 54,
};
pub const SYMB_7: Symbol = Symbol {
    bits: &[1, 1, 1, 1],
    value: 55,
};
pub const SYMB_8: Symbol = Symbol {
    bits: &[1, 0, 1, 1],
    value: 56,
};
pub const SYMB_9: Symbol = Symbol {
    bits: &[1, 0, 0, 0],
    value: 57,
};
pub const SYMB_EOC: Symbol = Symbol {
    bits: &[0, 1, 1, 0, 0, 0],
    value: 45,
};

const INVALID_SYMBOL: Result<&[u8], &str> = Err("Invalid character encountered in the input data");

const fn build_utf8_lookup_table() -> [Result<&'static [u8], &'static str>; 255] {
    let mut table = [INVALID_SYMBOL; 255];
    table[10] = Ok(SYMB_NEWLINE.bits);
    table[13] = Ok(&[]);
    table[44] = Ok(SYMB_COMMA.bits);
    table[45] = Ok(SYMB_MINUS.bits);
    table[46] = Ok(SYMB_DOT.bits);
    table[48] = Ok(SYMB_0.bits);
    table[49] = Ok(SYMB_1.bits);
    table[50] = Ok(SYMB_2.bits);
    table[51] = Ok(SYMB_3.bits);
    table[52] = Ok(SYMB_4.bits);
    table[53] = Ok(SYMB_5.bits);
    table[54] = Ok(SYMB_6.bits);
    table[55] = Ok(SYMB_7.bits);
    table[56] = Ok(SYMB_8.bits);
    table[57] = Ok(SYMB_9.bits);
    table
}
static UTF_8_LOOKUP_ENCODING_TABLE: [Result<&[u8], &str>; 255] = build_utf8_lookup_table();

pub struct Utf8Converter;
impl BnCsvConverter for Utf8Converter {
    fn encode(raw_data: impl IntoIterator<Item = u8>) -> impl Iterator<Item = std::io::Result<u8>> {
        encode(&UTF_8_LOOKUP_ENCODING_TABLE, &SYMB_EOC, raw_data)
    }
    fn decode(data: impl IntoIterator<Item = u8>) -> impl Iterator<Item = std::io::Result<u8>> {
        decode(
            DecodingTree::new(&[
                SYMB_0,
                SYMB_1,
                SYMB_2,
                SYMB_3,
                SYMB_4,
                SYMB_5,
                SYMB_6,
                SYMB_7,
                SYMB_8,
                SYMB_9,
                SYMB_COMMA,
                SYMB_DOT,
                SYMB_MINUS,
                SYMB_NEWLINE,
                SYMB_EOC,
            ]),
            &SYMB_EOC,
            data.into_iter()
                .map(|b| Ok::<Vec<u8>, io::Error>((0..=7).rev().map(|i| b >> i & 1_u8).collect()))
                .flat_map(|x| x.expect("Failed to read bytes")),
        )
    }
}
