# geode-rs

An effort to port the Geode client mod API to Rust (WIP).

## Crates

- [broma-rs](./crates/broma-rs) - an implementation of the Broma language parser in Rust.
- [geode-codegen](./crates/geode-codegen) - converts Broma definitions to Rust types (similar to Geode's codegen).
- [geode-macros](./crates/geode-macros) - proc macro implementations for geode-rs
- [geode-rs](./crates/geode-rs) - bindings for Cocos, the Geode loader API and Tulip
- [geode-example](./crates/geode-example) - example of a Rust Geode mod, hooking regular functions and constructors, storing own data in modified classes, logging

## TODO

- Currently this will only work on Windows, mainly because of geode-rs's loader bindings and the C++ stdlib shim. This is fixable, just needs some effort (add shims of other C++ stdlibs and use native platform API for getting loader functions, kinda like in base.rs; ensure cocos bindgen (geode-rs/build.rs) compiles on other platforms)
- Probably more? The `#[modify]` can likely be improved upon, like making `*mut PlayLayer` be `&mut PlayLayer`, or making calls to original functions safe.
