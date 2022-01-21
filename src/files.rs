use std::{fs, env};
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::bse::http::BasisSetExchange;

pub const ATO_ENV_VAR: &str = "ATO_DATA_PATH";
pub const DEFAULT_DATA_PATH: &str = ".ato_rs/data/";
pub const BASIS_SUBDIRECTORY: &str = "basis_sets/";
pub const BINCODE_BASIS_DICT: &str = "basis_set_dict.bc";
pub const JSON_BASIS_DICT: &str = "basis_set_dict.json";

pub enum BasisSetData {
    Downloaded,
    ExistsAlready,
}

#[tokio::main]
pub async fn update_data() -> Result<()> {
    if let Ok(BasisSetData::ExistsAlready) = ensure_data_exist() {
        BasisSetExchange::download_metadata().await?;
        BasisSetExchange::download_basis_sets().await?;
    }
    Ok(())
}

#[tokio::main]
pub async fn ensure_data_exist() -> Result<BasisSetData> {
    if data_exists()? {
        return Ok(BasisSetData::ExistsAlready);
    }
    create_data_dir()?;
    BasisSetExchange::download_metadata().await?;
    BasisSetExchange::download_basis_sets().await?;
    Ok(BasisSetData::Downloaded)
}

pub fn create_data_dir() -> Result<()> {
    let mut path = data_path("")?;
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir(parent).context("Could not create parent dir")?
        }
    };
    fs::create_dir(&path).context("Could not create data dir")?;
    path.push("basis_sets/");
    fs::create_dir(&path).context("Could not create basis set dir")?;
    Ok(())
}

pub fn data_exists() -> Result<bool> {
    let path = data_path(BASIS_SUBDIRECTORY)?;
    if !path.exists() {
        return Ok(false);
    }
    Ok(path.read_dir()?.next().is_some() && data_path(BINCODE_BASIS_DICT)?.exists())
}

pub fn data_path(suffix: &str) -> Result<PathBuf> {
    let mut path = match env::var(ATO_ENV_VAR) {
        Ok(value) => PathBuf::from(value),
        Err(_) => {
            let mut home = dirs::home_dir().context("Could not resolve the home directory")?;
            home.push(DEFAULT_DATA_PATH);
            home
        }
    };
    path.push(suffix);
    Ok(path)
}