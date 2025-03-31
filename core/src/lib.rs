pub mod catalogue;
mod management;
pub mod shared;

pub use management::application::ports::*;
pub use management::application::views::*;
pub use management::application::ManagementService;
