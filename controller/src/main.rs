use fjall::{Keyspace, Partition};

pub struct State {
    keyspace: Keyspace,
    cluster_state: Partition,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
