use std::error::Error;
use tokio::{main, sync::mpsc};

#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
}

#[main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel(1);
    let orders = vec![
        Message {
            order: Order::BUY,
            ticker: String::from("BYND"),
            amount: 5.5,
        },
        Message {
            order: Order::BUY,
            ticker: String::from("NET"),
            amount: 5.5,
        },
        Message {
            order: Order::BUY,
            ticker: String::from("PLTR"),
            amount: 5.5,
        },
    ];

    tokio::spawn(async move {
        for order in orders {
            if let Err(e) = tx.send(order.clone()).await {
                println!("send error: {:?}", e);
            }
            println!("sent: {:?}", order);
        }
    });

    while let Some(i) = rx.recv().await {
        println!("GOT = {:?}", i);
    }

    Ok(())
}
