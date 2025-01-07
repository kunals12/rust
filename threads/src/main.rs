use tokio::sync::broadcast;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100); // Task queue with broadcast channel

    // Spawn workers
    for id in 1..=4 {
        let mut worker_rx = tx.subscribe(); // Each worker subscribes to the broadcast
        task::spawn(async move {
            while let Ok(task) = worker_rx.recv().await {
                println!("Worker {} processing task {}", id, task);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Simulate work
            }
            println!("Worker {} done.", id);
        });
    }

    // Send tasks to the workers
    for task in 1..=10 {
        tx.send(task).unwrap(); // Send tasks
    }

    println!("All tasks dispatched!");

    // Give workers time to finish processing
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
