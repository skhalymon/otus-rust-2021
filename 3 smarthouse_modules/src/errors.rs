use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Debug)]
pub enum SmartHouseError {
    AlreadyAdded(String),
    NotFound(String),
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match &self {
            Self::AlreadyAdded(name) => format!("Name {} already added", name),
            Self::NotFound(name) => format!("Name {} not found", name),
        };
        writeln!(f, "{}", msg)
    }
}

impl std::error::Error for SmartHouseError {}
