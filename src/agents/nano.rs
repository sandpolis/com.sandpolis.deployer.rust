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

/// Install or reinstall a nano (C++) agent.
pub fn install(config: &HashMap<String, String>) -> Result<()> {

    debug!("Starting nano agent installation");
    return Ok(());
}
