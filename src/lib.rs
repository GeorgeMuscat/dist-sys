use std::io::{StdoutLock, Write};

use anyhow::{bail, Context};
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

pub trait Node<P> {
    fn step(&mut self, input: Message<P>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, P>(mut state: S) -> anyhow::Result<()>
where
    S: Node<P>,
    P: DeserializeOwned,
{
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<P>>();

    let mut stdout = std::io::stdout().lock();

    for input in inputs {
        let input = input.context("Maelstrom input from STDIN could not be deserialized")?;
        state.step(input, &mut stdout)?;
    }

    Ok(())
}
