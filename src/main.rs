//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use dotproperties::parse_from_slice;
use log::{debug, error, info};
use rust_embed::RustEmbed;
use std::collections::HashMap;

/// The type of agent to install
const CFG_AGENT_TYPE: &str = "agent.type";

/// The filesystem path where the agent should be installed
const CFG_AGENT_PATH: &str = "agent.path";

/// Whether the installer is allowed to disregard elements of the config in
/// order to recover from errors.
const CFG_INST_RECOVER: &str = "inst.recover";

pub mod agents {
    pub mod micro;
    pub mod nano;
    pub mod vanilla;
}

#[derive(Debug, Clone)]
enum ConfigValidationError {
    MissingAgentType,
}

#[derive(Debug, Clone)]
enum MainError {}

/// Contains embedded resources
#[derive(RustEmbed)]
#[folder = "res"]
struct BinaryAssets;

fn validate_config(config: &HashMap<String, String>) -> Result<(), ConfigValidationError> {
    // Check agent type
    if let Some(agent_type) = config.get(CFG_AGENT_TYPE) {
        // TODO
    } else {
        return Err(ConfigValidationError::MissingAgentType);
    }

    return Ok(());
}

fn main() -> Result<(), ConfigValidationError> {
    // Initialize logging
    env_logger::init();

    debug!("Starting installation");

    if let Some(config_properties) = BinaryAssets::get("config.properties") {
        let config = parse_from_slice(&config_properties)?.into_iter().collect();

        // Validate the configuration
        validate_config(&config)?;

        // Dispatch appropriate installer
        match config[CFG_AGENT_TYPE] {
            "nano" => agents::nano::install(&config),
            "micro" => agents::micro::install(&config),
            "vanilla" => agents::vanilla::install(&config),
            _ => {}
        }
    }

    return Ok(());
}
