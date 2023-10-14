// Copyright (C) 2023 michael mccune
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use clap::Parser;
use ub64m::manifest_from_filename;
use yaml_rust::YamlEmitter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None )]
struct Cli {
    /// The YAML manifest filename to decode.
    filename: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let manifest = manifest_from_filename(cli.filename)?;

    let mut out_yaml = String::new();
    let mut emitter = YamlEmitter::new(&mut out_yaml);
    emitter.dump(&manifest)?;

    println!("{}", out_yaml);

    Ok(())
}
