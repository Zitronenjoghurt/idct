use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CuriosityTagID(String);

impl CuriosityTagID {
    pub fn set<S: Into<String>>(&mut self, id: S) {
        self.0 = id.into();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for CuriosityTagID {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for CuriosityTagID {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for CuriosityTagID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<String> for CuriosityTagID {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
