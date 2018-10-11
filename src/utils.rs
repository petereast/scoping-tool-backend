use futures::future::Future;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub type BoxedFuture<'a, T, E> = Box<Future<Item = T, Error = E> + Send + 'a>;

pub fn generate_id() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(5)
        .collect()
}
