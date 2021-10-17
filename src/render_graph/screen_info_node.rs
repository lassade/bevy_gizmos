// TODO: Move rendering and pipeline stuff here

use bevy::{
    app::{Events, ManualEventReader},
    ecs::system::BoxedSystem,
    prelude::*,
    render::{
        render_graph::{CommandQueue, Node, ResourceSlots, SystemNode},
        renderer::{
            BufferId, BufferInfo, BufferMapMode, BufferUsage, RenderContext, RenderResource,
            RenderResourceBinding, RenderResourceBindings, RenderResourceContext,
        },
    },
    window::{WindowCreated, WindowId, WindowResized},
};

pub const SCREEN_INFO_NODE: &str = "screen_info";
pub const SCREEN_INFO_UNIFORM: &str = "ScreenInfo";

#[derive(Default)]
pub struct ScreenInfoNode {
    command_queue: CommandQueue,
}

impl Node for ScreenInfoNode {
    fn update(
        &mut self,
        _world: &World,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        self.command_queue.execute(render_context);
    }
}

impl SystemNode for ScreenInfoNode {
    fn get_system(&self) -> BoxedSystem {
        let system = lights_node_system.system().config(|config| {
            config.0 = Some(ScreenInfoState {
                screen_info_buffer: None,
                staging_buffer: None,
                command_queue: self.command_queue.clone(),
                ..Default::default()
            })
        });
        Box::new(system)
    }
}

#[derive(Default)]
pub struct ScreenInfoState {
    window_id: WindowId,
    window_created_event_reader: ManualEventReader<WindowCreated>,
    window_resized_event_reader: ManualEventReader<WindowResized>,
    screen_info_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
    command_queue: CommandQueue,
}

pub fn lights_node_system(
    mut state: Local<ScreenInfoState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    window_created_events: Res<Events<WindowCreated>>,
    window_resized_events: Res<Events<WindowResized>>,
    windows: Res<Windows>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel
    // with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
) {
    const BUFFER_SIZE: usize = std::mem::size_of::<[f32; 4]>();

    let state = &mut state;
    let window = windows.get(state.window_id).unwrap();

    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
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
        state.screen_info_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size: BUFFER_SIZE,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            ..Default::default()
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    if state
        .window_created_event_reader
        .iter(&window_created_events)
        .any(|e| e.id == window.id())
        || state
            .window_resized_event_reader
            .iter(&window_resized_events)
            .any(|e| e.id == window.id())
    {
        let w = window.physical_width() as f32;
        let h = window.physical_height() as f32;
        let aspect = w / h;
        let screen_info: [f32; 4] = [w, h, 1.0 / aspect, aspect];

        render_resource_context.map_buffer(staging_buffer, BufferMapMode::Write);
        render_resource_context.write_mapped_buffer(
            staging_buffer,
            0..BUFFER_SIZE as u64,
            &mut |data, _renderer| {
                screen_info.write_buffer_bytes(&mut data[0..BUFFER_SIZE]);
            },
        );
        render_resource_context.unmap_buffer(staging_buffer);

        let screen_info_buffer = state.screen_info_buffer.unwrap();
        state.command_queue.copy_buffer_to_buffer(
            staging_buffer,
            0,
            screen_info_buffer,
            0,
            BUFFER_SIZE as u64,
        );
    }
}
