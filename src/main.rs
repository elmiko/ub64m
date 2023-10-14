// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::path::Path;
use ub64m::load_yaml_from_string;
use yaml_rust::YamlEmitter;

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
        return Err(anyhow!(
            "Error: supplied path is not a file {}",
            manifest_path.display()
        ));
    }

    let raw = fs::read_to_string(manifest_path)?;

    let manifest = load_yaml_from_string(raw)?;

    let mut out_yaml = String::new();
    let mut emitter = YamlEmitter::new(&mut out_yaml);
    emitter.dump(&manifest)?;

    println!("{}", out_yaml);

    Ok(())
}
