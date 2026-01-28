use tokio::sync::mpsc;

static NUM_PRODUCER: usize = 4;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);

    for i in 0..NUM_PRODUCER {
        let tx_clone = tx.clone();
        tokio::spawn(async move { producer(i, tx_clone).await });
    }

    while let Some(value) = rx.recv().await {
        println!("Consumer consuming {}", value);
    }
}

async fn producer(id: usize, tx: mpsc::Sender<i32>) {
    loop {
        match tx.try_send(id as i32) {
            Ok(_) => {
                println!("Producer {} produced", id);
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                // wait until the channel is ready to accept more messages
                println!("Producer {} waiting, channel full", id);
                tokio::task::yield_now().await;
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                break;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
