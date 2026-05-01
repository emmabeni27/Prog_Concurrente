use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let icing = Arc::new(Semaphore::new(2)); // solo 2 tortas pueden usar glaseado

    let chocolate_cake = Arc::clone(&icing);
    let vanilla_cake = Arc::clone(&icing);
    let berry_cake = Arc::clone(&icing);

    let worker1 = tokio::spawn(async move {
        let permit = vanilla_cake.acquire().await.unwrap();
        println!("Vanilla cake is icing...");
        sleep(Duration::from_secs(2)).await;
        drop(permit);
        println!("Vanilla cake is done!");
    });

    let worker2 = tokio::spawn(async move {
        let permit = chocolate_cake.acquire().await.unwrap();
        println!("Chocolate cake is icing...");
        sleep(Duration::from_secs(2)).await;
        drop(permit);
        println!("Chocolate cake is done!");
    });

    let worker3 = tokio::spawn(async move {
        let permit = berry_cake.acquire().await.unwrap();
        println!("Berry cake is icing...");
        sleep(Duration::from_secs(2)).await;
        drop(permit);
        println!("Berry cake is done!");
    });

    worker1.await.unwrap();
    worker2.await.unwrap();
    worker3.await.unwrap();
}
