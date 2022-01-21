use crate::bse::basisset_json::InputData;
use crate::bse::metadata_json::InputMetaData;
use anyhow::{Context, Result};
use bincode::serialize_into;
use reqwest::{Url, Client};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{env, fs};
use futures::stream::StreamExt;
use crate::files::{data_path, BASIS_SUBDIRECTORY, JSON_BASIS_DICT, BINCODE_BASIS_DICT, ensure_data_exist};

pub const BSE_BASE_URL: &str = "https://www.basissetexchange.org/";
pub const BSE_BASIS_SUFFIX: &str = "/format/json/?";
pub const BSE_BASIS: &str = "/api/basis/";
pub const BSE_METADATA: &str = "/api/metadata/";

pub struct BasisSetExchange {}

impl BasisSetExchange {
    async fn request_names() -> Result<BTreeMap<String, String>> {
        let resp = reqwest::get(format!("{}{}", BSE_BASE_URL, BSE_METADATA)).await?
            .json::<BTreeMap<String, InputMetaData>>().await?;
        Ok(resp
            .keys()
            .zip(resp.values())
            .map(|(k, v)| (v.basename.to_lowercase(), k.clone()))
            .collect::<BTreeMap<String, String>>())
    }

    async fn basis_set_urls() -> Result<Vec<(String, String)>> {
        let names = BasisSetExchange::request_names().await?;
        let path = data_path(BASIS_SUBDIRECTORY)?;
        Ok(names.values().map(|name| (format!(
            "{}{}{}{}",
            BSE_BASE_URL, BSE_BASIS, &name, BSE_BASIS_SUFFIX),
                                      format!("{}{}.json", &path.to_str().unwrap(), name))
        ).collect::<Vec<(String, String)>>())
    }

    pub async fn download_basis_sets() -> Result<()> {
        let paths: Vec<(String, String)> = BasisSetExchange::basis_set_urls().await?;
        let client = Client::builder().build()?;
        let fetches = futures::stream::iter(
            paths.into_iter().map(|(url, path)| {
                let send_fut = client.get(&url).send();
                async move {
                    match send_fut.await {
                        Ok(resp) => {
                            match resp.bytes().await {
                                Ok(bytes) => {
                                    fs::write(&path, bytes).unwrap();
                                }
                                Err(_) => println!("ERROR reading {}", url),
                            }
                        }
                        Err(_) => println!("ERROR downloading {}", url),
                    }
                }
            })
        ).buffer_unordered(30).collect::<Vec<()>>();
        fetches.await;
        Ok(())
    }

    pub async fn download_metadata() -> Result<()> {
        let names = BasisSetExchange::request_names().await?;
        let json_path = data_path(JSON_BASIS_DICT)?;
        let bincode_path = data_path(BINCODE_BASIS_DICT)?;
        let mut f = BufWriter::new(File::create(&json_path).unwrap());
        serde_json::to_writer(f, &names);
        let mut f = BufWriter::new(File::create(&bincode_path).unwrap());
        serialize_into(&mut f, &names)?;
        Ok(())
    }

    fn read_names() -> Result<BTreeMap<String, String>> {
        let path = data_path(BINCODE_BASIS_DICT)?;
        let data = fs::read(&path).context("Unable to read file")?;
        Ok(bincode::deserialize(&data).context("Could not deserialize basis set names")?)
    }

    pub fn read_basis(name: &str) -> Result<InputData> {
        ensure_data_exist()?;
        let names = BasisSetExchange::read_names()?;
        let basis = names
            .get(&name.to_lowercase())
            .context("The basis set could not be found")?;
        let path = data_path(&format!("{}{}", BASIS_SUBDIRECTORY, basis))?;
        let data = fs::read_to_string(&path).context("Unable to read basis set file")?;
        let result = serde_json::from_str::<InputData>(&data)?;
        Ok(result)
    }
}

