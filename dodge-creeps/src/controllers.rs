use gdnative::prelude::*;
use gdnative::api::Area2D;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct PlayerController;

impl PlayerController {
    fn new(_owner: &Area2D) -> Self {
        Self
    }
}

#[methods]
impl PlayerController {
    #[export]
    fn _ready(&self, _owner: TRef<Area2D>) {
        
    }
}
