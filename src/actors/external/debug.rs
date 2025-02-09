/*
 * Copyright (C) 2022 Purism SPC
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::main;
use crate::state;

use std::thread;
use zbus::{blocking::Connection, ObjectServer};

use super::Void;

use std::convert::TryInto;


/// Accepts commands controlling the debug mode
struct Manager {
    sender: main::EventLoop,
    enabled: bool,
}

#[zbus::interface(name = "sm.puri.SqueekDebug")]
impl Manager {
    #[zbus(property, name = "Enabled")]
    fn get_enabled(&self) -> bool {
        self.enabled
    }
    #[zbus(property, name = "Enabled")]
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.sender
            .send(state::Event::Debug(
                if enabled { Event::Enable }
                else { Event::Disable }
            ))
            .unwrap();
    }
}

fn start(mgr: Manager) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::session()?;

    connection.object_server().at("/sm/puri/SqueekDebug", mgr)?;
    connection.request_name("sm.puri.SqueekDebug")?;

    loop {
        thread::park();
    }
}

pub fn init(sender: main::EventLoop) {
    let mgr = Manager {
        sender,
        enabled: false,
    };
    thread::spawn(move || {
        start(mgr).unwrap();
    });
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Enable,
    Disable,
}
