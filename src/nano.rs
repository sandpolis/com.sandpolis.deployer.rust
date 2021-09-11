//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::config::DistagentConfig;
use anyhow::{bail, Result};
use log::{debug, error, info};

/// Install or reinstall a nano (C++) agent
pub fn install(config: &DistagentConfig) -> Result<()> {
    debug!("Starting nano agent installation");
    return Ok(());
}
