mod compr;
mod fmt;
mod utils;

use std::io;
#[cfg(not(feature = "cli"))]
fn main() {
    panic!("This binary was built without the 'cli' feature. The executable is not meant to be run directly in this case.")
}

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
use cli::Cli;
#[cfg(feature = "cli")]
fn main() -> io::Result<()> {
    Cli::new().entrypoint()
}
