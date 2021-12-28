use gdnative::prelude::*;

mod hello_world;
mod panel_controller;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<hello_world::HelloWorld>();
    handle.add_class::<panel_controller::PanelController>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
