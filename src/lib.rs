use std::io::{BufRead, StdoutLock, Write};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<P> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<P>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<P> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: P,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InitPayload {
    Init(Init),
    InitOk,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<S, P> {
    fn from_init(state: S, init: Init) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn step(&mut self, input: Message<P>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, N, P>(init_state: S) -> anyhow::Result<()>
where
    N: Node<S, P>,
    P: DeserializeOwned,
{
    let mut stdin = std::io::stdin().lock().lines();

    let mut stdout = std::io::stdout().lock();

    // First message should be the init message
    let init_msg: Message<InitPayload> = serde_json::from_str(
        &stdin
            .next()
            .expect("No init message received")
            .context("failed to read init msg from stdin")?,
    )
    .context("failed to deserialize init message")?;

    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("First message should be the init message")
    };
    let mut node: N = Node::from_init(init_state, init)?;

    // Need to create the first reply for the node
    let reply = Message {
        src: init_msg.dst,
        dst: init_msg.src,
        body: Body {
            id: Some(0), // InitOk is always the 0th msg for a node.
            in_reply_to: init_msg.body.id,
            payload: InitPayload::InitOk,
        },
    };

    // Manually write the InitOk to stdout
    serde_json::to_writer(&mut stdout, &reply).context("Failed to srerialize the init msg")?;
    stdout
        .write_all(b"\n")
        .context("Failed to write the trailing newline")?;
    for line in stdin {
        let line = line.context("Maelstrom input from STDIN could not be read")?;
        let input: Message<P> = serde_json::from_str(&line)
            .context("Maelstrom input from STDIN could not be deserialized")?;
        node.step(input, &mut stdout)?;
    }

    Ok(())
}
