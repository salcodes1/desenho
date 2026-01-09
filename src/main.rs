use std::{
    cell::Cell,
    os::fd::{AsFd, FromRawFd, OwnedFd},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use calloop::{
    EventLoop, Interest, Mode, PostAction,
    generic::Generic,
    timer::{TimeoutAction, Timer},
};
use rustix::{event, fd::AsRawFd};
use vulkano::swapchain::Surface;
use vulkano::sync::GpuFuture;
use wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase;
use wayland_server::{
    ListeningSocket,
    backend::ClientData,
    protocol::{wl_compositor::WlCompositor, wl_shm::WlShm},
};
use winit::{
    application::ApplicationHandler,
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
    window::WindowAttributes,
};

use crate::{
    backends::{PresentBackend, winit::WinitBackend},
    vulkan::{VulkanRenderer, content::SurfaceContent},
};

mod backends;
mod geometry;
mod logging;
mod vulkan;
mod wayland;

static WAYLAND_SOCKET_NAME: &str = "desenho-1";

struct CompositorState {
    vulkan_renderer: Arc<VulkanRenderer>,
    winit_backend: WinitBackend,
    wayland_display: wayland::DisplayState,
    wayland_server: wayland_server::Display<wayland::DisplayState>,
}

fn main() {
    logging::init().unwrap();
    log::info!("Desenho initializing...");

    let vulkan_renderer = VulkanRenderer::new();

    let wayland_socket = ListeningSocket::bind(WAYLAND_SOCKET_NAME).unwrap();

    let mut state = CompositorState {
        vulkan_renderer: vulkan_renderer.clone(),
        winit_backend: WinitBackend::new(vulkan_renderer.clone()),
        wayland_display: wayland::DisplayState::new(),
        wayland_server: wayland_server::Display::<wayland::DisplayState>::new().unwrap(),
    };

    state
        .wayland_server
        .handle()
        .create_global::<wayland::DisplayState, WlCompositor, _>(4, ());
    state
        .wayland_server
        .handle()
        .create_global::<wayland::DisplayState, WlShm, _>(2, ());
    state
        .wayland_server
        .handle()
        .create_global::<wayland::DisplayState, XdgWmBase, _>(2, ());

    let mut event_loop: EventLoop<CompositorState> = calloop::EventLoop::try_new().unwrap();
    let loop_handle = event_loop.handle();

    loop_handle
        .insert_source(
            Timer::from_duration(Duration::from_millis(0)),
            move |_, _, state| {
                let status = state.winit_backend.winit_loop.pump_app_events(
                    Some(Duration::from_millis(0)),
                    &mut state.winit_backend.winit_app,
                );

                match status {
                    PumpStatus::Continue => TimeoutAction::ToDuration(Duration::from_millis(0)),
                    PumpStatus::Exit(_) => {
                        log::info!("Winit event loop exiting");
                        TimeoutAction::Drop
                    }
                }
            },
        )
        .unwrap();

    loop_handle
        .insert_source(
            Generic::new(wayland_socket.as_fd(), Interest::READ, Mode::Level),
            |_, _, state| {
                log::debug!("Wayland dispatch tick");
                if let Ok(Some(stream)) = wayland_socket.accept() {
                    state
                        .wayland_server
                        .handle()
                        .insert_client(stream, Arc::new(DummyClientData))
                        .unwrap();
                }
                Ok(PostAction::Continue)
            },
        )
        .unwrap();

    loop_handle
        .insert_source(
            Timer::from_duration(Duration::from_millis(0)),
            move |_, _, state| {
                state
                    .wayland_server
                    .dispatch_clients(&mut state.wayland_display)
                    .unwrap();
                state.wayland_server.flush_clients().unwrap();
                TimeoutAction::ToDuration(Duration::from_millis(0))
            },
        )
        .unwrap();

    loop_handle
        .insert_source(
            Timer::from_duration(Duration::from_millis(0)),
            move |_, _, state| {
                if let Some(surface_present) = &mut state.winit_backend.winit_app.surface_present {
                    let acquired_frame = surface_present.acquire_frame().unwrap();

                    let renderer = &state.vulkan_renderer;
                    let image = acquired_frame.view.clone();
                    let cmd_buffer = renderer.render_output_surfaces_commands(
                        image,
                        state.wayland_display.surfaces.values(),
                        &state.wayland_display
                    );

                    let future = acquired_frame
                        .ready
                        .then_execute(renderer.queue.clone(), cmd_buffer)
                        .unwrap();

                    let presented_future = (acquired_frame.present)(future.boxed());
                    presented_future
                        .then_signal_fence_and_flush()
                        .unwrap()
                        .cleanup_finished();
                }
                TimeoutAction::ToDuration(Duration::from_millis(0))
            },
        )
        .unwrap();

    event_loop.run(None, &mut state, |_| {}).unwrap();
}

struct DummyClientData;

impl ClientData for DummyClientData {}
