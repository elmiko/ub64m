// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;
use std::str;
use yaml_rust::{Yaml, YamlLoader};

/// Decode a base64 encoded `yaml_rust::Yaml::String` and leave other types
/// untouched. Replaces the value in-place.
pub fn decode_yaml_in_place(yaml: &mut Yaml) {
    match yaml {
        Yaml::Array(vec) => {
            for v in vec {
                decode_yaml_in_place(v);
            }
        }
        Yaml::Hash(hash) => {
            let iter = hash.iter_mut();
            for val in iter {
                decode_yaml_in_place(val.1);
            }
        }
        Yaml::String(src) => {
            if let Ok(bytes) = general_purpose::STANDARD.decode(src.as_str()) {
                if let Ok(decoded) = str::from_utf8(&bytes) {
                    if let Some(stripped) = decoded.strip_suffix('\n') {
                        *yaml = Yaml::String(String::from(stripped));
                    } else {
                        *yaml = Yaml::String(String::from(decoded));
                    }
                }
            }
        }
        _ => (),
    }
}

/// Parse a named file and return a `yaml_rust::Yaml` if possible,
/// or `Error` otherwise.
pub fn manifest_from_filename(filename: String) -> Result<Yaml> {
    let path: &Path = filename.as_ref();
    if !path.is_file() {
        return Err(anyhow!("supplied path must reference a YAML file."));
    }

    let raw = fs::read_to_string(path)?;
    load_yaml_from_string(raw)
}

fn load_yaml_from_string(raw: String) -> Result<Yaml> {
    let docs = YamlLoader::load_from_str(raw.as_str())?;

    match docs.len() {
        0 => Err(anyhow!(
                "no YAML documents found in manifest file, please check source file format."
            )),
        1 => Ok(docs[0].clone()),
        _ => Err(anyhow!(
                "more than one YAML document found in manifest file, only single document manifests are supported."
            )),
    }
}
