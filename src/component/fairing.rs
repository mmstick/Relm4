// Copyright 2021-2022 Aaron Erhardt <aaron.erhardt@t-online.de>
// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT or Apache-2.0

use super::{Controller, Fuselage};
use crate::{Receiver, Sender};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
/// Contains the post-launch input sender and output receivers with the root widget.
///
/// The receiver can be separated from the `Fairing` by choosing a method for handling it.
pub struct Fairing<Component, Root, Widgets, Input, Output> {
    /// The models and widgets maintained by the component.
    pub state: Rc<RefCell<Fuselage<Component, Widgets>>>,

    /// The widget that this component manages.
    pub widget: Root,

    /// Used for emitting events to the component.
    pub sender: Sender<Input>,

    /// The outputs being received by the component.
    pub receiver: Receiver<Output>,
}

impl<Component, Root, Widgets, Input: 'static, Output: 'static>
    Fairing<Component, Root, Widgets, Input, Output>
{
    /// Forwards output events to the designated sender.
    pub fn forward<X: 'static, F: (Fn(Output) -> X) + 'static>(
        self,
        sender_: Sender<X>,
        transform: F,
    ) -> Controller<Component, Root, Widgets, Input> {
        let Fairing {
            state,
            widget,
            sender,
            receiver,
        } = self;

        crate::spawn_local(crate::forward(receiver, sender_, transform));

        Controller {
            state,
            widget,
            sender,
        }
    }

    /// Given a mutable closure, captures the receiver for handling.
    pub fn connect_receiver<F: FnMut(&mut Sender<Input>, Output) + 'static>(
        self,
        mut func: F,
    ) -> Controller<Component, Root, Widgets, Input> {
        let Fairing {
            state,
            widget,
            sender,
            mut receiver,
        } = self;

        let mut sender_ = sender.clone();
        crate::spawn_local(async move {
            while let Some(event) = receiver.recv().await {
                func(&mut sender_, event);
            }
        });

        Controller {
            state,
            widget,
            sender,
        }
    }

    /// Ignore outputs from the component and take the handle.
    pub fn detach(self) -> Controller<Component, Root, Widgets, Input> {
        let Self {
            state,
            widget,
            sender,
            ..
        } = self;

        Controller {
            state,
            widget,
            sender,
        }
    }
}
