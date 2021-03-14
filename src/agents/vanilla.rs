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

///
const CFG_AGENT_MODULES: &str = "agent.modules";

/// Install or reinstall a vanilla (Java) agent.
pub fn install(config: &HashMap<String, String>) {
    if config.contains_key() {
        // Use embedded module
    } else {
        // Download the module
    }
}
