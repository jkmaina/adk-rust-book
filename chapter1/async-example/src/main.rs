use tokio::time::{sleep, Duration};

async fn do_something() {
    println!("Doing something async");
    // simulate async work
    sleep(Duration::from_millis(100)).await;
    println!("Finished async work");
}

#[tokio::main]
async fn main() {
    // Calling an async function returns a Future, it doesn't run yet.
    let future = do_something();

    // .await suspends the current function until the future completes.
    future.await;

    // You can also `.await` directly:
    // do_something().await;
}
