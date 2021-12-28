mod controllers;

use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<controllers::HelloWorld>();
    handle.add_class::<controllers::PanelController>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
