//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::BinaryAssets;
use crate::DistagentConfig;

use std::env;
use std::fs;
use std::io::copy;
use std::io::Write;
use std::path::Path;

use anyhow::{bail, Result};
use log::{debug, error, info};

/// Find existing agent installations
pub fn agent_search(config: &DistagentConfig) -> Vec<&Path> {
    debug!("Searching for existing kilo agent installations");

    let mut paths = Vec::new();

    // First check the location specified in the config
    let config_path = Path::new(&config.install_path);
    if config_path.exists() {
        paths.push(config_path);
    }

    // Next check the PATH
    match env::var("PATH") {
        Ok(path) => {
            for p in path
                .split(":")
                .map(|_p| Path::new(_p).join("sandpolis-agent"))
            {
                if p.exists() {
                    fs::read_link(p);
                }
            }
        }
        Err(e) => panic!(),
    }

    return paths;
}

/// Install or reinstall a kilo (Java) agent
pub fn install(config: &DistagentConfig) -> Result<()> {
    let existing = agent_search(config);

    let install_path: &Path = if existing.len() == 1 {
        debug!(
            "Found an existing installation at: {}",
            existing[0].display()
        );
        existing[0]
    } else if existing.len() > 1 {
        info!("Multiple existing installations found");
        // TODO
        Path::new(&config.install_path)
    } else {
        debug!("No existing installations found");
        Path::new(&config.install_path)
    };

    debug!("Starting kilo agent installation");

    // Create base directory
    fs::create_dir_all(install_path)?;

    // Build HTTP client for downloading modules
    let http = reqwest::blocking::Client::new();

    // Install the required modules
    for module in config.kilo.as_ref().expect("").modules.iter() {
        let mut dest = fs::File::create(install_path.join(format!("{}.jar", module.artifact)))?;

        if let Some(data) = BinaryAssets::get(&module.artifact) {
            dest.write_all(&data)?;
        } else {
            // Download the module instead
            let url = if module.gpr_module.is_some() {
                format!(
                    "https://api.sandpolis.com/v1/download/{}/{}/{}/{}",
                    module.gpr_module.as_ref().unwrap(),
                    module.gpr_package.as_ref().expect(""),
                    module.version.as_ref().expect(""),
                    module.filename
                )
            } else {
                format!(
                    "https://repo1.maven.org/maven2/{}/{}/{}/{}",
                    module.maven_group.as_ref().expect(""),
                    module.artifact,
                    module.version.as_ref().expect(""),
                    module.filename
                )
            };
            copy(&mut http.get(url).send()?, &mut dest)?;
        }
    }

    return Ok(());
}
