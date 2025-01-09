#[cfg(webgpu)]
pub mod webgpu;
#[cfg(webgpu)]
pub(crate) use webgpu::{get_browser_gpu_property, ContextWebGpu};

#[cfg(wgpu_core)]
pub mod wgpu_core;

#[cfg(wgpu_core)]
pub(crate) use wgpu_core::ContextWgpuCore;

//#[cfg(custom)]
pub mod custom;
//#[cfg(custom)]
pub(crate) use custom::DynContext;

pub trait Dispatch {
    type Target: ?Sized;

    fn dispatch(&self) -> &Self::Target;
}

pub trait DispatchMut: Dispatch {
    fn dispatch_mut(&mut self) -> &mut Self::Target;
}
