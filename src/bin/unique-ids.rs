use std::io::{StdoutLock, Write};

use anyhow::bail;
use dist_sys::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        guid: String,
    },

    Error {
        text: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

struct UniqueNode {
    msg_id: usize,
}

impl Node<Payload> for UniqueNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            // We received an Generate command
            Payload::Generate => {
                let guid = ulid::Ulid::new().to_string();
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.msg_id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk { guid },
                    },
                };

                // &mut * is called a "reborrow"
                // This allows to_writer to take ownership of the mutable borrow that is output.
                // A reborrow is essentially us dereferencing the mutable reference to the StdoutLock
                // and initiating a new mutable reference. Why is this allowed? Why does this allow us to use the mutable reference after the function call?
                //  - I think possibly because output still is never moved.
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;

                self.msg_id += 1;
            }
            Payload::GenerateOk { guid } => todo!(),
            Payload::Error { text } => todo!(),
            Payload::Init { .. } => {
                // When Init is received, we want to reply saying that we have initialised the node
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.msg_id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)?;
                output.write_all(b"\n")?;

                self.msg_id += 1;
            }
            Payload::InitOk => bail!("Received InitOk Message"),
        };

        // Increment id every time or only when we
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(UniqueNode { msg_id: 0 })
}
