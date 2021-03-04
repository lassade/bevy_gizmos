// TODO: Move rendering and pipeline stuff here

use bevy::{
    app::ManualEventReader,
    core::AsBytes,
    prelude::*,
    render::{
        render_graph::{Node, ResourceSlots},
        renderer::{
            BufferId, BufferInfo, BufferMapMode, BufferUsage, RenderContext, RenderResourceBinding,
            RenderResourceBindings,
        },
    },
    window::{WindowCreated, WindowId, WindowResized},
};

pub const SCREEN_INFO_NODE: &str = "screen_info";
pub const SCREEN_INFO_UNIFORM: &str = "ScreenInfo";

#[derive(Default)]
pub struct ScreenInfoNode {
    window_id: WindowId,
    window_created_event_reader: ManualEventReader<WindowCreated>,
    window_resized_event_reader: ManualEventReader<WindowResized>,
    screen_info_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

impl Node for ScreenInfoNode {
    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        const BUFFER_SIZE: usize = std::mem::size_of::<[f32; 4]>();

        // Fetch resources
        let window_created_events = resources.get::<Events<WindowCreated>>().unwrap();
        let window_resized_events = resources.get::<Events<WindowResized>>().unwrap();
        let windows = resources.get::<Windows>().unwrap();
        let mut render_resource_bindings = resources.get_mut::<RenderResourceBindings>().unwrap();

        let window = windows.get(self.window_id).unwrap();

        let render_resource_context = render_context.resources_mut();

        let staging_buffer = if let Some(staging_buffer) = self.staging_buffer {
            staging_buffer
        } else {
            let buffer = render_resource_context.create_buffer(BufferInfo {
                size: BUFFER_SIZE,
                buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                ..Default::default()
            });
            render_resource_bindings.set(
                SCREEN_INFO_UNIFORM,
                RenderResourceBinding::Buffer {
                    buffer,
                    range: 0..BUFFER_SIZE as u64,
                    dynamic_index: None,
                },
            );
            self.screen_info_buffer = Some(buffer);

            let staging_buffer = render_resource_context.create_buffer(BufferInfo {
                size: BUFFER_SIZE,
                buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
                ..Default::default()
            });

            self.staging_buffer = Some(staging_buffer);
            staging_buffer
        };

        if self
            .window_created_event_reader
            .iter(&window_created_events)
            .any(|e| e.id == window.id())
            || self
                .window_resized_event_reader
                .iter(&window_resized_events)
                .any(|e| e.id == window.id())
        {
            let w = window.physical_width() as f32;
            let h = window.physical_height() as f32;
            let aspect = w / h;
            let screen_info: [f32; 4] = [w, h, aspect, 1.0 / aspect];

            render_resource_context.map_buffer(staging_buffer, BufferMapMode::Write);
            render_resource_context.write_mapped_buffer(
                staging_buffer,
                0..BUFFER_SIZE as u64,
                &mut |data, _renderer| {
                    data[0..BUFFER_SIZE].copy_from_slice(screen_info.as_bytes());
                },
            );
            render_resource_context.unmap_buffer(staging_buffer);

            let screen_info_buffer = self.screen_info_buffer.unwrap();
            render_context.copy_buffer_to_buffer(
                staging_buffer,
                0,
                screen_info_buffer,
                0,
                BUFFER_SIZE as u64,
            );
        }
    }
}
