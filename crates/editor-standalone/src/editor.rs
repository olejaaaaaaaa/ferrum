use ferrum_graph::RenderGraph;
use ferrum_render::RenderContext;

use winit::error::EventLoopError;
use winit::event_loop::EventLoopWindowTarget;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::{self, Window, WindowBuilder};
use winit::event::WindowEvent;
use crate::engine::Engine;

use super::EditorArgs;
pub type EventHandler<T> = Box<dyn FnMut(&Event<T>, &EventLoopWindowTarget<T>)>;

///
/// # Main Editor for Ferrum Engine
///
pub struct Editor<T: 'static> {
    /// Main game loop
    main_loop: EventLoop<T>,
    /// Event handlers game logics
    event_handlers: Vec<EventHandler<T>>,
    /// Engine
    engine: Engine<T>

}

impl<T> Editor<T> {

    /// Create Editor
    pub fn new(event_loop: EventLoop<T>, mut args: EditorArgs) -> Self {

        let window = Self::build_window(&event_loop, &mut args);
        let ctx = RenderContext::default(window);
        let graph = RenderGraph::new();
        let engine = Engine::<T>::new(ctx, graph);

        Self {
            main_loop: event_loop,
            engine,
            event_handlers: Vec::new()
        }
    }

    /// Add custom event handler
    pub fn add_event_handler(&mut self, handler: EventHandler<T>) {
        self.event_handlers.push(handler);
    }

    /// Event handler for Close Request
    fn default_event_handler() -> EventHandler<T> {

        Box::new(move |event, event_loop_target| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        event_loop_target.exit();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
    }

    /// Build Default Window with size: 600x480
    fn build_window(event_loop: &EventLoop<T>, args: &mut EditorArgs) -> winit::window::Window {

        args.height = if (args.height) == 0 { 480 } else { args.height };
        args.width = if (args.width) == 0 { 600 } else { args.width };

        let window = WindowBuilder::new()
            .with_title("Ferrum Editor")
            .with_inner_size(winit::dpi::PhysicalSize{ width: args.width, height: args.height })
            .build(&event_loop)
            .expect("Error create window");

        window
    }

    /// Run Editor
    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {

        self.event_handlers.push(Self::default_event_handler());
        self.event_handlers.extend(self.engine.event_handlers);

        self.main_loop.run(move |event, target| {

            for handler in &mut self.event_handlers {
                handler(&event, target);
            }

            if let Event::AboutToWait = event {
                self.engine.ctx.window.raw.request_redraw();
            }
        })
    }
}