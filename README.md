# geode-rs

An effort to port the Geode client mod API to Rust (WIP).

## Crates

- [broma-rs](./crates/broma-rs) - an implementation of the Broma language parser in Rust.
- [stl-core](./crates/stl-core) - partial implementations of the 3 major C++ stdlibs (libcxx, msvc, gnustl)
- [geode-codegen](./crates/geode-codegen) - converts Broma definitions to Rust types (similar to Geode's codegen).
- [geode-macros](./crates/geode-macros) - proc macro implementations for geode-rs
- [geode-rs](./crates/geode-rs) - bindings for Cocos, FMOD, the Geode loader API and Tulip
- [geode-example](./crates/geode-example) - example of a Rust Geode mod, hooking regular functions and constructors, storing own data in modified classes, logging, cross-compiling for android32/android64/imac/m1 from Windows, rendering egui ui, calling safe geode loader apis
- [geode-egui](./crates/geode-egui) - crossplatform egui backend implemented in Cocos, similar to [gd-imgui-cocos](https://github.com/matcool/gd-imgui-cocos)
- [mac-universal](./crates/mac-universal) - used to merge intel and m1 mac binaries into one fat mach-o .dylib in geode-example (apple bullscheisse)

## TODO

- Safe cocos bindings
- Inherit cocos classes?
- android32 (untested, compiles), iOS, MacOS
- ~~Automatically generate loader bindings? (long shot)~~ semi-done?
