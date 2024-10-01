pub use prost;
pub use tonic;

pub mod cri {
    tonic::include_proto!("runtime.v1");
}