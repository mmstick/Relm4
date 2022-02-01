// Copyright 2021-2022 Aaron Erhardt <aaron.erhardt@t-online.de>
// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT or Apache-2.0

use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
/// Controls the component from afar.
pub struct Controller<Component, Root, Widgets, Input> {
    /// The models and widgets maintained by the component.
    pub state: Rc<RefCell<Fuselage<Component, Widgets>>>,

    /// The widget that this component manages.
    pub widget: Root,

    /// Used for emitting events to the component.
    pub sender: Sender<Input>,
}

impl<Component, Root, Widgets, Input> Controller<Component, Root, Widgets, Input> {
    /// Emits an input to the component.
    pub fn emit(&self, event: Input) {
        let _ = self.sender.send(event);
    }
}
