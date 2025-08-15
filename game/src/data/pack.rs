use crate::data::GameData;
use crate::error::{GameError, GameResult};
use crate::CORE_DATA_BYTES;
use serde::{Deserialize, Serialize};
use std::path::Path;

const ZSTD_COMPRESSION_LEVEL: i32 = 22;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataPack {
    pub meta: DataPackMeta,
    pub data: GameData,
}

impl DataPack {
    pub fn from_core_data() -> GameResult<Self> {
        Self::from_bytes(CORE_DATA_BYTES)
    }

    pub fn from_bytes(bytes: &[u8]) -> GameResult<Self> {
        Ok(rmp_serde::from_slice(&zstd::decode_all(bytes)?)?)
    }

    pub fn from_json(json: &str) -> GameResult<Self> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_bytes(&self) -> GameResult<Vec<u8>> {
        Ok(zstd::encode_all(
            &*rmp_serde::to_vec(&self)?,
            ZSTD_COMPRESSION_LEVEL,
        )?)
    }

    pub fn to_json(&self) -> GameResult<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    pub fn write_idct_file(&self, path: &Path) -> GameResult<()> {
        std::fs::write(path, self.to_bytes()?)?;
        Ok(())
    }

    pub fn write_json_file(&self, path: &Path) -> GameResult<()> {
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }

    pub fn from_file(path: &Path) -> GameResult<Self> {
        let extension = path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .ok_or_else(|| GameError::file_extension_read(path))?;

        match extension {
            "json" => Self::from_json(&std::fs::read_to_string(path)?),
            "idct" => Self::from_bytes(&std::fs::read(path)?),
            _ => Err(GameError::invalid_file_extension(extension)),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataPackMeta {
    pub name: String,
}
