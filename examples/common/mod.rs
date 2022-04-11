#[cfg(feature = "dim3")]
mod environment3d;
#[cfg(feature = "dim3")]
pub use self::environment3d::setup_environment;

#[cfg(feature = "dim2")]
mod environment2d;
#[cfg(feature = "dim2")]
pub use self::environment2d::setup_environment;

#[cfg(feature = "dim3")]
pub const BOWL_SIZE: [f32 ; 3] = [10.0, 3.0, 10.0];