use environment::redis_url;
use redis::{cmd as redis_cmd, Client, Connection, ConnectionAddr, ConnectionInfo};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use uuid::Uuid;

use redis_state_manager::event_stream::EventStream;

#[derive(Serialize, Deserialize)]
pub struct WrappedEvent<T: Serialize> {
    pub ev: T,
    response_queue: String,
}

pub struct RedisState {
    pub host_id: String,
    pub redis_connection: Connection,
}

impl RedisState {
    pub fn new(host_id: String) -> Self {
        let redis_connection_info = ConnectionInfo {
            addr: Box::from(ConnectionAddr::Tcp(redis_url().into(), 6379)),
            db: 2,
            passwd: None,
        };
        let redis_client =
            Client::open(redis_connection_info).expect("Can't connect to redis (state)");

        let redis_connection = redis_client
            .get_connection()
            .expect("Can't connect to redis (state)");

        Self {
            host_id,
            redis_connection,
        }
    }

    pub fn emit<T>(&self, ev: T, queue_name: String) -> Result<String, String>
    where
        T: Serialize,
    {
        let response_key = Uuid::new_v4().to_string();

        let transport_payload = WrappedEvent {
            ev,
            response_queue: response_key.clone(),
        };

        match to_string(&transport_payload) {
            Ok(msg) => {
                redis_cmd("LPUSH")
                    .arg(queue_name)
                    .arg(msg)
                    .execute(&self.redis_connection);
                Ok(response_key)
            }
            Err(_) => Err(String::from("Couldn't parse outgoing event")),
        }
    }

    pub fn _get_event_response<T>(
        &self,
        response_queue_id: String,
        timeout: Option<i32>,
    ) -> Result<T, String>
    where
        T: DeserializeOwned + Clone,
    {
        let response: String = redis_cmd("BRPOPLPUSH")
            .arg(response_queue_id)
            .arg("consumed_responses")
            .arg(timeout.unwrap_or(0))
            .query(&self.redis_connection)
            .expect("Can't get event response");
        from_str(response.as_str()).map_err(|_| "Couldn't deserialize incoming response".into())
    }

    pub fn _send_response<T>(&self, response_queue_id: String, data: T) -> ()
    where
        T: Serialize,
    {
        redis_cmd("LPUSH")
            .arg(response_queue_id)
            .arg(to_string::<T>(&data).expect("Can't send response"))
            .execute(&self.redis_connection)
    }

    pub fn get_next_incoming_event<T>(&self, queue_id: String) -> Option<WrappedEvent<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let response: String = redis_cmd("BRPOPLPUSH")
            .arg(queue_id)
            .arg("consumed_events")
            .arg(0)
            .query(&self.redis_connection)
            .expect("Can't get next event");
        println!("Next incoming event: {}", response);
        Some(from_str(response.as_str()).expect("Can't parse next_incoming_event"))
    }

    pub fn get_queue_iter<'a, T>(state: &'a RedisState, queue_id: String) -> EventStream<'a, T>
    where
        T: DeserializeOwned,
    {
        EventStream::new(queue_id, state)
    }

    pub fn save_event<T>(&self, event: T, queues: Vec<String>) -> Result<(), String>
    where
        T: DeserializeOwned + Serialize,
    {
        match to_string(&event) {
            Ok(event_string) => {
                for queue_name in queues {
                    // TODO: Make this an atomic pipeline with MULTI-EXEC
                    redis_cmd("RPUSH")
                        .arg(to_event_queue_name(queue_name))
                        .arg(event_string.clone())
                        .execute(&self.redis_connection);
                }
                Ok(())
            }
            Err(_) => Err("Couldn't parse event".into()),
        }
    }

    pub fn read_events<T>(&self, query: String) -> Result<Vec<T>, String>
    where
        T: DeserializeOwned + Serialize,
    {
        println!("Attempting to read events");
        let ev_list: Result<Vec<String>, _> = redis_cmd("LRANGE")
            .arg(to_event_queue_name(query))
            .arg(0)
            .arg(-1)
            .query(&self.redis_connection);

        match ev_list {
            Ok(events) => Ok(events
                .iter()
                .filter_map(|e| match from_str::<T>(e) {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }).collect()),
            Err(_) => Err("Nope".into()),
        }
    }
}

fn to_event_queue_name(input: String) -> String {
    format!("event_shard:scopify.session.{}:events", input)
}
