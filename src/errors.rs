use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {    
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        todo!() 
    }
}

impl ErrorTrait for Error {
    
}