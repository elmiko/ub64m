// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use clap::Parser;
use std::io::stdin;
use std::io::Read;
use std::str;
use ub64m::{decode_yaml_in_place, manifest_from_filename, manifest_from_string};
use yaml_rust::YamlEmitter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None )]
struct Cli {
    /// The YAML manifest filename to decode.
    filename: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut manifest = match cli.filename.as_str() {
        "-" => {
            let mut buffer = Vec::new();
            stdin().read_to_end(&mut buffer)?;
            let raw = str::from_utf8(&buffer)?;
            manifest_from_string(String::from(raw))?
        }
        _ => manifest_from_filename(cli.filename)?,
    };

    decode_yaml_in_place(&mut manifest);

    let mut out_yaml = String::new();
    let mut emitter = YamlEmitter::new(&mut out_yaml);
    emitter.dump(&manifest)?;

    println!("{}", out_yaml);

    Ok(())
}
