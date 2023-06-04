use std::error::Error;
use tokio::main;
use tokio::sync::{mpsc, mpsc::Sender, oneshot};

#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<u32>,
}

pub struct OrderBookActor {
    pub receiver: mpsc::Receiver<Message>,
    pub total_inversted: f32,
    pub investment_cap: f32,
}

impl OrderBookActor {
    fn new(receiver: mpsc::Receiver<Message>, investment_cap: f32) -> Self {
        OrderBookActor {
            receiver,
            total_inversted: 0.0,
            investment_cap,
        }
    }

    fn handle_message(&mut self, message: Message) {
        if message.amount + self.total_inversted >= self.investment_cap {
            println!(
                "rejecting purchase, total invested: {}",
                self.total_inversted
            );
            let _ = message.respond_to.send(0);
        } else {
            self.total_inversted += message.amount;
            println!(
                "processing purchase, total invested: {}",
                self.total_inversted
            );
            let _ = message.respond_to.send(1);
        }
    }

    async fn run(mut self) {
        println!("actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

struct BuyOrder {
    pub ticker: String,
    pub amount: f32,
    pub order: Order,
    pub sender: Sender<Message>,
}

impl BuyOrder {
    fn new(amount: f32, ticker: String, sender: Sender<Message>) -> Self {
        BuyOrder {
            ticker,
            amount,
            order: Order::BUY,
            sender,
        }
    }

    async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message {
            order: self.order,
            amount: self.amount,
            ticker: self.ticker,
            respond_to: send,
        };

        let _ = self.sender.send(message).await;

        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome: {}", outcome),
        }
    }
}

#[main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<Message>(1);
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = BuyOrder::new(5.5, String::from("BYND"), tx1.clone());
            buy_actor.send().await;
        }
        drop(tx1);
    });

    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = BuyOrder::new(5.5, String::from("YYDS"), tx2.clone());
            buy_actor.send().await;
        }
        drop(tx2);
    });

    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = BuyOrder::new(5.5, String::from("OJBK"), tx3.clone());
            buy_actor.send().await;
        }
        drop(tx3);
    });

    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = BuyOrder::new(5.5, String::from("PLTR"), tx.clone());
            buy_actor.send().await;
        }
        drop(tx);
    });

    let actor = OrderBookActor::new(rx, 20.0);
    actor.run().await;

    Ok(())
}
