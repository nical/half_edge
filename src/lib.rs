extern crate sid as id;

pub mod id_internals;
pub mod iterators;
pub mod kernel;

pub use kernel::{
    ConnectivityKernel,
    EdgeId, VertexId, FaceId,
    EdgeIdRange, VertexIdRange, FaceIdRange,
    edge_id, vertex_id, face_id
};
pub use iterators::*;

pub use id::{Id, IdVec, IdRange};
