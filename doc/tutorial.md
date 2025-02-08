# AWS Lambda with Rust runtime

After analyzing existing options to run on AWS Lambda, was decided to try out with Rust language. It seems to be easier to setup and provides about the best price per request value comparable with C/C++/Go.

## Tutorial plan

For each of the lesson I will try to make corresponding commit with name of lesson.

## Let's start

First you need to get RustUp and install Rust with Cargo.

Second step will be installing cargo-lambda:

```bash
cargo install cargo-lambda
```

## 1. Initial setup

Each of our Lambda function will be stored in separate Cargo module. They may have very different dependencies, and we don't want all of our functions to be bundled with dependencies they don't require. Yes, Cargo actually supports to have multiply binaries in one module and have different dependencies for eac of them, but configuring and managing it is quite difficult. Going with multiply modules is much more straightforward and have lots of benefits later on.

Then I'm going with following initial setup:

I will use:

- **Cargo workspaces** – to manage multiple Lambda functions within a single repository.
- **cargo-lambda** – the official AWS tool for building and locally testing Rust Lambda functions.
- **AWS Lambda Rust Runtime** – the official runtime for Rust Lambda.
- **cargo-watch** (or the built-in **cargo-lambda** watch) – to automatically rebuild on code changes and simplify local debugging.
- **DynamoDB Local** (if needed) – to work with DynamoDB without connecting to the cloud.

```bash
mkdir rust_cargo_lambda
cd rust_cargo_lambda/
git init

# Creating shared library
cargo new common_lib --lib

# and lets make three lambda functions
cargo new function_one --bin
cargo new function_two --bin
cargo new function_three --bin
```

When I did it myself, I forgot to initialize git repository in the beginning, and `cargo new ...` did it in each of functions subdirectories. If its happening for you, then remove `.git` in all of subdirectories.

`Cargo.toml`:

```tolm
[workspace]
members = [
    "function_one",
    "function_two",
    "function_three",
    "common_lib",     # you probably want to have more then one library ("common" - is bad naming, use something more descriptive. here I use it just as example)
]
```

`rust-toolchain.toml` (Optional):

```toml
[toolchain]
channel = "stable"
```

This is how it should look for you:

```css
rust_cargo_lambda/
├── Cargo.toml                # Workspace
├── rust-toolchain.toml       # Rust version (Optional, but can be useful)
├── .gitignore
├── common-lib/               # library
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── function-one/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── function-two/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── function-three/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

Remove all unnecessary files if they exist. I had `.gitignore` file in each submodule. Let's create one in root of project:

`.gitignore`:

```gitignore
target
**/*.rs.bk
**/*.log
Cargo.lock
```

Each function has "main.rs" as entry point. It is default in Cargo. I do not like this. Later on it will be hard to navigate between them to edit, when all functions are named "main.rs". Let's rename them.

Open each of the submodule's `Cargo.toml` and add there these lines:

```toml
...
[[bin]]
name = "function_one"
path = "src/function_one.rs"
...
```

For library it will be this (`common_lib/Cargo.toml`):

```toml
[lib]
name = "common_lib"
path = "src/common_lib.rs"
```

And the last thing that I do not like in my Hello world project, which was introduces already by me initially. Structure of directories is not perfect already. Everything is mixed and its hard to guess by structure where is what. Especially when there are `target`, `doc`, and `.vscode` directories, and you may add more here. Even for such a small project it is already a problem. So I will make refactoring on early stage not to have this problem anymore.

I create `app` directory and inside it two more directories: `functions`, `libraries`.

I think you got an idea. Choose your own names, if you don't like my ones. More important is that now it has quite straight representative structure:

```css
├── app
│   ├── functions
│   │   ├── function_one
│   │   │   ├── src
│   │   │   │   └── function_one.rs
│   │   │   └── Cargo.toml
│   │   ├── function_three
│   │   │   ├── src
│   │   │   │   └── function_three.rs
│   │   │   └── Cargo.toml
│   │   └── function_two
│   │       ├── src
│   │       │   └── function_two.rs
│   │       └── Cargo.toml
│   └── libraries
│       └── common_lib
│           ├── src
│           │   └── common_lib.rs
│           └── Cargo.toml
├── doc
│   └── tutorial.md
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── rust-toolchain.toml
```

Modify workspace `Cargo.toml` accordingly:

```toml
[workspace]
members = [
    # functions
    "app/functions/function_one",
    "app/functions/function_two",
    "app/functions/function_three",
    # libraries
    "app/libraries/common_lib",
]
```

You can already notice, that project can be build with `cargo build` in root workspace project and in each separate submodule. You can even run it: `cargo run --bin function_one`.

During build I get this warning:

```bash
...
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
```

Let's fix it. `Cargo.toml`:

```toml
[workspace]
resolver = "2"  # https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
...
```

Now it builds with no warnings and have pretty structure. I think that's enough for the first lesson.

[Browse the code at Lesson 1 checkpoint](https://github.com/BootstrapLaboratory/aws_lambda_rust_runtime/tree/lesson-1)
