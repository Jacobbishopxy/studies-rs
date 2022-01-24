//! https://github.com/dtolnay/syn/blob/master/examples/dump-syntax/src/main.rs

use std::fmt::Display;

enum Error {
    IncorrectUsage,
    ReadFile(std::io::Error),
    ParseFile {
        error: syn::Error,
        // filepath: PathBuf,
        source_code: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
