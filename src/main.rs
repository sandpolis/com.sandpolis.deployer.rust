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
use log::{debug, error, info};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct KiloAgentModule {
    gpr_module: Option<String>,

    gpr_package: Option<String>,

    /// The artifact's group
    maven_group: Option<String>,

    /// The artifact's identifier
    artifact: String,

    /// The artifact's filename
    filename: String,

    /// The artifact's version string
    version: Option<String>,

    /// The artifact's integrity hash
    hash: Option<String>,
}

#[derive(Deserialize)]
pub struct KiloAgentConfig {
    /// A list of all required modules in G:A:V format
    modules: Vec<KiloAgentModule>,
}

#[derive(Deserialize)]
pub struct CallbackConfig {
    /// The callback address
    address: String,

    /// The callback identifier
    identifier: String,
}

#[derive(Serialize)]
pub struct CallbackResult {
    result: bool,
    install_path: String,
}

#[derive(Deserialize)]
pub struct DistagentConfig {
    /// The type of agent to install
    agent_type: String,

    /// The filesystem path where the agent should be installed
    install_path: String,

    /// Whether the installer is allowed to disregard elements of the config in
    /// order to recover from errors.
    autorecover: bool,

    kilo: Option<KiloAgentConfig>,

    callback: Option<CallbackConfig>,
}

pub mod agents {
    pub mod kilo;
    pub mod micro;
    pub mod nano;
}
pub mod callback;
pub mod systemd;

/// Contains embedded resources
#[derive(RustEmbed)]
#[folder = "res"]
pub struct BinaryAssets;

/// Validate the configuration
fn validate_config(config: &DistagentConfig) -> Result<()> {
    // Check agent type
    /*if let Some(agent_type) = config.get(CFG_AGENT_TYPE) {
        if ! vec!["nano", "micro", "kilo"].contains(&agent_type.as_str()) {
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
    }*/

    return Ok(());
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    debug!("Starting automated installation");

    if let Some(config_data) = BinaryAssets::get("distagent.json") {
        let config: DistagentConfig = serde_json::from_slice(&config_data)?;

        // Validate the configuration
        validate_config(&config)?;

        // Dispatch appropriate installer
        match config.agent_type.as_str() {
            "nano" => agents::nano::install(&config),
            "micro" => agents::micro::install(&config),
            "kilo" => agents::kilo::install(&config),
            _ => Ok(()),
        }?;
    }

    return Ok(());
}
