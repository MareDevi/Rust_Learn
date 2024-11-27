use anyhow::{anyhow, Result};
use std::{thread, sync::mpsc};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
    });

    consumer.join().map_err (| e | anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(idx:usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}