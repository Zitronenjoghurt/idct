use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimensionID(String);

impl DimensionID {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self(id.into())
    }
}

impl From<String> for DimensionID {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for DimensionID {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for DimensionID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for DimensionID {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
