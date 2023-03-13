use std::{fmt::Display, path::Path};

use crate::parser::FireLocation;

pub enum CompileException<'a> {
    NoFileAccess(&'a Path),
    FileException(&'a Path),
    DuplicateResource(&'a FireLocation),
    UnrecognizedToken(String, usize, usize),
    UnfinishedString(usize, usize),
    UnrecognizedEscape(usize, usize)
}
impl Display for CompileException<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileException::NoFileAccess(p) => f.write_str(format!("Cannot read file {:?}.", p).as_str())?,
            CompileException::FileException(p) => f.write_str(format!("An error occured inside of {:?}.", p).as_str())?,
            CompileException::DuplicateResource(p) => f.write_str(format!("Duplicate resource {:?}.", p).as_str())?,
            CompileException::UnrecognizedToken(a, b, c) => f.write_str(format!("Unrecognized token {:?}. At {}:{}", a, b, c).as_str())?,
            CompileException::UnfinishedString(a, b) => f.write_str(format!("Unfinished string. Started at {}:{}", a, b).as_str())?,
            CompileException::UnrecognizedEscape(a, b) => f.write_str(format!("Unrecognized escape code. At {}:{}", a, b).as_str())?,
        };
        Ok(())
    }
}

// Represents the minecraft commands that we output
pub type CompiledCommands = Vec<String>;