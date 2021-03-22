//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use anyhow::{bail, Result};
use dotproperties::parse_from_slice;
use log::{debug, error, info};
use rust_embed::RustEmbed;
use std::collections::HashMap;

/// The type of agent to install
pub static CFG_AGENT_TYPE: &'static str = "agent.type";

/// The filesystem path where the agent should be installed
pub static CFG_AGENT_PATH: &'static str = "agent.path";

/// Whether the installer is allowed to disregard elements of the config in
/// order to recover from errors.
pub static CFG_INST_RECOVER: &'static str = "inst.recover";

pub mod agents {
    pub mod micro;
    pub mod nano;
    pub mod vanilla;
}
pub mod http;
pub mod systemd;

/// Contains embedded resources
#[derive(RustEmbed)]
#[folder = "res"]
pub struct BinaryAssets;

/// Validate the configuration
fn validate_config(config: &HashMap<String, String>) -> Result<()> {

    // Check agent type
    if let Some(agent_type) = config.get(CFG_AGENT_TYPE) {
        if ! vec!["nano", "micro", "vanilla"].contains(&agent_type.as_str()) {
            bail!("Invalid agent type");
        }
    } else {
        bail!("Missing agent type");
    }

    // Check install path
    if let Some(agent_path) = config.get(CFG_AGENT_PATH) {
        // TODO
    } else {
        bail!("Missing agent path");
    }

    return Ok(());
}

fn main() -> Result<()> {

    // Initialize logging
    env_logger::init();

    debug!("Starting automated installation");

    if let Some(config_properties) = BinaryAssets::get("config.properties") {
        let config = parse_from_slice(&config_properties)
            .expect("Failed to parse properties file").into_iter().collect();

        // Validate the configuration
        validate_config(&config)?;

        // Dispatch appropriate installer
        match config[CFG_AGENT_TYPE].as_str() {
            "nano" => agents::nano::install(&config),
            "micro" => agents::micro::install(&config),
            "vanilla" => agents::vanilla::install(&config),
            _ => Ok(())
        }?;
    }

    return Ok(());
}
