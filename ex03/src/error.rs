pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidFormulaSyntax,
    InvalidFormulaGrammar,
    InvalidFormulaResult,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidFormulaSyntax => write!(f, "Formula contains unknown characters"),
            Error::InvalidFormulaGrammar => write!(f, "Formula grammar cannot be resolved"),
            Error::InvalidFormulaResult => write!(f, "Formula result is invalid"),
        }
    }
}

impl std::error::Error for Error {}
