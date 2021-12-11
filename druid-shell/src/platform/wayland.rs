pub mod xdg {
    pub mod popup {
        pub use wayland_protocols::xdg_shell::client::xdg_positioner::Anchor;
        pub use wayland_protocols::xdg_shell::client::xdg_positioner::ConstraintAdjustment;
        pub use wayland_protocols::xdg_shell::client::xdg_positioner::Gravity;
    }
}

pub mod layershell {
    use crate::application::Application;
    use crate::backend::wayland::surfaces::layershell::Margin;
    use crate::backend::wayland::window as backend;
    use crate::error::Error;
    use crate::window;

    pub use crate::backend::wayland::surfaces::layershell::Config;
    pub use wayland_protocols::wlr::unstable::layer_shell::v1::client::zwlr_layer_shell_v1::Layer;
    pub use wayland_protocols::wlr::unstable::layer_shell::v1::client::zwlr_layer_surface_v1::Anchor;
    pub use wayland_protocols::wlr::unstable::layer_shell::v1::client::zwlr_layer_surface_v1::KeyboardInteractivity;

    /// A builder type for creating new layershells.
    pub struct Builder(backend::layershell::Builder);

    impl Builder {
        /// Create a new `WindowBuilder`.
        ///
        /// Takes the [`Application`](crate::Application) that this window is for.
        pub fn new(app: Application) -> Builder {
            Builder(backend::layershell::Builder::new(app.backend_app))
        }

        /// Set the [`WinHandler`] for this window.
        ///
        /// This is the object that will receive callbacks from this window.
        pub fn set_handler(mut self, handler: Box<dyn window::WinHandler>) -> Self {
            self.0.set_handler(handler);
            self
        }

        pub fn layer(mut self, layer: Layer) -> Self {
            self.0.config.layer = layer;
            self
        }

        pub fn keyboard_interactivity(mut self, mode: KeyboardInteractivity) -> Self {
            self.0.config.keyboard_interactivity = mode;
            self
        }

        pub fn anchor(mut self, anchor: Anchor) -> Self {
            self.0.config.anchor = anchor;
            self
        }

        pub fn exclusive_zone(mut self, n: i32) -> Self {
            self.0.config.exclusive_zone = n;
            self
        }

        pub fn size(mut self, dim: impl Into<kurbo::Size>) -> Self {
            self.0.config.initial_size = dim.into();
            self
        }

        pub fn margin(mut self, margin: impl Into<Margin>) -> Self {
            self.0.config.margin = margin.into();
            self
        }

        /// Attempt to construct the platform window.
        ///
        /// If this fails, your application should exit.
        pub fn build(self) -> Result<window::WindowHandle, Error> {
            self.0.build().map(window::WindowHandle).map_err(Into::into)
        }
    }
}
