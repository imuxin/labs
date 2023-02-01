use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Msg {
    V4(i32),
    V6(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, mut receiver) = mpsc::channel::<Msg>(32);
    let sender2 = sender.clone();
    tokio::spawn(async move {
        std::thread::sleep(std::time::Duration::from_secs(2));
        sender.send(Msg::V4(10)).await.unwrap();
    });

    tokio::spawn(async move {
        std::thread::sleep(std::time::Duration::from_secs(4));
        sender2.send(Msg::V6("String".to_string())).await.unwrap();
    });

    loop {
        if let Some(_msg) = receiver.recv().await {
            match _msg {
                Msg::V4(body) => {
                    println!("msg: {}", body)
                }
                Msg::V6(body) => {
                    println!("msg: {}", body)
                }
            }
        }
    }
}
