use std::io;

pub struct TryChunked<I> {
    iter: I,
    size: usize,
}

pub trait TryChunks: Iterator + Sized {
    fn try_chunks(self, size: usize) -> TryChunked<Self> {
        TryChunked {
            iter: self,
            size,
        }
    }
}

impl<I, V> Iterator for TryChunked<I>
where
    I: Iterator<Item = io::Result<V>>,
{
    type Item = io::Result<Vec<V>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self
        .iter
        .by_ref()
        .take(self.size) 
        .collect::<io::Result<Vec<V>>>() {
            Ok(v) if v.is_empty() => None, 
            Ok(v) => Some(Ok(v)),           
            Err(e) => Some(Err(e)),
        }
    }
}

impl<V, I> TryChunks for I where I: Iterator<Item = io::Result<V>> {}

