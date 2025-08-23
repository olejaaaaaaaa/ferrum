

use std::env;

use winit::error::EventLoopError;
use winit::event_loop::EventLoopWindowTarget;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::{self, Window, WindowBuilder};
use winit::event::WindowEvent;

use super::EditorArgs;

pub type EventHandler<T> = Box<dyn FnMut(&Event<T>, &EventLoopWindowTarget<T>)>;

///
/// # Main Editor for Ferrum Engine
///
pub struct Editor<T: 'static> {
    /// Main game loop
    event_loop: EventLoop<T>,
    /// Window
    window: Window,
    /// Event handlers game logics
    event_handlers: Vec<EventHandler<T>>
}

impl<T> Editor<T> {

    /// Create Editor
    pub fn new(event_loop: EventLoop<T>, args: EditorArgs) -> Self {

        let window = Self::build_window(&event_loop, &args);

        Self {
            event_loop,
            window,
            event_handlers: Vec::new()
        }
    }

    /// Add custom event handler
    pub fn add_event_handler(&mut self, handler: EventHandler<T>) {
        self.event_handlers.push(handler);
    }

    fn create_drag_drop_handler() -> EventHandler<T> {
        Box::new(move |event, target| {
            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::DroppedFile(path) = event {
                    println!("File dropped: {:?}", path);
                }
            }
        })
    }

    /// Event handler for Close Request
    fn default_event_handler() -> EventHandler<T> {

        Box::new(move |event, event_loop_target| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        event_loop_target.exit();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == winit::event::ElementState::Pressed {
                            match event.logical_key {
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                                    event_loop_target.exit();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        })
    }

    /// Build Default Window with size: 600x480
    fn build_window(event_loop: &EventLoop<T>, args: &EditorArgs) -> winit::window::Window {

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
        self.event_handlers.push(Self::create_drag_drop_handler());

        let path = env::args().collect::<Vec<String>>();

        self.event_loop.run(move |event, target| {
            for handler in &mut self.event_handlers {
                handler(&event, target);
            }
        })
    }
}