// This is a crate root for the libraytracer
// We must collect here the other contained modules. We could declare them here
// with mod { etc }, but we can also just say mod etc; and have it in another file

// Rust will look for a vec3.rs file, or a vec3/mod.rs file

pub mod camera;
pub mod hitable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;
