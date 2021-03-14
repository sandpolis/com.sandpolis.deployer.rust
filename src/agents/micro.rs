//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use log::{debug, error, info};
use std::collections::HashMap;

/// Install or reinstall a micro (Rust) agent.
pub fn install(config: &HashMap<String, String>) {
    if config.contains_key("micro-agent") {
        // Use embedded module
        let mut file = File::create("name.test");

        file.write_all(config["micro-agent"]?);
    } else {
        // Download the module
        download_artifact("https://repo1.maven.org/maven2/%s/%s/%s/%s");
    }
}
