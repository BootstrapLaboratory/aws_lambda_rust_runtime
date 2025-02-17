pub use once_cell;

#[macro_export]
macro_rules! define_global_config {
    ($config_type:ty, $load_fn:path) => {
        static GLOBAL_CONFIG: ::config_macro::once_cell::sync::OnceCell<$config_type> =
            ::config_macro::once_cell::sync::OnceCell::new();

        /// Retrieves a reference to the global configuration.
        ///
        /// Panics if the configuration has not been initialized.
        pub fn get_config() -> &'static $config_type {
            GLOBAL_CONFIG
                .get()
                .expect("Config not initialized; call get_or_init_config() at startup.")
        }

        /// Retrieves the global configuration, initializing it if necessary.
        ///
        /// Returns a reference to the global configuration.
        pub fn get_or_init_config() -> &'static $config_type {
            if GLOBAL_CONFIG.get().is_none() {
                let config = $load_fn();
                let _ = GLOBAL_CONFIG.set(config);
            }
            GLOBAL_CONFIG.get().expect("Config not initialized")
        }
    };
}
