use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(_handle: InitHandle) {
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
