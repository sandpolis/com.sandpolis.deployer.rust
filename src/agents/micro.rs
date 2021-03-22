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
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;

/// Install or reinstall a micro (Rust) agent.
pub fn install(config: &HashMap<String, String>) -> Result<()> {

    debug!("Starting micro agent installation");

    let path = config.get(crate::CFG_AGENT_PATH).expect("Missing agent path");

    // Create the agent directory
    create_dir_all(path)?;

    if let Some(executable) = crate::BinaryAssets::get("agent-micro") {
        let exe_path = path.to_string() + "/agent-micro";

        File::create(exe_path)?.write_all(&executable)?;
    } else {
        // Download the module
        //download_artifact("https://repo1.maven.org/maven2/%s/%s/%s/%s");
    }

    return Ok(());
}
