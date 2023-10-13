// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::path::Path;
use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Parser)]
#[clap(author, version, about, long_about = None )]
struct Cli {
    /// The YAML manifest filename to decode
    filename: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let manifest_path: &Path = cli.filename.as_ref();
    if !manifest_path.is_file() {
        return Err(anyhow!("Path is not a file {}", manifest_path.display()));
    }

    let raw = fs::read_to_string(manifest_path)?;

    let docs = YamlLoader::load_from_str(raw.as_str())?;

    println!("{}", docs.len());

    match docs.len() {
        0 => {
            return Err(anyhow!(
                "No YAML documents found in {}",
                manifest_path.display()
            ))
        }
        1 => (),
        _ => {
            return Err(anyhow!(
                "More than one YAML documents found in {}",
                manifest_path.display()
            ))
        }
    }

    Ok(())
}
