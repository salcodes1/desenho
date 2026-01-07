use crate::{
    backends::{self, PresentBackend},
    vulkan::{self},
};
use std::sync::Arc;
use winit::{application::ApplicationHandler, window::WindowAttributes};

pub(crate) struct WinitBackend {
    pub(crate) winit_app: WinitApp,
    pub(crate) winit_loop: winit::event_loop::EventLoop<()>,
}

pub(crate) struct WinitApp {
    vulkan_renderer: Arc<vulkan::VulkanRenderer>,
    window: Option<Arc<winit::window::Window>>,
    surface_present: Option<backends::surface::SurfacePresent>,
}

impl WinitBackend {
    pub fn new(vulkan_renderer: Arc<vulkan::VulkanRenderer>) -> Self {
        let winit_app = WinitApp::new(vulkan_renderer);
        let winit_loop = winit::event_loop::EventLoop::new().unwrap();

        WinitBackend {
            winit_app,
            winit_loop,
        }
    }
}

impl WinitApp {
    fn new(vulkan_renderer: Arc<vulkan::VulkanRenderer>) -> Self {
        WinitApp {
            vulkan_renderer: vulkan_renderer,
            window: None,
            surface_present: None,
        }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                    // renderer.empty_frame();
                    // self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

impl PresentBackend for WinitBackend {
    fn acquire_frame(&mut self) -> anyhow::Result<super::AcquiredFrame> {
        self.winit_app.acquire_frame()
        // return self.winit_app.window.as_ref().unwrap().inner_size().into();
    }
}

impl PresentBackend for WinitApp {
    fn acquire_frame(&mut self) -> anyhow::Result<super::AcquiredFrame> {
        self.surface_present
            .as_mut()
            .unwrap()
            .acquire_frame()
    }
}

