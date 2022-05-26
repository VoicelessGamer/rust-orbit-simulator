use gdnative::prelude::*;

mod n_body_simulation;
mod math;
mod godot_integration;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<godot_integration::n_body_system_godot::NBodySystem>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);