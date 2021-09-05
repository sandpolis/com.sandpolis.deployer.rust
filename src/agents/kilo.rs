//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::DistagentConfig;
use crate::BinaryAssets;

use anyhow::{bail, Result};
use log::{debug, error, info};
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::io::Write;
use std::path::Path;

/// Find existing agent installations
pub fn agent_search(config: &DistagentConfig) -> Vec<&Path> {

    debug!("Searching for existing kilo agent installations");

    let mut paths = Vec::new();

    // First check the location specified in the config
    let config_path = Path::new(&config.install_path);
    if config_path.exists() {
        paths.push(config_path);
    }

    return paths
}

/// Install or reinstall a kilo (Java) agent
pub fn install(config: &DistagentConfig) -> Result<()> {

    let mut install_path: Path;

    let existing = agent_search(config);

    if existing.len() == 1 {
        debug!("Found an existing installation at: {}", existing[0].display());
        install_path = existing[0];
    } else if existing.len() > 1 {
        info!("Multiple existing installations found");
    } else {
        debug!("No existing installations found");
        install_path = Path::new(&config.install_path);
    }

    debug!("Starting kilo agent installation");

    // Build HTTP client for downloading modules
    let http = reqwest::blocking::Client::new();

    // Install the required modules
    for module in config.kilo.modules.iter() {

        let mut dest = File::create(install_path.join(format!("{}.jar", module.artifact)))?;

        if let Some(data) = BinaryAssets::get(&module.artifact) {
            File::create(dest)?.write_all(&data)?;
        } else {
            // Download the module instead
            let url = format!("https://api.sandpolis.com/v1/download/{}/{}", module.group, module.artifact);
            copy(&mut http.get(url).send()?, &mut dest)?;
        }
    }

    return Ok(());
}
