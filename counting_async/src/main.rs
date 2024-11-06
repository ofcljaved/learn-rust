use std::time::Duration;

use trpl;
fn main() {
    trpl::run(async {
         let task = trpl::spawn_task(async {
            for i in 1..10 {
                println!("HI {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("Two {i}");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        task.await.unwrap();
    });
    println!("Hello, world!");
}
