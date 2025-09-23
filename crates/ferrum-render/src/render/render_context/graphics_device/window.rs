use crate::core::{
    App,
    AppBuilder
};

use super::{GraphicsDevice, GraphicsDeviceBuilder};
use super::app::WithApp;
use winit::window::Window;
pub struct WithWindow<'n, 'w> {
    pub app: App<'n>,
    pub window: &'w Window
}

impl<'n, 'w> GraphicsDeviceBuilder<WithApp<'n>> {

    pub fn with_window(self, window: &'w Window) -> GraphicsDeviceBuilder<WithWindow<'n, 'w>> {
        GraphicsDeviceBuilder {
            state: WithWindow {
                app: self.state.app,
                window
            },
        }
    }
}