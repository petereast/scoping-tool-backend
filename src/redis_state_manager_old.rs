use environment::redis_url;
use redis::{cmd as redis_cmd, Client, Connection, ConnectionAddr, ConnectionInfo, FromRedisValue};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use std::thread;
use uuid::Uuid;

#[derive(Serialize)]
struct OutgoingWrapper<T: Serialize> {
    ev: T,
    // The name of where the response is going to be put
    response_queue: String,
}

// The above struct is to be passed into the actix app-state

pub struct RedisState {
    pub host_id: String,
    pub redis_connection: Connection,
}

impl RedisState {
    pub fn _new(host_id: String) -> Self {
        // Connect to logger
        let redis_connection_info = ConnectionInfo {
            addr: Box::from(ConnectionAddr::Tcp(redis_url().into(), 6379)),
            db: 2,
            passwd: None,
        };
        let redis_client =
            Client::open(redis_connection_info).expect("Can't connect to redis (state)");

        let redis_connection = redis_client.get_connection().unwrap();
        // It's acceptable for this to blow up if the connection fails.

        Self {
            host_id,
            redis_connection,
        }
    }

    pub fn emit(&self, ev: String, queue_name: String) -> Result<String, String> {
        let response_key = Uuid::new_v4().to_string();
        // TODO: Do some enums on the event queue name
        // Return the response id
        let transport_payload = OutgoingWrapper {
            ev,
            response_queue: response_key.clone(),
        };

        let queue_msg = to_string(&transport_payload).expect("thang ain't right");
        redis_cmd("LPUSH")
            .arg(queue_name)
            .arg(queue_msg)
            .execute(&self.redis_connection);

        Ok(response_key)
    }

    pub fn get_event_response<T>(
        &self,
        response_queue_id: String,
        _timeout: i32,
    ) -> Result<T, String>
    where
        T: DeserializeOwned + Clone,
    {
        let response: String = redis_cmd("BRPOPLPUSH")
            .arg(response_queue_id)
            .arg("consumed_responses")
            .query(&self.redis_connection)
            .unwrap();
        from_str(response.as_str()).map_err(|_| "Couldn't deserialize incoming response".into())
    }

    pub fn send_response<T>(
        &self,
        response_queue_id: String,
        data: T
        ) -> () where T: Serialize {
        redis_cmd("LPUSH")
            .arg(response_queue_id)
            .arg(to_string(data))
            .execute(&self.redis_connection);
    }

    fn get_incoming_events<T>(&self, queue_id: String) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let response: String = redis_cmd("BRPOPLPUSH")
            .arg(queue_id)
            .arg("consumed_events")
            .query(&self.redis_connection)
            .unwrap();
        Some(from_str(response.as_str()).unwrap())
    }

    pub fn _wait_for_queue<T>(self, queue_id: String) -> EventStream<T>
    where
        T: DeserializeOwned,
    {
        EventStream::new(queue_id, Box::from(self))
    }
}

// TODO: There should be an enum of usable queues
// TODO: Also there should be a wrapper around the returned event

// Returns the id of the queue the data is going to be stored in
// Serializable s :: s -> String
pub fn _send_response(data: String) -> String {
    let response_id = Uuid::new_v4();

    // Spawn a thread to perform the IO operation asynchronously
    println!("{}", data);

    response_id.clone().to_string()
}
