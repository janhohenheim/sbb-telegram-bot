extern crate iron;
extern crate reqwest;
extern crate csv;

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

use self::iron::prelude::*;

#[derive(Debug)]
pub enum BroadcastErr {
    Csv(csv::Error),
    Iron(IronError),
    Parse(ParseIntError),
}

impl fmt::Display for BroadcastErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BroadcastErr::Csv(ref e) => e.fmt(f),
            BroadcastErr::Iron(ref e) => e.fmt(f),
            BroadcastErr::Parse(ref e) => e.fmt(f),
        }
    }
}
impl Error for BroadcastErr {
    fn description(&self) -> &str {
        "Something went wrong while doing csv stuff"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BroadcastErr::Csv(ref e) => Some(e),
            BroadcastErr::Iron(ref e) => Some(e),
            BroadcastErr::Parse(ref e) => Some(e),
        }
    }
}

impl From<csv::Error> for BroadcastErr {
    fn from(e: csv::Error) -> Self {
        BroadcastErr::Csv(e)
    }
}

impl From<IronError> for BroadcastErr {
    fn from(e: IronError) -> Self {
        BroadcastErr::Iron(e)
    }
}

impl From<ParseIntError> for BroadcastErr {
    fn from(e: ParseIntError) -> Self {
        BroadcastErr::Parse(e)
    }
}
