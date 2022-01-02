use gdnative::prelude::*;
use gdnative::api::RigidBody2D;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Mob;

impl Mob {
    fn new(_owner: &RigidBody2D) -> Self {
        Self
    }
}

#[methods]
impl Mob {
    #[export]
    fn _ready(&self, _owner: TRef<RigidBody2D>) {
        godot_print!("A Mob is ready");
    }
}
