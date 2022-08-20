/*
*    markov-algorithms — Rust implementation of Markov Algorithms.
*
*    Copyright (C) 2022 by Sergey Ivanov <quixoticaxisgit@gmail.com, quixoticaxisgit@mail.ru>
*
*    This program is free software: you can redistribute it and/or modify
*    it under the terms of the GNU General Public License as published by
*    the Free Software Foundation, either version 3 of the License, or
*    (at your option) any later version.
*
*    This program is distributed in the hope that it will be useful,
*    but WITHOUT ANY WARRANTY; without even the implied warranty of
*    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*    GNU General Public License for more details.
*
*    You should have received a copy of the GNU General Public License
*    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{
    io::stdin,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use anyhow::{Context, Ok, Result};

/// A quick-and-dirty helper to handle user input.
/// It reads lines from console in a separate thread and also handles Ctrl-C.
pub struct UserInputHandler {
    receiver: Receiver<bool>,
    sender: Sender<bool>,
    should_continue: bool,
    thread_handle: Option<JoinHandle<()>>,
}

impl UserInputHandler {
    /// Initializes communication infrastructure and sets the Ctrl-C handler.
    pub fn setup() -> Result<Self> {
        let (signal_sender, signal_receiver) = channel();

        let handler_sender = signal_sender.clone();
        ctrlc::set_handler(move || {
            handler_sender
                .send(false)
                .expect("Could not send signal on the channel.")
        })
        .with_context(|| "Failed to setup the ctrl-C handler")?;

        Ok(Self {
            receiver: signal_receiver,
            sender: signal_sender,
            should_continue: true,
            thread_handle: None,
        })
    }

    /// Spawns a thread for [stdin](std::io::stdin), waits until either the line is read or Ctrl-C signal is handled.
    pub fn should_continue(&mut self) -> Result<bool> {
        self.wait_for_previous_thread();

        println!("Press ENTER to continue or hit Ctrl-C to exit.");

        if self.should_continue {
            let stdin_sender = self.sender.clone();

            self.thread_handle = Some(thread::spawn(move || {
                stdin()
                    .read_line(&mut String::new())
                    .expect("Could not read user's input.");
                stdin_sender
                    .send(true)
                    .expect("Could not send signal on the channel.")
            }));

            self.should_continue = self.receiver.recv()?;
        }

        Ok(self.should_continue)
    }

    fn wait_for_previous_thread(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle
                .join()
                .expect("The thread either finishes its work, or the application closes.");
        }
    }
}
