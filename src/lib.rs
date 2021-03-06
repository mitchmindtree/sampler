#[cfg(feature="wav")] extern crate hound;
extern crate instrument;
extern crate pitch_calc as pitch;
extern crate sample;
extern crate time_calc as time;

pub use map::{Audio, Map, Sample};
pub use mode::Mode;
pub use sampler::{Frames, Sampler};

pub mod dynamic;
pub mod map;
mod mode;
mod sampler;

#[cfg(feature="serde_serialization")]
mod serde;

pub type Velocity = f32;
