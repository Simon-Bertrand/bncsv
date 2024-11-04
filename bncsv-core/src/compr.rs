use std::{io, iter};

use crate::utils::iterators::TryChunks;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Symbol {
    pub bits: &'static [u8],
    pub value: u8,
}
pub trait BnCsvConverter {
    fn encode(raw_data: impl IntoIterator<Item = u8>) -> impl Iterator<Item = std::io::Result<u8>>;
    fn decode(data: impl IntoIterator<Item = u8>) -> impl Iterator<Item = std::io::Result<u8>>;
}

pub struct DecoderUnfold<I: Iterator<Item = u8>> {
    iter: I,
    decoding_tree: DecodingTree,
    eoc_symbol: &'static Symbol,
}
impl<I: Iterator<Item = u8>> DecoderUnfold<I> {
    pub fn new(iter: I, decoding_tree: DecodingTree, eoc_symbol: &'static Symbol) -> Self {
        DecoderUnfold {
            iter,
            decoding_tree,
            eoc_symbol,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for DecoderUnfold<I>
where
    I: Iterator<Item = u8>,
{
    type Item = std::io::Result<Symbol>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut loc_target = Box::new(&self.decoding_tree);
        for bit in self.iter.by_ref() {
            if bit == 0 && loc_target.down.is_some() {
                *loc_target = loc_target.down.as_ref().unwrap();
            } else if bit == 1 && loc_target.up.is_some() {
                *loc_target = loc_target.up.as_ref().unwrap();
            } else {
                return Some(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid character {} encountered in the input data", bit),
                )));
            }

            match loc_target.root {
                Some(symb) if symb == *self.eoc_symbol => {
                    return None;
                }
                Some(symb) => {
                    return Some(Ok(symb));
                }
                None => {}
            }
        }
        None
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DecodingTree {
    pub root: Option<Symbol>,
    pub up: Option<Box<DecodingTree>>,
    pub down: Option<Box<DecodingTree>>,
}

impl DecodingTree {
    fn init(depth: usize, symbols: Vec<Symbol>) -> DecodingTree {
        if symbols.len() == 1 {
            return DecodingTree {
                root: Some(symbols[0]),
                down: None,
                up: None,
            };
        }
        let down_part: Vec<Symbol> = symbols
            .clone()
            .into_iter()
            .filter(|x| x.bits.get(depth) == Some(&0_u8))
            .collect();
        let up_part: Vec<Symbol> = symbols
            .into_iter()
            .filter(|x| x.bits.get(depth) == Some(&1_u8))
            .collect();
        DecodingTree {
            root: None,
            down: {
                if !down_part.is_empty() {
                    Some(Box::new(DecodingTree::init(depth + 1, down_part)))
                } else {
                    None
                }
            },
            up: {
                if !up_part.is_empty() {
                    Some(Box::new(DecodingTree::init(depth + 1, up_part)))
                } else {
                    None
                }
            },
        }
    }

    pub fn new(symbols: &[Symbol]) -> DecodingTree {
        DecodingTree::init(0, symbols.to_vec())
    }
}
pub fn encode<'a>(
    lookup_table: &'static [Result<&[u8], &str>],
    eoc_symb: &'static Symbol,
    data: impl IntoIterator<Item = u8>,
) -> impl Iterator<Item = std::io::Result<u8>> {
    // Reads a interator of u8, encode them using the lookup table and return a bytes stream with the encoded data
    // and EOC symbol concatenated at the end of the stream.
    // The output bytes are valid-size bytes of the bits sent and ready to be saved in a file.
    data.into_iter()
        .flat_map(|x: u8| -> Box<dyn Iterator<Item = Result<u8, io::Error>>> {
            match lookup_table.get(usize::from(x)) {
                Some(Ok(y)) => Box::new(y.iter().map(|&y| Ok(y))),
                Some(Err(_)) | None => Box::new(iter::once(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid character encountered in the input data",
                )))),
            }
        })
        .chain(eoc_symb.bits.iter().map(|&y| Ok(y)))
        .try_chunks(8)
        .map(|x| match x {
            Ok(mut buff) => {
                if buff.len() != 8 {
                    buff.extend(iter::repeat(0_u8).take(8 - buff.len()));
                }
                Ok(buff
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc_res, (i, curr)| acc_res | (curr << (7 - i))))
            }
            Err(e) => Err(e),
        })
}

pub fn decode(
    decoding_tree: DecodingTree,
    eoc_symbol: &'static Symbol,
    data: impl IntoIterator<Item = u8>,
) -> impl Iterator<Item = std::io::Result<u8>> {
    // Reads an encoded iterator of 0_u8 and 1_u8, binary search for the corresponding utf_8 value and return a stream of those utf_8 values.
    DecoderUnfold::new(data.into_iter(), decoding_tree, eoc_symbol).map(|x| match x {
        Ok(symb) => Ok(symb.value),
        Err(e) => Err(e),
    })
}
