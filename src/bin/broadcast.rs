use std::{
    collections::{HashMap, HashSet},
    io::{StdoutLock, Write},
};

use distributers::{main_loop, Body, Init, Message, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: HashSet<usize>, // HashSet is used because it is guaranteed that all messages are unique
    },
    Topology {
        topology: HashMap<String, HashSet<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    node: String,
    msg_id: usize,
    messages: HashSet<usize>,
}

impl Node<(), Payload> for BroadcastNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        let reply = match input.body.payload {
            Payload::Broadcast { message } => {
                self.messages.insert(message);
                input.into_reply(Some(&mut self.msg_id), Payload::BroadcastOk)
            }
            Payload::Read => input.into_reply(
                Some(&mut self.msg_id),
                Payload::ReadOk {
                    messages: self.messages.clone(),
                },
            ),
            Payload::Topology { ref topology } => {
                input.into_reply(Some(&mut self.msg_id), Payload::TopologyOk)
            }

            Payload::ReadOk { .. } | Payload::BroadcastOk | Payload::TopologyOk => {
                // Return without sending a reply
                return Ok(());
            }
        };
        // &mut * is called a "reborrow"
        // This allows to_writer to take ownership of the mutable borrow that is output.
        // A reborrow is essentially us dereferencing the mutable reference to the StdoutLock
        // and initiating a new mutable reference. Why is this allowed? Why does this allow us to use the mutable reference after the function call?
        //  - I think possibly because output still is never moved.
        serde_json::to_writer(&mut *output, &reply)?;
        output.write_all(b"\n")?;
        // Increment id every time or only when we
        Ok(())
    }

    fn from_init(_state: (), init: Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            node: init.node_id,
            msg_id: 1, // msg_id starts at 1 because init msg handled by main_loop is the 0th msg
            messages: HashSet::new(),
        })
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
