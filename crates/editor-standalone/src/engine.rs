use ferrum_render::RenderContext;
use ferrum_graph::RenderGraph;
use crate::editor::EventHandler;
use winit::event::WindowEvent;
use winit::event::Event;
pub struct Engine<T: 'static> {
    /// Context wrapper
    pub ctx: RenderContext,
    /// Graph
    pub graph: RenderGraph,
    /// Engine Handlers
    pub event_handlers: Vec<EventHandler<T>>,
}


impl<T: 'static> Engine<T> {
    pub fn new(ctx: RenderContext, graph: RenderGraph) -> Self {

        let drag_drop_handler = Self::create_drag_drop_handler();

        Engine { ctx, graph, event_handlers: vec![drag_drop_handler] }
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

}

