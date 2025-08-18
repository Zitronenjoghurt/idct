use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CuriosityPropertyID(String);

impl CuriosityPropertyID {
    pub fn set(&mut self, id: impl Into<String>) {
        self.0 = id.into();
    }
}

impl From<String> for CuriosityPropertyID {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for CuriosityPropertyID {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for CuriosityPropertyID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for CuriosityPropertyID {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
