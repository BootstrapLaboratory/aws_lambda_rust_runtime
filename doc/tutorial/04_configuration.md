# 4. Configuration

My project already has a decent number of modules, but it’s missing a configuration setup. In this lesson, I’ll add one.

## Principles and Goals

- **Central Access**: A single point to access configuration.
- **Modular**: Each subsystem/feature is configured separately so that only the necessary configurations are loaded at any given time.
- **Easy to Manage / Extensible**: Configurations should be straightforward to update and expand.
- **Performance**: Ensure the configuration system does not become a bottleneck

Achieving all of these goals can be challenging, but I will give it a try. There is no single way to handle configuration in Rust, since it is a general-purpose language. I can recommend [Figment](https://docs.rs/figment/) and [Config](https://docs.rs/config/), which are used in popular Rust web frameworks like [Rocket](https://rocket.rs/guide/v0.5/configuration/).

However, I will not use these libraries right now. Instead, I will set up my own configuration module that remains compatible with libraries like `Figment` and `Config`.

## Configuration Module as a Central Place

All configurations will be stored in a separate subproject called `configuration` (of library type).

In the project root, run:

```bash
cd app
cargo new --lib configuration
```

`app/configuration/Cargo.toml`

```toml
[package]
name = "configuration"
version = "0.1.0"
edition = "2021"
```

`app/configuration/src/lib.rs`

```rust
pub mod dynamodb;
```

`app/configuration/src/dynamodb.rs`

```rust
use std::env;

/// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Optional custom endpoint for DynamoDB.
    pub endpoint: Option<String>,
}

/// Constructs the configuration from environment variables.
pub fn load_config() -> Config {
    let endpoint = env::var("DYNAMODB_ENDPOINT").ok();
    Config { endpoint }
}
```

I define the `Config` structure with an optional `endpoint` property. When running DynamoDB locally, this property should be set to `http://localhost:8000`. In a production AWS environment, the property will remain `None`, allowing the DynamoDB client to use its default configuration.

The `load_config()` function currently loads variables manually from the environment. Most likely, or maybe not, I will want to automate this process using a specialized library (such as `Figment` or `Config`). I want to demonstrate that everything can be customized and that specialized libraries are not always necessary. Even if I use these libraries, my approach to defining and handling configuration remains unchanged; only the body of `load_config()` would be "simplified" (at the potential cost of performance, which is up to you to decide).

For example, using `Figment` might look like this:

```rust
...
pub fn load_config() -> Config {
    use figment::{
        providers::{Env, Format, Json, Toml},
        Figment,
    };

    Figment::new()
        .merge(Toml::file("Config.toml"))
        .merge(Env::prefixed("DYNAMODB_"))
        .merge(Env::raw().only(&["RUST", "RUST_DOC"]))
        .join(Json::file("Config.json"))
        .extract()
        .expect("Cannot load config")
}
...
```

This approach requires an additional dependency, which I will cover in the next lesson with a separate code example. In this lesson, I will show that configuring a Rust project without external libraries is not only possible but sometimes even a better choice.

## Configuration Helper Macro (Optional)

Currently, the configuration should be loaded once at application startup. If you call `load_config()` a second time, it will execute the loading process again, which might be undesirable if the process is slow. To address this, I can save the result of the initial load in a global variable and return that value on subsequent accesses rather than reloading the configuration.

I will create a Rust macro so that I can reuse it across all my configuration modules and avoid code duplication.

In the project root, run:

```bash
cd app/libraries
cargo new --lib config_macro
```

`app/libraries/config_macro/Cargo.toml`

```toml
[package]
name = "config_macro"
version = "0.1.0"
edition = "2021"

[dependencies]
once_cell = { workspace = true }
```

`app/libraries/config_macro/src/lib.rs`

```rust
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
```

This macro exports a `get_config()` function, which panics if the configuration has not been initialized, and a `get_or_init_config()` function that loads the configuration if it has not already been set. I might use it someday — or perhaps never — but I implemented it just in case. It is compatible with Figment/Config since it addresses a different aspect of configuration without conflicting with those libraries.

**Usage:**

`app/configuration/Cargo.toml`

```toml
[dependencies]
# internal dependencies
config_macro = { path = "../libraries/config_macro" }
```

`app/configuration/src/dynamodb.rs`

```rust
...
/// add this to the end of file
use config_macro::define_global_config;
define_global_config!(Config, load_config);
```

## Summary

- **Centralized Access:** The configuration library provides a single access point for project configuration.
- **Modularity:** At the moment, only the DynamoDB configuration is added, but additional configurations can be easily incorporated.
- **Ease of Management / Extensibility:** Rust structures are used to clearly define configurations, and the system is compatible with popular configuration libraries such as `Figment` and `Config`.
- **Performance:** Full control over performance is achieved due to the flexibility of the approach.

**Project structure:**

```plain
├── app
│   ├── configuration             - configuration is here
│   │   ├── src
│   │   │   ├── dynamodb.rs       - modular configuration
│   │   │   ├── redis.rs
│   │   │   ...
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── functions                 - lambda functions
│   │   ├── function_one
│   │   ├── function_three
│   │   └── function_two
│   │       ...
│   └── libraries                 - helper/wrapper libraries
│       ├── config_macro
│       └── lambda_http_wrapper
└── Cargo.toml                    - workspace Cargo.toml
```

I will begin using this configuration library in my next lesson, where I connect my Lambda functions with a DynamoDB database. For now, this lesson is sufficient.

[Browse the code at Lesson 4 checkpoint](https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime/tree/lesson-4)
