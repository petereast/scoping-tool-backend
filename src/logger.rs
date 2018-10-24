// Make a simple redis logger

use environment::redis_url;
use mpsc::{sync_channel, Receiver, SyncSender};
use redis::{cmd, Client};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct RedisPublishLogger {
    sender: SyncSender<String>,
}

impl RedisPublishLogger {
    pub fn new() -> Self {
        let (sender, incoming_logs): (SyncSender<String>, Receiver<String>) = sync_channel(10);

        thread::spawn(move || {
            // Start new redis connection and start listening for channel shit
            println!("[debug] redis_url: {}", redis_url());
            let client =
                Client::open("redis://scopingtoolbackend_redis_1").expect("Can't connect to redis");

            let con = match client.get_connection() {
                Ok(conn) => conn,
                Err(_) => {
                    println!("Couldn't connect to redis, trying again in 5 seconds");

                    sleep(Duration::new(5, 0));
                    println!("Trying again...");
                    client.get_connection().unwrap()
                }
            };

            loop {
                let next_log = incoming_logs.recv().unwrap();
                cmd("PUBLISH")
                    .arg("logs_scopify")
                    .arg(String::from(next_log))
                    .execute(&con);
            }
        });
        Self { sender }
    }

    pub fn get_sender(&self) -> SyncSender<String> {
        self.sender.clone()
    }
}

#[derive(Clone)]
pub struct Logger {
    sender: SyncSender<String>,
    init_time: SystemTime,
}

impl Logger {
    pub fn new() -> Self {
        let logger_backend = RedisPublishLogger::new();

        Self {
            sender: logger_backend.get_sender(),
            init_time: SystemTime::now(),
        }
    }

    pub fn with_backend(be: Arc<RedisPublishLogger>) -> Self {
        Self {
            sender: be.get_sender(),
            init_time: SystemTime::now(),
        }
    }

    pub fn log(&self, msg: String) {
        let formatted_message = format!(
            "[log] ({}) {}",
            self.init_time.elapsed().unwrap().as_secs(),
            msg
        );

        println!("{}", formatted_message);
        self.sender
            .send(formatted_message)
            .expect("Logging error, cannot send to redis")
    }
}
