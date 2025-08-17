use base64::Engine;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone)]
pub struct RandomSeed([u8; 32]);

impl RandomSeed {
    pub fn new(seed: [u8; 32]) -> Self {
        Self(seed)
    }

    pub fn random() -> Self {
        Self(rand::random())
    }

    pub fn create_rng(&self) -> StdRng {
        StdRng::from_seed(self.0)
    }

    pub fn to_base64(self) -> String {
        base64::engine::general_purpose::STANDARD.encode(self.as_ref())
    }

    pub fn from_base64(base64: &str) -> Result<Self, base64::DecodeError> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(base64)?;
        Ok(Self::from(bytes.as_slice()))
    }
}

impl Default for RandomSeed {
    fn default() -> Self {
        Self::random()
    }
}

impl AsRef<[u8]> for RandomSeed {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&[u8]> for RandomSeed {
    fn from(value: &[u8]) -> Self {
        let mut seed = [0u8; 32];
        let len = value.len().min(32);
        seed[..len].copy_from_slice(&value[..len]);
        Self(seed)
    }
}

impl From<u8> for RandomSeed {
    fn from(value: u8) -> Self {
        Self([value; 32])
    }
}

impl Serialize for RandomSeed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_base64())
    }
}

impl<'de> Deserialize<'de> for RandomSeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_base64(&s).map_err(serde::de::Error::custom)
    }
}
