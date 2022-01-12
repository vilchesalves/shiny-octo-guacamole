use gdnative::{
    api::{AnimatedSprite, RigidBody2D},
    prelude::*,
};
use rand::Rng;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Mob {
    #[property(default = 150.0)]
    pub min_speed: f32,
    #[property(default = 250.0)]
    pub max_speed: f32,
}

impl Mob {
    fn new(_owner: &RigidBody2D) -> Self {
        Self {
            min_speed: 150.0,
            max_speed: 250.0,
        }
    }
}

fn randomize_sprite(animated_sprite: &AnimatedSprite) {
    let mut rng = rand::thread_rng();
    let available_animations = unsafe {
        animated_sprite
            .sprite_frames()
            .expect("sprite frames failed")
            .assume_safe()
            .get_animation_names()
    };
    let chosen_animation =
        available_animations.get(rng.gen_range(0..available_animations.len()));
    animated_sprite.set_animation(chosen_animation);
}

#[methods]
impl Mob {
    #[export]
    fn _ready(&self, owner: &RigidBody2D) {
        randomize_sprite(unsafe {
            owner
                .get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("couldn't get sprite")
                .as_ref()
        });
    }

    #[export]
    fn _on_visibility_notifier_2d_screen_exited(&self, owner: &RigidBody2D) {
        owner.queue_free();
    }
}
