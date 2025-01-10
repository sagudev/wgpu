#![allow(dead_code)]
use std::pin::Pin;
use std::sync::Arc;

use wgpu::custom::{
    AdapterInterface, DeviceInterface, DispatchAdapter, DispatchDevice, DispatchQueue,
    DispatchShaderModule, DispatchSurface, InstanceInterface, QueueInterface, RequestAdapterFuture,
    ShaderModuleInterface,
};

#[derive(Debug, Clone)]
pub struct Counter(Arc<()>);

impl Counter {
    pub fn new() -> Self {
        Self(Arc::new(()))
    }

    pub fn count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}

#[derive(Debug)]
pub struct CustomInstance(pub Counter);

impl InstanceInterface for CustomInstance {
    fn new(__desc: &wgpu::InstanceDescriptor) -> Self
    where
        Self: Sized,
    {
        Self(Counter::new())
    }

    unsafe fn create_surface(
        &self,
        _target: wgpu::SurfaceTargetUnsafe,
    ) -> Result<DispatchSurface, wgpu::CreateSurfaceError> {
        unimplemented!()
    }

    fn request_adapter(
        &self,
        _options: &wgpu::RequestAdapterOptions<'_, '_>,
    ) -> std::pin::Pin<Box<dyn RequestAdapterFuture>> {
        Box::pin(std::future::ready(Some(DispatchAdapter::custom(
            CustomAdapter(self.0.clone()),
        ))))
    }

    fn poll_all_devices(&self, _force_wait: bool) -> bool {
        unimplemented!()
    }
}

#[derive(Debug)]
struct CustomAdapter(Counter);

impl AdapterInterface for CustomAdapter {
    fn request_device(
        &self,
        desc: &wgpu::DeviceDescriptor<'_>,
        _trace_dir: Option<&std::path::Path>,
    ) -> Pin<Box<dyn wgpu::custom::RequestDeviceFuture>> {
        assert_eq!(desc.label, Some("device"));
        let res: Result<_, wgpu::RequestDeviceError> = Ok((
            DispatchDevice::custom(CustomDevice(self.0.clone())),
            DispatchQueue::custom(CustomQueue(self.0.clone())),
        ));
        Box::pin(std::future::ready(res))
    }

    fn is_surface_supported(&self, _surface: &DispatchSurface) -> bool {
        unimplemented!()
    }

    fn features(&self) -> wgpu::Features {
        unimplemented!()
    }

    fn limits(&self) -> wgpu::Limits {
        unimplemented!()
    }

    fn downlevel_capabilities(&self) -> wgpu::DownlevelCapabilities {
        unimplemented!()
    }

    fn get_info(&self) -> wgpu::AdapterInfo {
        unimplemented!()
    }

    fn get_texture_format_features(
        &self,
        _format: wgpu::TextureFormat,
    ) -> wgpu::TextureFormatFeatures {
        unimplemented!()
    }

    fn get_presentation_timestamp(&self) -> wgpu::PresentationTimestamp {
        unimplemented!()
    }
}

#[derive(Debug)]
struct CustomDevice(Counter);

impl DeviceInterface for CustomDevice {
    fn features(&self) -> wgpu::Features {
        unimplemented!()
    }

    fn limits(&self) -> wgpu::Limits {
        unimplemented!()
    }

    fn create_shader_module(
        &self,
        desc: wgpu::ShaderModuleDescriptor<'_>,
        _shader_bound_checks: wgpu::ShaderRuntimeChecks,
    ) -> DispatchShaderModule {
        assert_eq!(desc.label, Some("shader"));
        DispatchShaderModule::custom(CustomShaderModule(self.0.clone()))
    }

    unsafe fn create_shader_module_spirv(
        &self,
        _desc: &wgpu::ShaderModuleDescriptorSpirV<'_>,
    ) -> DispatchShaderModule {
        unimplemented!()
    }

    fn create_bind_group_layout(
        &self,
        _desc: &wgpu::BindGroupLayoutDescriptor<'_>,
    ) -> wgpu::custom::DispatchBindGroupLayout {
        unimplemented!()
    }

    fn create_bind_group(
        &self,
        _desc: &wgpu::BindGroupDescriptor<'_>,
    ) -> wgpu::custom::DispatchBindGroup {
        unimplemented!()
    }

    fn create_pipeline_layout(
        &self,
        _desc: &wgpu::PipelineLayoutDescriptor<'_>,
    ) -> wgpu::custom::DispatchPipelineLayout {
        unimplemented!()
    }

    fn create_render_pipeline(
        &self,
        _desc: &wgpu::RenderPipelineDescriptor<'_>,
    ) -> wgpu::custom::DispatchRenderPipeline {
        unimplemented!()
    }

    fn create_compute_pipeline(
        &self,
        _desc: &wgpu::ComputePipelineDescriptor<'_>,
    ) -> wgpu::custom::DispatchComputePipeline {
        unimplemented!()
    }

    unsafe fn create_pipeline_cache(
        &self,
        _desc: &wgpu::PipelineCacheDescriptor<'_>,
    ) -> wgpu::custom::DispatchPipelineCache {
        unimplemented!()
    }

    fn create_buffer(&self, _desc: &wgpu::BufferDescriptor<'_>) -> wgpu::custom::DispatchBuffer {
        unimplemented!()
    }

    fn create_texture(&self, _desc: &wgpu::TextureDescriptor<'_>) -> wgpu::custom::DispatchTexture {
        unimplemented!()
    }

    fn create_blas(
        &self,
        _desc: &wgpu::CreateBlasDescriptor<'_>,
        _sizes: wgpu::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, wgpu::custom::DispatchBlas) {
        unimplemented!()
    }

    fn create_tlas(&self, _desc: &wgpu::CreateTlasDescriptor<'_>) -> wgpu::custom::DispatchTlas {
        unimplemented!()
    }

    fn create_sampler(&self, _desc: &wgpu::SamplerDescriptor<'_>) -> wgpu::custom::DispatchSampler {
        unimplemented!()
    }

    fn create_query_set(
        &self,
        _desc: &wgpu::QuerySetDescriptor<'_>,
    ) -> wgpu::custom::DispatchQuerySet {
        unimplemented!()
    }

    fn create_command_encoder(
        &self,
        _desc: &wgpu::CommandEncoderDescriptor<'_>,
    ) -> wgpu::custom::DispatchCommandEncoder {
        unimplemented!()
    }

    fn create_render_bundle_encoder(
        &self,
        _desc: &wgpu::RenderBundleEncoderDescriptor<'_>,
    ) -> wgpu::custom::DispatchRenderBundleEncoder {
        unimplemented!()
    }

    fn set_device_lost_callback(&self, _device_lost_callback: wgpu::custom::BoxDeviceLostCallback) {
        unimplemented!()
    }

    fn on_uncaptured_error(&self, _handler: Box<dyn wgpu::UncapturedErrorHandler>) {
        unimplemented!()
    }

    fn push_error_scope(&self, _filter: wgpu::ErrorFilter) {
        unimplemented!()
    }

    fn pop_error_scope(&self) -> Pin<Box<dyn wgpu::custom::PopErrorScopeFuture>> {
        unimplemented!()
    }

    fn start_capture(&self) {
        unimplemented!()
    }

    fn stop_capture(&self) {
        unimplemented!()
    }

    fn poll(&self, _maintain: wgpu::Maintain) -> wgpu::MaintainResult {
        unimplemented!()
    }

    fn get_internal_counters(&self) -> wgpu::InternalCounters {
        unimplemented!()
    }

    fn generate_allocator_report(&self) -> Option<wgpu::AllocatorReport> {
        unimplemented!()
    }

    fn destroy(&self) {
        unimplemented!()
    }
}

#[derive(Debug)]
struct CustomShaderModule(Counter);

impl ShaderModuleInterface for CustomShaderModule {
    fn get_compilation_info(&self) -> Pin<Box<dyn wgpu::custom::ShaderCompilationInfoFuture>> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct CustomQueue(Counter);

impl QueueInterface for CustomQueue {
    fn write_buffer(
        &self,
        _buffer: &wgpu::custom::DispatchBuffer,
        _offset: wgpu::BufferAddress,
        _data: &[u8],
    ) {
        todo!()
    }

    fn create_staging_buffer(
        &self,
        _size: wgpu::BufferSize,
    ) -> Option<wgpu::custom::DispatchQueueWriteBuffer> {
        todo!()
    }

    fn validate_write_buffer(
        &self,
        _buffer: &wgpu::custom::DispatchBuffer,
        _offset: wgpu::BufferAddress,
        _size: wgpu::BufferSize,
    ) -> Option<()> {
        todo!()
    }

    fn write_staging_buffer(
        &self,
        _buffer: &wgpu::custom::DispatchBuffer,
        _offset: wgpu::BufferAddress,
        _staging_buffer: &wgpu::custom::DispatchQueueWriteBuffer,
    ) {
        todo!()
    }

    fn write_texture(
        &self,
        _texture: wgpu::TexelCopyTextureInfo<'_>,
        _data: &[u8],
        _data_layout: wgpu::TexelCopyBufferLayout,
        _size: wgpu::Extent3d,
    ) {
        todo!()
    }

    fn submit(
        &self,
        _command_buffers: &mut dyn Iterator<Item = wgpu::custom::DispatchCommandBuffer>,
    ) -> u64 {
        todo!()
    }

    fn get_timestamp_period(&self) -> f32 {
        todo!()
    }

    fn on_submitted_work_done(&self, _callback: wgpu::custom::BoxSubmittedWorkDoneCallback) {
        todo!()
    }
}
