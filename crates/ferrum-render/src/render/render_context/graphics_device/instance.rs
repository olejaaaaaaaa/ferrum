use std::ffi::CStr;

use winit::{
    raw_window_handle::HasDisplayHandle,
    window::Window
};

use crate::core::{
    App,
    InstanceBuilder,
    Instance
};

use super::{
    GraphicsDeviceBuilder,
    window::WithWindow
};

pub struct WithInstance<'n> {
    pub app: App<'n>,
    pub instance: Instance
}

impl<'n, 'w> GraphicsDeviceBuilder<WithWindow<'n, 'w>> {

    pub fn with_instance<F>(self, build_fn: F) -> GraphicsDeviceBuilder<WithInstance<'n>>
    where F: FnOnce(App<'n>, &'w Window) -> Instance {

        let instance = build_fn(self.state.app.clone(), self.state.window);

        GraphicsDeviceBuilder {
            state: WithInstance {
                app: self.state.app,
                instance: instance
            }
        }
    }

    pub fn with_default_instance(self) -> GraphicsDeviceBuilder<WithInstance<'n>> {

        self.with_instance(|app, window| {

            let raw_display_handle = window
                .display_handle()
                .expect("Error get display handle")
                .as_raw();

            let mut window_ext = ash_window::enumerate_required_extensions(raw_display_handle)
                .unwrap()
                .iter()
                .map(|&ptr| unsafe { CStr::from_ptr(ptr) })
                .collect::<Vec<_>>();

            #[cfg(debug_assertions)]
            window_ext.push(c"VK_EXT_debug_utils");

            InstanceBuilder::new()
                .with_entry(app.entry)
                .with_app_info(app.raw)
                .with_api_version(app.api_version)
                .with_layers(&[
                    #[cfg(debug_assertions)]
                    c"VK_LAYER_KHRONOS_validation"
                ])
                .with_extensions(&window_ext)
                .build()
                .expect("Error create Instance")
        })

    }
}