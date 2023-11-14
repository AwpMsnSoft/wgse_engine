use super::kernel;

#[cfg(not(feature = "meta_collect"))]
mod implement {
    use super::kernel::Kernel;
    use anyhow::Result;
    #[cfg(not(feature = "meta_init"))]
    use enum_dispatch::enum_dispatch;
    use wgse_kernel::types::{common::Argument, wrapper::BinVec};
    #[cfg(feature = "meta_init")]
    use wgse_utils::wgse_command_interface;
    #[cfg(not(feature = "meta_init"))]
    use wgse_utils::wgse_command_trait_impl;

    #[cfg(feature = "meta_init")]
    pub trait WgseCommand {
        #[wgse_command_interface]
        fn execute(&self, kernel: &mut Kernel, args: &BinVec<Argument>) -> Result<()>;
    }

    #[cfg(not(feature = "meta_init"))]
    #[enum_dispatch]
    pub trait WgseCommand {
        #[allow(unused_variables)]
        fn execute(&self, kernel: &mut Kernel, args: &BinVec<Argument>) -> Result<()>;
    }

    #[cfg(not(feature = "meta_init"))]
    #[wgse_command_trait_impl(WgseCommand)]
    pub enum WgseCommands {}
}

#[cfg(not(feature = "meta_collect"))]
pub use implement::*;
