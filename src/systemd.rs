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
use std::process::Command;

/// Stop and delete any services matching the given name.
#[cfg(target_os = "linux")]
pub fn remove_service(name: String) {
    if let Ok(status) = Command::new("systemctl").arg("stop").arg(name).status() {
        if status.success() {
            debug!("Successfully stopped service");
        }
    }
}
