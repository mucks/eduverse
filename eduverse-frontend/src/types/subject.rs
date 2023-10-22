use std::fmt::Display;

use enum_iterator::Sequence;
use serde::Serialize;

#[derive(FromPrimitive, ToPrimitive, Sequence, Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Subject {
    English = 1,
    Science = 2,
    History = 3,
    Geography = 4,
    Art = 5,
    Music = 6,
    PhysicalEducation = 7,
    ComputerScience = 8,
    Math = 9,
    Other = 10,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
