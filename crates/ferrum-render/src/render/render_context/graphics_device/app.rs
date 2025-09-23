use super::{
    GraphicsDevice,
    GraphicsDeviceBuilder
};

use crate::core::{
    App,
    AppBuilder
};

pub struct WithApp<'n> {
    pub app: App<'n>,
}

impl<'n> GraphicsDeviceBuilder<()> {
    pub fn new() -> Self {
        Self {
            state: (),
        }
    }

    pub fn with_app<F>(self, build_fn: F) -> GraphicsDeviceBuilder<WithApp<'n>>
    where F: FnOnce() -> App<'n> {

        let app = build_fn();

        GraphicsDeviceBuilder {
            state: WithApp { app },
        }
    }

    pub fn with_default_app(self) -> GraphicsDeviceBuilder<WithApp<'n>> {
        self.with_app(|| {
            AppBuilder::default()
                .with_min_required_api_version(ash::vk::API_VERSION_1_0)
                .build()
                .expect("Error create App")
        })
    }
}