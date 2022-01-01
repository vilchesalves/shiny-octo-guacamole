use gdnative::prelude::*;
use gdnative::api::Area2D;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct PlayerController {
    #[property(default = 400.0)]
    speed: f32,
}

impl PlayerController {
    fn new(_owner: &Area2D) -> Self {
        Self {
            speed: 400.0
        }
    }
}

#[methods]
impl PlayerController {
    #[export]
    fn _ready(&self, _owner: TRef<Area2D>) {
        
    }
}
