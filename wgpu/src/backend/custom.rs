#![allow(ambiguous_wide_pointer_comparisons)]

use crate::dispatch::{
    self, AdapterInterface, BindGroupInterface, BindGroupLayoutInterface, BlasInterface,
    BufferInterface, BufferMappedRangeInterface, CommandBufferInterface, CommandEncoderInterface,
    ComputePassInterface, ComputePipelineInterface, DeviceInterface, DispatchBindGroup,
    DispatchBindGroupLayout, DispatchBuffer, DispatchCommandBuffer, DispatchComputePipeline,
    DispatchQuerySet, DispatchRenderPipeline, DispatchTexture, InstanceInterface, InterfaceTypes,
    PipelineCacheInterface, PipelineLayoutInterface, QuerySetInterface, QueueInterface,
    QueueWriteBufferInterface, RenderBundleEncoderInterface, RenderBundleInterface,
    RenderPassInterface, RenderPipelineInterface, SamplerInterface, ShaderModuleInterface,
    SurfaceInterface, SurfaceOutputDetailInterface, TextureInterface, TextureViewInterface,
    TlasInterface,
};

use std::{ops::Range, pin::Pin, sync::Arc};

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
    };
}

dyn_type!(pub ref struct DynContext(dyn InstanceInterface));

impl InstanceInterface for DynContext {
    fn new(_desc: &crate::InstanceDescriptor) -> Self
    where
        Self: Sized,
    {
        unimplemented!("DynInstance cannot be created without knowing inner type")
        // while this info could be provided via generics we do not want that
    }

    unsafe fn create_surface(
        &self,
        target: crate::SurfaceTargetUnsafe,
    ) -> Result<dispatch::DispatchSurface, crate::CreateSurfaceError> {
        unsafe { self.0.create_surface(target) }
    }

    fn request_adapter(
        &self,
        options: &crate::RequestAdapterOptions<'_, '_>,
    ) -> Pin<Box<dyn dispatch::RequestAdapterFuture>> {
        self.0.request_adapter(options)
    }

    fn poll_all_devices(&self, force_wait: bool) -> bool {
        self.0.poll_all_devices(force_wait)
    }
}

dyn_type!(pub ref struct DynAdapter(dyn AdapterInterface));

impl AdapterInterface for DynAdapter {
    fn request_device(
        &self,
        desc: &crate::DeviceDescriptor<'_>,
        trace_dir: Option<&std::path::Path>,
    ) -> Pin<Box<dyn dispatch::RequestDeviceFuture>> {
        self.0.request_device(desc, trace_dir)
    }

    fn is_surface_supported(&self, surface: &dispatch::DispatchSurface) -> bool {
        self.0.is_surface_supported(surface)
    }

    fn features(&self) -> crate::Features {
        self.0.features()
    }

    fn limits(&self) -> crate::Limits {
        self.0.limits()
    }

    fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities {
        self.0.downlevel_capabilities()
    }

    fn get_info(&self) -> crate::AdapterInfo {
        self.0.get_info()
    }

    fn get_texture_format_features(
        &self,
        format: crate::TextureFormat,
    ) -> crate::TextureFormatFeatures {
        self.0.get_texture_format_features(format)
    }

    fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp {
        self.0.get_presentation_timestamp()
    }
}

dyn_type!(pub ref struct DynDevice(dyn DeviceInterface));

impl DeviceInterface for DynDevice {
    fn features(&self) -> crate::Features {
        self.0.features()
    }

    fn limits(&self) -> crate::Limits {
        self.0.limits()
    }

    fn create_shader_module(
        &self,
        desc: crate::ShaderModuleDescriptor<'_>,
        shader_runtime_checks: wgt::ShaderRuntimeChecks,
    ) -> dispatch::DispatchShaderModule {
        self.0.create_shader_module(desc, shader_runtime_checks)
    }

    unsafe fn create_shader_module_spirv(
        &self,
        desc: &crate::ShaderModuleDescriptorSpirV<'_>,
    ) -> dispatch::DispatchShaderModule {
        unsafe { self.0.create_shader_module_spirv(desc) }
    }

    fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor<'_>,
    ) -> dispatch::DispatchBindGroupLayout {
        self.0.create_bind_group_layout(desc)
    }

    fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<'_>,
    ) -> dispatch::DispatchBindGroup {
        self.0.create_bind_group(desc)
    }

    fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<'_>,
    ) -> dispatch::DispatchPipelineLayout {
        self.0.create_pipeline_layout(desc)
    }

    fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<'_>,
    ) -> dispatch::DispatchRenderPipeline {
        self.0.create_render_pipeline(desc)
    }

    fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<'_>,
    ) -> dispatch::DispatchComputePipeline {
        self.0.create_compute_pipeline(desc)
    }

    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> dispatch::DispatchPipelineCache {
        unsafe { self.0.create_pipeline_cache(desc) }
    }

    fn create_buffer(&self, desc: &crate::BufferDescriptor<'_>) -> dispatch::DispatchBuffer {
        self.0.create_buffer(desc)
    }

    fn create_texture(&self, desc: &crate::TextureDescriptor<'_>) -> dispatch::DispatchTexture {
        self.0.create_texture(desc)
    }

    fn create_blas(
        &self,
        desc: &crate::CreateBlasDescriptor<'_>,
        sizes: crate::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, dispatch::DispatchBlas) {
        self.0.create_blas(desc, sizes)
    }

    fn create_tlas(&self, desc: &crate::CreateTlasDescriptor<'_>) -> dispatch::DispatchTlas {
        self.0.create_tlas(desc)
    }

    fn create_sampler(&self, desc: &crate::SamplerDescriptor<'_>) -> dispatch::DispatchSampler {
        self.0.create_sampler(desc)
    }

    fn create_query_set(&self, desc: &crate::QuerySetDescriptor<'_>) -> dispatch::DispatchQuerySet {
        self.0.create_query_set(desc)
    }

    fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<'_>,
    ) -> dispatch::DispatchCommandEncoder {
        self.0.create_command_encoder(desc)
    }

    fn create_render_bundle_encoder(
        &self,
        desc: &crate::RenderBundleEncoderDescriptor<'_>,
    ) -> dispatch::DispatchRenderBundleEncoder {
        self.0.create_render_bundle_encoder(desc)
    }

    fn set_device_lost_callback(&self, device_lost_callback: dispatch::BoxDeviceLostCallback) {
        self.0.set_device_lost_callback(device_lost_callback);
    }

    fn on_uncaptured_error(&self, handler: Box<dyn crate::UncapturedErrorHandler>) {
        self.0.on_uncaptured_error(handler);
    }

    fn push_error_scope(&self, filter: crate::ErrorFilter) {
        self.0.push_error_scope(filter);
    }

    fn pop_error_scope(&self) -> Pin<Box<dyn dispatch::PopErrorScopeFuture>> {
        self.0.pop_error_scope()
    }

    fn start_capture(&self) {
        self.0.start_capture();
    }

    fn stop_capture(&self) {
        self.0.stop_capture();
    }

    fn poll(&self, maintain: crate::Maintain) -> crate::MaintainResult {
        self.0.poll(maintain)
    }

    fn get_internal_counters(&self) -> crate::InternalCounters {
        self.0.get_internal_counters()
    }

    fn generate_allocator_report(&self) -> Option<wgt::AllocatorReport> {
        self.0.generate_allocator_report()
    }

    fn destroy(&self) {
        self.0.destroy();
    }
}

dyn_type!(pub ref struct DynQueue(dyn QueueInterface));

impl QueueInterface for DynQueue {
    fn write_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        data: &[u8],
    ) {
        self.0.write_buffer(buffer, offset, data);
    }

    fn create_staging_buffer(
        &self,
        size: crate::BufferSize,
    ) -> Option<dispatch::DispatchQueueWriteBuffer> {
        self.0.create_staging_buffer(size)
    }

    fn validate_write_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: wgt::BufferAddress,
        size: wgt::BufferSize,
    ) -> Option<()> {
        self.0.validate_write_buffer(buffer, offset, size)
    }

    fn write_staging_buffer(
        &self,
        buffer: &dispatch::DispatchBuffer,
        offset: crate::BufferAddress,
        staging_buffer: &dispatch::DispatchQueueWriteBuffer,
    ) {
        self.0.write_staging_buffer(buffer, offset, staging_buffer);
    }

    fn write_texture(
        &self,
        texture: crate::TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: crate::TexelCopyBufferLayout,
        size: crate::Extent3d,
    ) {
        self.0.write_texture(texture, data, data_layout, size);
    }

    fn submit(
        &self,
        command_buffers: &mut dyn Iterator<Item = dispatch::DispatchCommandBuffer>,
    ) -> u64 {
        self.0.submit(command_buffers)
    }

    fn get_timestamp_period(&self) -> f32 {
        self.0.get_timestamp_period()
    }

    fn on_submitted_work_done(&self, callback: dispatch::BoxSubmittedWorkDoneCallback) {
        self.0.on_submitted_work_done(callback);
    }

    #[cfg(any(webgpu, webgl))]
    fn copy_external_image_to_texture(
        &self,
        _source: &wgt::CopyExternalImageSourceInfo,
        _dest: wgt::CopyExternalImageDestInfo<&crate::api::Texture>,
        _size: crate::Extent3d,
    ) {
        unimplemented!()
    }
}

dyn_type!(pub ref struct DynShaderModule(dyn ShaderModuleInterface));

impl ShaderModuleInterface for DynShaderModule {
    fn get_compilation_info(&self) -> Pin<Box<dyn dispatch::ShaderCompilationInfoFuture>> {
        self.0.get_compilation_info()
    }
}

dyn_type!(pub ref struct DynBindGroupLayout(dyn BindGroupLayoutInterface));

impl BindGroupLayoutInterface for DynBindGroupLayout {}

dyn_type!(pub ref struct DynBindGroup(dyn BindGroupInterface));

impl BindGroupInterface for DynBindGroup {}

dyn_type!(pub ref struct DynTextureView(dyn TextureViewInterface));

impl TextureViewInterface for DynTextureView {}

dyn_type!(pub ref struct DynSampler(dyn SamplerInterface));

impl SamplerInterface for DynSampler {}

dyn_type!(pub ref struct DynBuffer(dyn BufferInterface));

impl BufferInterface for DynBuffer {
    fn map_async(
        &self,
        mode: crate::MapMode,
        range: Range<crate::BufferAddress>,
        callback: dispatch::BufferMapCallback,
    ) {
        self.0.map_async(mode, range, callback);
    }

    fn get_mapped_range(
        &self,
        sub_range: Range<crate::BufferAddress>,
    ) -> dispatch::DispatchBufferMappedRange {
        self.0.get_mapped_range(sub_range)
    }

    fn unmap(&self) {
        self.0.unmap();
    }

    fn destroy(&self) {
        self.0.destroy();
    }

    #[cfg(webgpu)]
    fn get_mapped_range_as_array_buffer(
        &self,
        _sub_range: Range<wgt::BufferAddress>,
    ) -> Option<js_sys::ArrayBuffer> {
        unimplemented!()
    }
}

dyn_type!(pub ref struct DynTexture(dyn TextureInterface));

impl TextureInterface for DynTexture {
    fn create_view(
        &self,
        desc: &crate::TextureViewDescriptor<'_>,
    ) -> dispatch::DispatchTextureView {
        self.0.create_view(desc)
    }

    fn destroy(&self) {
        self.0.destroy();
    }
}

dyn_type!(pub ref struct DynBlas(dyn BlasInterface));

impl BlasInterface for DynBlas {
    fn destroy(&self) {
        self.0.destroy();
    }
}

dyn_type!(pub ref struct DynTlas(dyn TlasInterface));

impl TlasInterface for DynTlas {
    fn destroy(&self) {
        self.0.destroy();
    }
}

dyn_type!(pub ref struct DynQuerySet(dyn QuerySetInterface));

impl QuerySetInterface for DynQuerySet {}

dyn_type!(pub ref struct DynPipelineLayout(dyn PipelineLayoutInterface));

impl PipelineLayoutInterface for DynPipelineLayout {}

dyn_type!(pub ref struct DynRenderPipeline(dyn RenderPipelineInterface));

impl RenderPipelineInterface for DynRenderPipeline {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout {
        self.0.get_bind_group_layout(index)
    }
}

dyn_type!(pub ref struct DynComputePipeline(dyn ComputePipelineInterface));

impl ComputePipelineInterface for DynComputePipeline {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout {
        self.0.get_bind_group_layout(index)
    }
}

dyn_type!(pub ref struct DynPipelineCache(dyn PipelineCacheInterface));

impl PipelineCacheInterface for DynPipelineCache {
    fn get_data(&self) -> Option<Vec<u8>> {
        self.0.get_data()
    }
}

dyn_type!(pub mut struct DynCommandEncoder(dyn CommandEncoderInterface));

impl CommandEncoderInterface for DynCommandEncoder {
    fn copy_buffer_to_buffer(
        &self,
        source: &DispatchBuffer,
        source_offset: crate::BufferAddress,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
        copy_size: crate::BufferAddress,
    ) {
        self.0.copy_buffer_to_buffer(
            source,
            source_offset,
            destination,
            destination_offset,
            copy_size,
        );
    }

    fn copy_buffer_to_texture(
        &self,
        source: crate::TexelCopyBufferInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.0
            .copy_buffer_to_texture(source, destination, copy_size);
    }

    fn copy_texture_to_buffer(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyBufferInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.0
            .copy_texture_to_buffer(source, destination, copy_size);
    }

    fn copy_texture_to_texture(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    ) {
        self.0
            .copy_texture_to_texture(source, destination, copy_size);
    }

    fn begin_compute_pass(
        &self,
        desc: &crate::ComputePassDescriptor<'_>,
    ) -> dispatch::DispatchComputePass {
        self.0.begin_compute_pass(desc)
    }

    fn begin_render_pass(
        &self,
        desc: &crate::RenderPassDescriptor<'_>,
    ) -> dispatch::DispatchRenderPass {
        self.0.begin_render_pass(desc)
    }

    fn finish(&mut self) -> DispatchCommandBuffer {
        Arc::get_mut(&mut self.0).unwrap().finish()
    }

    fn clear_texture(
        &self,
        texture: &DispatchTexture,
        subresource_range: &crate::ImageSubresourceRange,
    ) {
        self.0.clear_texture(texture, subresource_range);
    }

    fn clear_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferAddress>,
    ) {
        self.0.clear_buffer(buffer, offset, size);
    }

    fn insert_debug_marker(&self, label: &str) {
        self.0.insert_debug_marker(label);
    }

    fn push_debug_group(&self, label: &str) {
        self.0.push_debug_group(label);
    }

    fn pop_debug_group(&self) {
        self.0.pop_debug_group();
    }

    fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32) {
        self.0.write_timestamp(query_set, query_index);
    }

    fn resolve_query_set(
        &self,
        query_set: &DispatchQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
    ) {
        self.0.resolve_query_set(
            query_set,
            first_query,
            query_count,
            destination,
            destination_offset,
        );
    }

    fn build_acceleration_structures_unsafe_tlas<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        tlas: &mut dyn Iterator<Item = &'a crate::TlasBuildEntry<'a>>,
    ) {
        self.0.build_acceleration_structures_unsafe_tlas(blas, tlas);
    }

    fn build_acceleration_structures<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        tlas: &mut dyn Iterator<Item = &'a crate::TlasPackage>,
    ) {
        self.0.build_acceleration_structures(blas, tlas);
    }
}

dyn_type!(pub mut struct DynComputePass(dyn ComputePassInterface));

impl ComputePassInterface for DynComputePass {
    fn set_pipeline(&mut self, pipeline: &DispatchComputePipeline) {
        Arc::get_mut(&mut self.0).unwrap().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_bind_group(index, bind_group, offsets);
    }

    fn set_push_constants(&mut self, offset: u32, data: &[u8]) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_push_constants(offset, data);
    }

    fn insert_debug_marker(&mut self, label: &str) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .insert_debug_marker(label);
    }

    fn push_debug_group(&mut self, group_label: &str) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .push_debug_group(group_label);
    }

    fn pop_debug_group(&mut self) {
        Arc::get_mut(&mut self.0).unwrap().pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .write_timestamp(query_set, query_index);
    }

    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .begin_pipeline_statistics_query(query_set, query_index);
    }

    fn end_pipeline_statistics_query(&mut self) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .end_pipeline_statistics_query();
    }

    fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .dispatch_workgroups(x, y, z);
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .dispatch_workgroups_indirect(indirect_buffer, indirect_offset);
    }

    fn end(&mut self) {
        Arc::get_mut(&mut self.0).unwrap().end()
    }
}

dyn_type!(pub mut struct DynRenderPass(dyn RenderPassInterface));

impl RenderPassInterface for DynRenderPass {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        Arc::get_mut(&mut self.0).unwrap().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_bind_group(index, bind_group, offsets);
    }

    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_index_buffer(buffer, index_format, offset, size);
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_vertex_buffer(slot, buffer, offset, size);
    }

    fn set_push_constants(&mut self, stages: crate::ShaderStages, offset: u32, data: &[u8]) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_push_constants(stages, offset, data);
    }

    fn set_blend_constant(&mut self, color: crate::Color) {
        Arc::get_mut(&mut self.0).unwrap().set_blend_constant(color);
    }

    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_scissor_rect(x, y, width, height);
    }

    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    fn set_stencil_reference(&mut self, reference: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_stencil_reference(reference);
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        Arc::get_mut(&mut self.0).unwrap().draw(vertices, instances);
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indexed(indices, base_vertex, instances);
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indirect(indirect_buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indexed_indirect(indirect_buffer, indirect_offset);
    }

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    ) {
        Arc::get_mut(&mut self.0).unwrap().multi_draw_indirect(
            indirect_buffer,
            indirect_offset,
            count,
        );
    }

    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .multi_draw_indexed_indirect(indirect_buffer, indirect_offset, count);
    }

    fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .multi_draw_indirect_count(
                indirect_buffer,
                indirect_offset,
                count_buffer,
                count_buffer_offset,
                max_count,
            );
    }

    fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .multi_draw_indexed_indirect_count(
                indirect_buffer,
                indirect_offset,
                count_buffer,
                count_buffer_offset,
                max_count,
            );
    }

    fn insert_debug_marker(&mut self, label: &str) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .insert_debug_marker(label);
    }

    fn push_debug_group(&mut self, group_label: &str) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .push_debug_group(group_label);
    }

    fn pop_debug_group(&mut self) {
        Arc::get_mut(&mut self.0).unwrap().pop_debug_group();
    }

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .write_timestamp(query_set, query_index);
    }

    fn begin_occlusion_query(&mut self, query_index: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .begin_occlusion_query(query_index);
    }

    fn end_occlusion_query(&mut self) {
        Arc::get_mut(&mut self.0).unwrap().end_occlusion_query();
    }

    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .begin_pipeline_statistics_query(query_set, query_index);
    }

    fn end_pipeline_statistics_query(&mut self) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .end_pipeline_statistics_query();
    }

    fn execute_bundles(
        &mut self,
        render_bundles: &mut dyn Iterator<Item = &dispatch::DispatchRenderBundle>,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .execute_bundles(render_bundles);
    }

    fn end(&mut self) {
        Arc::get_mut(&mut self.0).unwrap().end();
    }
}

dyn_type!(pub ref struct DynCommandBuffer(dyn CommandBufferInterface));

impl CommandBufferInterface for DynCommandBuffer {}

dyn_type!(pub mut struct DynRenderBundleEncoder(dyn RenderBundleEncoderInterface));

impl RenderBundleEncoderInterface for DynRenderBundleEncoder {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline) {
        Arc::get_mut(&mut self.0).unwrap().set_pipeline(pipeline);
    }

    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_bind_group(index, bind_group, offsets);
    }

    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_index_buffer(buffer, index_format, offset, size);
    }

    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_vertex_buffer(slot, buffer, offset, size);
    }

    fn set_push_constants(&mut self, stages: crate::ShaderStages, offset: u32, data: &[u8]) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .set_push_constants(stages, offset, data);
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        Arc::get_mut(&mut self.0).unwrap().draw(vertices, instances);
    }

    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indexed(indices, base_vertex, instances);
    }

    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indirect(indirect_buffer, indirect_offset);
    }

    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    ) {
        Arc::get_mut(&mut self.0)
            .unwrap()
            .draw_indexed_indirect(indirect_buffer, indirect_offset);
    }

    fn finish(self, _desc: &crate::RenderBundleDescriptor<'_>) -> dispatch::DispatchRenderBundle
    where
        Self: Sized,
    {
        unimplemented!("RenderBundleEncoderInterface is cannot dyn dispatch finish")
    }
}

dyn_type!(pub ref struct DynRenderBundle(dyn RenderBundleInterface));

impl RenderBundleInterface for DynRenderBundle {}

dyn_type!(pub ref struct DynSurface(dyn SurfaceInterface));

impl SurfaceInterface for DynSurface {
    fn get_capabilities(&self, adapter: &dispatch::DispatchAdapter) -> wgt::SurfaceCapabilities {
        self.0.get_capabilities(adapter)
    }

    fn configure(&self, device: &dispatch::DispatchDevice, config: &crate::SurfaceConfiguration) {
        self.0.configure(device, config);
    }

    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        crate::SurfaceStatus,
        dispatch::DispatchSurfaceOutputDetail,
    ) {
        self.0.get_current_texture()
    }
}

dyn_type!(pub ref struct DynSurfaceOutputDetail(dyn SurfaceOutputDetailInterface));

impl SurfaceOutputDetailInterface for DynSurfaceOutputDetail {
    fn present(&self) {
        self.0.present();
    }

    fn texture_discard(&self) {
        self.0.texture_discard();
    }
}

dyn_type!(pub mut struct DynQueueWriteBuffer(dyn QueueWriteBufferInterface));

impl QueueWriteBufferInterface for DynQueueWriteBuffer {
    fn slice(&self) -> &[u8] {
        self.0.slice()
    }

    fn slice_mut(&mut self) -> &mut [u8] {
        Arc::get_mut(&mut self.0).unwrap().slice_mut()
    }
}

dyn_type!(pub mut struct DynBufferMappedRange(dyn BufferMappedRangeInterface));

impl BufferMappedRangeInterface for DynBufferMappedRange {
    fn slice(&self) -> &[u8] {
        self.0.slice()
    }

    fn slice_mut(&mut self) -> &mut [u8] {
        Arc::get_mut(&mut self.0).unwrap().slice_mut()
    }
}

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
