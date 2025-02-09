/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::actors::Destination;
use crate::actors::popover;
use crate::logging;
use std::thread;
use zbus::blocking::Connection;

use super::Void;


#[zbus::proxy(
    interface = "org.freedesktop.ScreenSaver",
    default_service = "org.freedesktop.ScreenSaver",
    default_path = "/org/freedesktop/ScreenSaver"
)]
pub trait Manager {
    #[zbus(signal)]
    fn active_changed(&self, active: bool) -> fdo::Result<()>;
}

/// Listens to screensaver (screen lock) changes
pub fn init(destination: popover::Destination) {
    thread::spawn(move || {
        if let Err(e) = start(destination) {
            log_print!(
                logging::Level::Surprise,
                "Could not track screensaver status, giving up: {:?}",
                e,
            );
        }
    });
}

fn start(destination: popover::Destination) -> Result<Void, zbus::Error> {
    let conn = Connection::session()?;
    let manager = ManagerProxyBlocking::new(&conn)?;

    let mut active_changed = manager.receive_active_changed()?;

    for m in active_changed {
        match m.args() {
            Ok(args) => destination.send(popover::Event::ScreensaverActive(args.active)),
            Err(err) => log_print!(
                logging::Level::Bug,
                "Encountered unhandled event: {:?}",
                err,
            ),
        }
    }

    unreachable!()
}
