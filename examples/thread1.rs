use anyhow::Result;
use std::{sync::mpsc, thread, time::Duration};

const PRODUCER_COUNT: usize = 4;

#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..PRODUCER_COUNT {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: id={}, value={}", msg.id, msg.value);
        }
        println!("consumer exit");
        999
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("thread join error: {:?}", e))?;

    println!("secret: {:?}", secret);
    Ok(())
}

fn producer(id: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>() % 10000;
        tx.send(Msg::new(id, value))?;
        if value % 10 == 0 {
            break;
        }
        let sleep_time = rand::random::<u64>() % 1000;
        thread::sleep(Duration::from_millis(sleep_time));
    }

    println!("producer {} exit", id);
    Ok(())
}
