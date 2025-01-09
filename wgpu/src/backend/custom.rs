#![allow(ambiguous_wide_pointer_comparisons)]

use crate::backend::Dispatch;
use crate::backend::DispatchMut;
use crate::dispatch::{
    AdapterInterface, BindGroupInterface, BindGroupLayoutInterface, BlasInterface, BufferInterface,
    BufferMappedRangeInterface, CommandBufferInterface, CommandEncoderInterface,
    ComputePassInterface, ComputePipelineInterface, DeviceInterface, InstanceInterface,
    InterfaceTypes, PipelineCacheInterface, PipelineLayoutInterface, QuerySetInterface,
    QueueInterface, QueueWriteBufferInterface, RenderBundleEncoderInterface, RenderBundleInterface,
    RenderPassInterface, RenderPipelineInterface, SamplerInterface, ShaderModuleInterface,
    SurfaceInterface, SurfaceOutputDetailInterface, TextureInterface, TextureViewInterface,
    TlasInterface,
};

use std::sync::Arc;

macro_rules! dyn_type {
    // cloning of arc forbidden
    (pub mut struct $name:ident(dyn $interface:tt)) => {
        #[derive(Debug)]
        pub struct $name(Arc<dyn $interface>);
        crate::cmp::impl_eq_ord_hash_arc_address!($name => .0);

        impl $name {
            pub fn new<T: $interface>(t: T) -> Self {
                Self(Arc::new(t))
            }
        }

        impl Dispatch for $name {
            type Target = dyn $interface;

            #[inline]
            fn dispatch(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }

        impl DispatchMut for $name {
            #[inline]
            fn dispatch_mut(&mut self) -> &mut Self::Target {
                Arc::get_mut(&mut self.0).expect("")
            }
        }
    };
    // cloning of arc is allowed
    (pub ref struct $name:ident(dyn $interface:tt)) => {
        #[derive(Debug, Clone)]
        pub struct $name(Arc<dyn $interface>);
        crate::cmp::impl_eq_ord_hash_arc_address!($name => .0);

        impl $name {
            pub fn new<T: $interface>(t: T) -> Self {
                Self(Arc::new(t))
            }
        }

        impl Dispatch for $name {
            type Target = dyn $interface;

            #[inline]
            fn dispatch(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
    };
}

dyn_type!(pub ref struct DynContext(dyn InstanceInterface));
dyn_type!(pub ref struct DynAdapter(dyn AdapterInterface));
dyn_type!(pub ref struct DynDevice(dyn DeviceInterface));
dyn_type!(pub ref struct DynQueue(dyn QueueInterface));
dyn_type!(pub ref struct DynShaderModule(dyn ShaderModuleInterface));
dyn_type!(pub ref struct DynBindGroupLayout(dyn BindGroupLayoutInterface));
dyn_type!(pub ref struct DynBindGroup(dyn BindGroupInterface));
dyn_type!(pub ref struct DynTextureView(dyn TextureViewInterface));
dyn_type!(pub ref struct DynSampler(dyn SamplerInterface));
dyn_type!(pub ref struct DynBuffer(dyn BufferInterface));
dyn_type!(pub ref struct DynTexture(dyn TextureInterface));
dyn_type!(pub ref struct DynBlas(dyn BlasInterface));
dyn_type!(pub ref struct DynTlas(dyn TlasInterface));
dyn_type!(pub ref struct DynQuerySet(dyn QuerySetInterface));
dyn_type!(pub ref struct DynPipelineLayout(dyn PipelineLayoutInterface));
dyn_type!(pub ref struct DynRenderPipeline(dyn RenderPipelineInterface));
dyn_type!(pub ref struct DynComputePipeline(dyn ComputePipelineInterface));
dyn_type!(pub ref struct DynPipelineCache(dyn PipelineCacheInterface));
dyn_type!(pub mut struct DynCommandEncoder(dyn CommandEncoderInterface));
dyn_type!(pub mut struct DynComputePass(dyn ComputePassInterface));
dyn_type!(pub mut struct DynRenderPass(dyn RenderPassInterface));
dyn_type!(pub ref struct DynCommandBuffer(dyn CommandBufferInterface));
dyn_type!(pub mut struct DynRenderBundleEncoder(dyn RenderBundleEncoderInterface));
dyn_type!(pub ref struct DynRenderBundle(dyn RenderBundleInterface));
dyn_type!(pub ref struct DynSurface(dyn SurfaceInterface));
dyn_type!(pub ref struct DynSurfaceOutputDetail(dyn SurfaceOutputDetailInterface));
dyn_type!(pub mut struct DynQueueWriteBuffer(dyn QueueWriteBufferInterface));
dyn_type!(pub mut struct DynBufferMappedRange(dyn BufferMappedRangeInterface));

impl InterfaceTypes for DynContext {
    type Instance = DynContext;

    type Adapter = DynAdapter;

    type Device = DynDevice;

    type Queue = DynQueue;

    type ShaderModule = DynShaderModule;

    type BindGroupLayout = DynBindGroupLayout;

    type BindGroup = DynBindGroup;

    type TextureView = DynTextureView;

    type Sampler = DynSampler;

    type Buffer = DynBuffer;

    type Texture = DynTexture;

    type Blas = DynBlas;

    type Tlas = DynTlas;

    type QuerySet = DynQuerySet;

    type PipelineLayout = DynPipelineLayout;

    type RenderPipeline = DynRenderPipeline;

    type ComputePipeline = DynComputePipeline;

    type PipelineCache = DynPipelineCache;

    type CommandEncoder = DynCommandEncoder;

    type ComputePass = DynComputePass;

    type RenderPass = DynRenderPass;

    type CommandBuffer = DynCommandBuffer;

    type RenderBundleEncoder = DynRenderBundleEncoder;

    type RenderBundle = DynRenderBundle;

    type Surface = DynSurface;

    type SurfaceOutputDetail = DynSurfaceOutputDetail;

    type QueueWriteBuffer = DynQueueWriteBuffer;

    type BufferMappedRange = DynBufferMappedRange;
}
