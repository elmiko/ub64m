// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

pub fn manifest_from_filename(filename: String) -> Result<Yaml> {
    let path: &Path = filename.as_ref();
    if !path.is_file() {
        return Err(anyhow!("supplied path must reference a YAML file."));
    }

    let raw = fs::read_to_string(path)?;
    return load_yaml_from_string(raw);
}

fn load_yaml_from_string(raw: String) -> Result<Yaml> {
    let docs = YamlLoader::load_from_str(raw.as_str())?;

    match docs.len() {
        0 => {
            return Err(anyhow!(
                "no YAML documents found in manifest file, please check source file format."
            ))
        }
        1 => return Ok(docs[0].clone()),
        _ => {
            return Err(anyhow!(
                "more than one YAML document found in manifest file, only single document manifests are supported."
            ))
        }
    }
}
