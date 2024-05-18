pub mod adapter;
pub mod command_queue;
pub mod device;
pub mod error;
pub mod factory;
pub mod misc;
pub mod swapchain;
pub mod sync;

mod conv;
mod utils;

pub(crate) trait HasInterface {
    type Raw;
    fn as_raw(&self) -> &Self::Raw;
}
