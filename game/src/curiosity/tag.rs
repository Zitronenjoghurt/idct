use serde::{Deserialize, Serialize};

pub mod rules;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagID(String);

impl TagID {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }
}

impl From<String> for TagID {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for TagID {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for TagID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for TagID {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
