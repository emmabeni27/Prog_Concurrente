use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let icing = Arc::new(Semaphore::new(2)); // solo 2 tortas a la vez

    let mut handles = vec![];

    for i in 1..=10 {
        let icing_clone = Arc::clone(&icing);

        let handle = tokio::spawn(async move {
            let permit = icing_clone.acquire().await.unwrap();
            println!("Cake {i} is icing...");
            sleep(Duration::from_secs(2)).await;
            drop(permit);
            println!("Cake {i} is done!");
        });

        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}
