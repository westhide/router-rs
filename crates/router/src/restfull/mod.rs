pub use axum::routing::MethodFilter as M;

// Get
pub const GET: M = M::GET;
// Head
pub const HEAD: M = M::HEAD;
// Post
pub const POST: M = M::POST;
// Put
pub const PUT: M = M::PUT;
// Delete
pub const DELETE: M = M::DELETE;
// Connect
pub const CONNECT: M = M::CONNECT;
// Options
pub const OPTIONS: M = M::OPTIONS;
// Trace
pub const TRACE: M = M::TRACE;
// Patch
pub const PATCH: M = M::PATCH;
