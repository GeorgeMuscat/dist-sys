use std::io::{StdoutLock, Write};

use distributers::{main_loop, Init, Message, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    // Received Echo command
    Echo { echo: String },

    // Reply to an Echo command
    EchoOk { echo: String },
    Error { text: String },
}

struct EchoNode {
    msg_id: usize,
}

impl Node<(), Payload> for EchoNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            // We received an Echo command
            Payload::Echo { ref echo } => {
                let echo = echo.to_string();
                let reply = input.into_reply(Some(&mut self.msg_id), Payload::EchoOk { echo });

                // &mut * is called a "reborrow"
                // This allows to_writer to take ownership of the mutable borrow that is output.
                // A reborrow is essentially us dereferencing the mutable reference to the StdoutLock
                // and initiating a new mutable reference. Why is this allowed? Why does this allow us to use the mutable reference after the function call?
                //  - I think possibly because output still is never moved.
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;
                self.msg_id += 1
            }
            Payload::EchoOk { .. } => {}
            Payload::Error { .. } => {}
        };

        Ok(())
    }

    fn from_init(_state: (), _init: Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(EchoNode { msg_id: 1 }) // start at 1 because init response is the 0th msg
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, EchoNode, _>(())
}
