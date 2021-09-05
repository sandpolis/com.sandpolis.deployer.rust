//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::{CallbackConfig, CallbackResult};

use std::net::TcpStream;
use std::io::Write;

use anyhow::{bail, Result};
use log::{debug, error, info};
use serde_json;

/// Perform a result callback
pub fn callback(config: &CallbackConfig, result: &CallbackResult) -> Result<()> {
    let data = serde_json::to_string(result)?;

    // Fire off the callback
    TcpStream::connect(&config.address)?.write(data.as_str().as_bytes())?;

    return Ok(());
}
