use std::time::Duration;

fn main() {
    trpl::run(async {
       let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let tx1_fut = async move {
            let vals = vec!["hi","hello","you", "who"];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(250)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("YOu send me this: {val}");
            }  
        };

        let tx_fut = async move {
            let vals = vec!["Hi","Hello","You", "Who"];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(250)).await;
            }
        };

        trpl::join!(tx1_fut, rx_fut, tx_fut)
    });
    println!("Hello, world!");
}
