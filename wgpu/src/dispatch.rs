//! Infrastructure for dispatching calls to the appropriate "backend". The "backends" are:
//!
//! - `wgpu_core`: An implementation of the the wgpu api on top of various native graphics APIs.
//! - `webgpu`: An implementation of the wgpu api which calls WebGPU directly.
//!
//! The interface traits are all object safe and listed in the `InterfaceTypes` trait.
//!
//! The method for dispatching should optimize well if only one backend is compiled in,
//! as-if there was no dispatching at all.

#![allow(drop_bounds)] // This exists to remind implementors to impl drop.
#![allow(clippy::too_many_arguments)] // It's fine.
#![allow(missing_docs, clippy::missing_safety_doc)] // Interfaces are not documented

use crate::{WasmNotSend, WasmNotSendSync};

use std::{any::Any, fmt::Debug, future::Future, hash::Hash, ops::Range, pin::Pin, sync::Arc};

use crate::backend;

/// Create a single trait with the given supertraits and a blanket impl for all types that implement them.
///
/// This is useful for creating a trait alias as a shorthand.
macro_rules! trait_alias {
    ($name:ident: $($bound:tt)+) => {
        pub trait $name: $($bound)+ {}
        impl<T: $($bound)+> $name for T {}
    };
}

// Various return futures in the API.
trait_alias!(RequestAdapterFuture: Future<Output = Option<DispatchAdapter>> + WasmNotSend + 'static);
trait_alias!(RequestDeviceFuture: Future<Output = Result<(DispatchDevice, DispatchQueue), crate::RequestDeviceError>> + WasmNotSend + 'static);
trait_alias!(PopErrorScopeFuture: Future<Output = Option<crate::Error>> + WasmNotSend + 'static);
trait_alias!(ShaderCompilationInfoFuture: Future<Output = crate::CompilationInfo> + WasmNotSend + 'static);

// We can't use trait aliases here, as you can't convert from a dyn Trait to dyn Supertrait _yet_.
#[cfg(send_sync)]
pub type BoxDeviceLostCallback = Box<dyn FnOnce(crate::DeviceLostReason, String) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BoxDeviceLostCallback = Box<dyn FnOnce(crate::DeviceLostReason, String) + 'static>;
#[cfg(send_sync)]
pub type BoxSubmittedWorkDoneCallback = Box<dyn FnOnce() + Send + 'static>;
#[cfg(not(send_sync))]
pub type BoxSubmittedWorkDoneCallback = Box<dyn FnOnce() + 'static>;
#[cfg(send_sync)]
pub type BufferMapCallback = Box<dyn FnOnce(Result<(), crate::BufferAsyncError>) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BufferMapCallback = Box<dyn FnOnce(Result<(), crate::BufferAsyncError>) + 'static>;

// Common traits on all the interface traits
trait_alias!(CommonTraits: Any + Debug + WasmNotSendSync);

pub trait InstanceInterface: CommonTraits {
    fn new(desc: &crate::InstanceDescriptor) -> Self
    where
        Self: Sized;

    unsafe fn create_surface(
        &self,
        target: crate::SurfaceTargetUnsafe,
    ) -> Result<DispatchSurface, crate::CreateSurfaceError>;

    fn request_adapter(
        &self,
        options: &crate::RequestAdapterOptions<'_, '_>,
    ) -> Pin<Box<dyn RequestAdapterFuture>>;

    fn poll_all_devices(&self, force_wait: bool) -> bool;
}

pub trait AdapterInterface: CommonTraits {
    fn request_device(
        &self,
        desc: &crate::DeviceDescriptor<'_>,
        trace_dir: Option<&std::path::Path>,
    ) -> Pin<Box<dyn RequestDeviceFuture>>;

    fn is_surface_supported(&self, surface: &DispatchSurface) -> bool;

    fn features(&self) -> crate::Features;

    fn limits(&self) -> crate::Limits;

    fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities;

    fn get_info(&self) -> crate::AdapterInfo;

    fn get_texture_format_features(
        &self,
        format: crate::TextureFormat,
    ) -> crate::TextureFormatFeatures;

    fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp;
}

pub trait DeviceInterface: CommonTraits {
    fn features(&self) -> crate::Features;
    fn limits(&self) -> crate::Limits;

    fn create_shader_module(
        &self,
        desc: crate::ShaderModuleDescriptor<'_>,
        shader_bound_checks: crate::ShaderRuntimeChecks,
    ) -> DispatchShaderModule;
    unsafe fn create_shader_module_spirv(
        &self,
        desc: &crate::ShaderModuleDescriptorSpirV<'_>,
    ) -> DispatchShaderModule;
    fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor<'_>,
    ) -> DispatchBindGroupLayout;
    fn create_bind_group(&self, desc: &crate::BindGroupDescriptor<'_>) -> DispatchBindGroup;
    fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<'_>,
    ) -> DispatchPipelineLayout;
    fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<'_>,
    ) -> DispatchRenderPipeline;
    fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<'_>,
    ) -> DispatchComputePipeline;
    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> DispatchPipelineCache;
    fn create_buffer(&self, desc: &crate::BufferDescriptor<'_>) -> DispatchBuffer;
    fn create_texture(&self, desc: &crate::TextureDescriptor<'_>) -> DispatchTexture;
    fn create_blas(
        &self,
        desc: &crate::CreateBlasDescriptor<'_>,
        sizes: crate::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, DispatchBlas);
    fn create_tlas(&self, desc: &crate::CreateTlasDescriptor<'_>) -> DispatchTlas;
    fn create_sampler(&self, desc: &crate::SamplerDescriptor<'_>) -> DispatchSampler;
    fn create_query_set(&self, desc: &crate::QuerySetDescriptor<'_>) -> DispatchQuerySet;
    fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<'_>,
    ) -> DispatchCommandEncoder;
    fn create_render_bundle_encoder(
        &self,
        desc: &crate::RenderBundleEncoderDescriptor<'_>,
    ) -> DispatchRenderBundleEncoder;

    fn set_device_lost_callback(&self, device_lost_callback: BoxDeviceLostCallback);

    fn on_uncaptured_error(&self, handler: Box<dyn crate::UncapturedErrorHandler>);
    fn push_error_scope(&self, filter: crate::ErrorFilter);
    fn pop_error_scope(&self) -> Pin<Box<dyn PopErrorScopeFuture>>;

    fn start_capture(&self);
    fn stop_capture(&self);

    fn poll(&self, maintain: crate::Maintain) -> crate::MaintainResult;

    fn get_internal_counters(&self) -> crate::InternalCounters;
    fn generate_allocator_report(&self) -> Option<crate::AllocatorReport>;

    fn destroy(&self);
}

pub trait QueueInterface: CommonTraits {
    fn write_buffer(&self, buffer: &DispatchBuffer, offset: crate::BufferAddress, data: &[u8]);

    fn create_staging_buffer(&self, size: crate::BufferSize) -> Option<DispatchQueueWriteBuffer>;
    fn validate_write_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: crate::BufferSize,
    ) -> Option<()>;
    fn write_staging_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        staging_buffer: &DispatchQueueWriteBuffer,
    );

    fn write_texture(
        &self,
        texture: crate::TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: crate::TexelCopyBufferLayout,
        size: crate::Extent3d,
    );
    #[cfg(any(webgpu, webgl))]
    fn copy_external_image_to_texture(
        &self,
        source: &crate::CopyExternalImageSourceInfo,
        dest: crate::CopyExternalImageDestInfo<&crate::api::Texture>,
        size: crate::Extent3d,
    );

    fn submit(&self, command_buffers: &mut dyn Iterator<Item = DispatchCommandBuffer>) -> u64;

    fn get_timestamp_period(&self) -> f32;
    fn on_submitted_work_done(&self, callback: BoxSubmittedWorkDoneCallback);
}

pub trait ShaderModuleInterface: CommonTraits {
    fn get_compilation_info(&self) -> Pin<Box<dyn ShaderCompilationInfoFuture>>;
}
pub trait BindGroupLayoutInterface: CommonTraits {}
pub trait BindGroupInterface: CommonTraits {}
pub trait TextureViewInterface: CommonTraits {}
pub trait SamplerInterface: CommonTraits {}
pub trait BufferInterface: CommonTraits {
    fn map_async(
        &self,
        mode: crate::MapMode,
        range: Range<crate::BufferAddress>,
        callback: BufferMapCallback,
    );
    fn get_mapped_range(&self, sub_range: Range<crate::BufferAddress>)
        -> DispatchBufferMappedRange;
    #[cfg(webgpu)]
    fn get_mapped_range_as_array_buffer(
        &self,
        sub_range: Range<crate::BufferAddress>,
    ) -> Option<js_sys::ArrayBuffer>;

    fn unmap(&self);

    fn destroy(&self);
}
pub trait TextureInterface: CommonTraits {
    fn create_view(&self, desc: &crate::TextureViewDescriptor<'_>) -> DispatchTextureView;

    fn destroy(&self);
}
pub trait BlasInterface: CommonTraits {
    fn destroy(&self);
}
pub trait TlasInterface: CommonTraits {
    fn destroy(&self);
}
pub trait QuerySetInterface: CommonTraits {}
pub trait PipelineLayoutInterface: CommonTraits {}
pub trait RenderPipelineInterface: CommonTraits {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
}
pub trait ComputePipelineInterface: CommonTraits {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
}
pub trait PipelineCacheInterface: CommonTraits {
    fn get_data(&self) -> Option<Vec<u8>>;
}
pub trait CommandEncoderInterface: CommonTraits {
    fn copy_buffer_to_buffer(
        &self,
        source: &DispatchBuffer,
        source_offset: crate::BufferAddress,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
        copy_size: crate::BufferAddress,
    );
    fn copy_buffer_to_texture(
        &self,
        source: crate::TexelCopyBufferInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    );
    fn copy_texture_to_buffer(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyBufferInfo<'_>,
        copy_size: crate::Extent3d,
    );
    fn copy_texture_to_texture(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    );

    fn begin_compute_pass(&self, desc: &crate::ComputePassDescriptor<'_>) -> DispatchComputePass;
    fn begin_render_pass(&self, desc: &crate::RenderPassDescriptor<'_>) -> DispatchRenderPass;
    fn finish(&mut self) -> DispatchCommandBuffer;

    fn clear_texture(
        &self,
        texture: &DispatchTexture,
        subresource_range: &crate::ImageSubresourceRange,
    );
    fn clear_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferAddress>,
    );

    fn insert_debug_marker(&self, label: &str);
    fn push_debug_group(&self, label: &str);
    fn pop_debug_group(&self);

    fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32);
    fn resolve_query_set(
        &self,
        query_set: &DispatchQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
    );

    fn build_acceleration_structures_unsafe_tlas<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        tlas: &mut dyn Iterator<Item = &'a crate::TlasBuildEntry<'a>>,
    );
    fn build_acceleration_structures<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        tlas: &mut dyn Iterator<Item = &'a crate::TlasPackage>,
    );
}
pub trait ComputePassInterface: CommonTraits {
    fn set_pipeline(&mut self, pipeline: &DispatchComputePipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_push_constants(&mut self, offset: u32, data: &[u8]);

    fn insert_debug_marker(&mut self, label: &str);
    fn push_debug_group(&mut self, group_label: &str);
    fn pop_debug_group(&mut self);

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn end_pipeline_statistics_query(&mut self);

    fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32);
    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn end(&mut self);
}
pub trait RenderPassInterface: CommonTraits {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_push_constants(&mut self, stages: crate::ShaderStages, offset: u32, data: &[u8]);
    fn set_blend_constant(&mut self, color: crate::Color);
    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32);
    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    );
    fn set_stencil_reference(&mut self, reference: u32);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    );
    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    );
    fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    );
    fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    );

    fn insert_debug_marker(&mut self, label: &str);
    fn push_debug_group(&mut self, group_label: &str);
    fn pop_debug_group(&mut self);

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn begin_occlusion_query(&mut self, query_index: u32);
    fn end_occlusion_query(&mut self);
    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn end_pipeline_statistics_query(&mut self);

    fn execute_bundles(&mut self, render_bundles: &mut dyn Iterator<Item = &DispatchRenderBundle>);

    fn end(&mut self);
}

pub trait RenderBundleEncoderInterface: CommonTraits {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_push_constants(&mut self, stages: crate::ShaderStages, offset: u32, data: &[u8]);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );

    fn finish(self, desc: &crate::RenderBundleDescriptor<'_>) -> DispatchRenderBundle
    where
        Self: Sized;
}

pub trait CommandBufferInterface: CommonTraits {}
pub trait RenderBundleInterface: CommonTraits {}

pub trait SurfaceInterface: CommonTraits {
    fn get_capabilities(&self, adapter: &DispatchAdapter) -> crate::SurfaceCapabilities;

    fn configure(&self, device: &DispatchDevice, config: &crate::SurfaceConfiguration);
    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        crate::SurfaceStatus,
        DispatchSurfaceOutputDetail,
    );
}

pub trait SurfaceOutputDetailInterface: CommonTraits {
    fn present(&self);
    fn texture_discard(&self);
}

pub trait QueueWriteBufferInterface: CommonTraits {
    fn slice(&self) -> &[u8];

    fn slice_mut(&mut self) -> &mut [u8];
}

pub trait BufferMappedRangeInterface: CommonTraits {
    fn slice(&self) -> &[u8];
    fn slice_mut(&mut self) -> &mut [u8];
}

/// Generates Dispatch types for each of the interfaces. Each type is a wrapper around the
/// wgpu_core and webgpu types, and derefs to the appropriate interface trait-object.
///
/// When there is only one backend, deviritualization fires and all dispatches should turn into
/// direct calls. If there are multiple, some dispatching will occur.
///
/// This also provides `as_*` methods so that the backend implementations can dereference other
/// arguments. These are similarly free when there is only one backend.
///
/// In the future, we may want a truly generic backend, which could be extended from this enum.
macro_rules! dispatch_types_inner {
    (
        {ref type $name:ident = $subtype:ident: $interface:ident};
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
        pub enum $name {
            #[cfg(wgpu_core)]
            Core(Arc<backend::wgpu_core::interface_types::$subtype>),
            #[cfg(webgpu)]
            WebGPU(Arc<backend::webgpu::interface_types::$subtype>),
            #[allow(private_interfaces)]
            //#[cfg(custom)]
            Custom(backend::custom::interface_types::$subtype),
        }

        impl $name {
            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core(&self) -> &backend::wgpu_core::interface_types::$subtype {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core_opt(&self) -> Option<&backend::wgpu_core::interface_types::$subtype> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu(&self) -> &backend::webgpu::interface_types::$subtype {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu_opt(&self) -> Option<&backend::webgpu::interface_types::$subtype> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            //#[cfg(custom)]
            #[inline]
            pub fn custom<T: $interface>(t: T) -> Self {
                Self::Custom(backend::custom::interface_types::$subtype::new(t))
            }
        }

        #[cfg(wgpu_core)]
        impl From<backend::wgpu_core::interface_types::$subtype> for $name {
            #[inline]
            fn from(value: backend::wgpu_core::interface_types::$subtype) -> Self {
                Self::Core(Arc::new(value))
            }
        }

        #[cfg(webgpu)]
        impl From<backend::webgpu::interface_types::$subtype> for $name {
            #[inline]
            fn from(value: backend::webgpu::interface_types::$subtype) -> Self {
                Self::WebGPU(Arc::new(value))
            }
        }

        impl std::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value.as_ref(),
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value.as_ref(),
                    //#[cfg(webgpu)]
                    Self::Custom(value) => value.deref(),
                }
            }
        }
    };
    (
        {mut type $name:ident = $subtype:ident: $interface:ident};
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            #[cfg(wgpu_core)]
            Core(backend::wgpu_core::interface_types::$subtype),
            #[cfg(webgpu)]
            WebGPU(backend::webgpu::interface_types::$subtype),
            #[allow(private_interfaces)]
            //#[cfg(custom)]
            Custom(backend::custom::interface_types::$subtype),
        }

        impl $name {
            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core(&self) -> &backend::wgpu_core::interface_types::$subtype {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core_mut(&mut self) -> &mut backend::wgpu_core::interface_types::$subtype {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core_opt(&self) -> Option<&backend::wgpu_core::interface_types::$subtype> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(unused)]
            pub fn as_core_mut_opt(
                &mut self,
            ) -> Option<&mut backend::wgpu_core::interface_types::$subtype> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu(&self) -> &backend::webgpu::interface_types::$subtype {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu_mut(&mut self) -> &mut backend::webgpu::interface_types::$subtype {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu_opt(&self) -> Option<&backend::webgpu::interface_types::$subtype> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(unused)]
            pub fn as_webgpu_mut_opt(
                &mut self,
            ) -> Option<&mut backend::webgpu::interface_types::$subtype> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            //#[cfg(custom)]
            #[inline]
            pub fn custom<T: $interface>(t: T) -> Self {
                Self::Custom(backend::custom::interface_types::$subtype::new(t))
            }
        }

        #[cfg(wgpu_core)]
        impl From<backend::wgpu_core::interface_types::$subtype> for $name {
            #[inline]
            fn from(value: backend::wgpu_core::interface_types::$subtype) -> Self {
                Self::Core(value)
            }
        }

        #[cfg(webgpu)]
        impl From<backend::webgpu::interface_types::$subtype> for $name {
            #[inline]
            fn from(value: backend::webgpu::interface_types::$subtype) -> Self {
                Self::WebGPU(value)
            }
        }

        impl std::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value,
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value,
                    //#[cfg(custom)]
                    Self::Custom(value) => value.deref(),
                }
            }
        }

        impl std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value,
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value,
                    //#[cfg(custom)]
                    Self::Custom(value) => value.deref_mut(),
                }
            }
        }
    };
}

dispatch_types_inner! {
    {
        ref type DispatchInstance = Instance:InstanceInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchAdapter = Adapter:AdapterInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchDevice = Device:DeviceInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchQueue = Queue:QueueInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchShaderModule = ShaderModule:ShaderModuleInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchBindGroupLayout = BindGroupLayout:BindGroupLayoutInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchBindGroup = BindGroup:BindGroupInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchTextureView = TextureView:TextureViewInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchSampler = Sampler:SamplerInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchBuffer = Buffer:BufferInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchTexture = Texture:TextureInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchBlas = Blas:BlasInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchTlas = Tlas:TlasInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchQuerySet = QuerySet:QuerySetInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchPipelineLayout = PipelineLayout:PipelineLayoutInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchRenderPipeline = RenderPipeline:RenderPipelineInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchComputePipeline = ComputePipeline:ComputePipelineInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchPipelineCache = PipelineCache:PipelineCacheInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchCommandEncoder = CommandEncoder:CommandEncoderInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchComputePass = ComputePass:ComputePassInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchRenderPass = RenderPass:RenderPassInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchCommandBuffer = CommandBuffer:CommandBufferInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchRenderBundleEncoder = RenderBundleEncoder:RenderBundleEncoderInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchRenderBundle = RenderBundle:RenderBundleInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchSurface = Surface:SurfaceInterface
    };
}
dispatch_types_inner! {
    {
        ref type DispatchSurfaceOutputDetail = SurfaceOutputDetail:SurfaceOutputDetailInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchQueueWriteBuffer = QueueWriteBuffer:QueueWriteBufferInterface
    };
}
dispatch_types_inner! {
    {
        mut type DispatchBufferMappedRange = BufferMappedRange:BufferMappedRangeInterface
    };
}
