use std::fmt;

#[derive(Debug)]
pub enum ExceptionType {
    AddressOutOfRange,
    StackOverflow,
    StackPointerOutOfRange,
    SDL,
    BadArgument,
    BadInstruction,
    Other
}

#[derive(Debug)]
pub struct Exception {
    exception_type: ExceptionType,
}

impl Exception {
    pub(crate) fn new(p0: ExceptionType) -> Exception {
        Exception {
            exception_type: p0,
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Exception: {:?}",
               self.exception_type)
    }
}