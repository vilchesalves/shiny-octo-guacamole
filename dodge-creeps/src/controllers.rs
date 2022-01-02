use gdnative::api::Area2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct PlayerController {
    screen_size: Option<Size2>,

    #[property(default = 400.0)]
    speed: f32,
}

impl PlayerController {
    fn new(_owner: &Area2D) -> Self {
        Self {
            screen_size: None,
            speed: 400.0,
        }
    }
}

#[methods]
impl PlayerController {
    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        self.screen_size = Some(owner.get_viewport_rect().size);
    }

    #[export]
    fn _process(&self, owner: &Area2D, delta: f32) {
        let input = Input::godot_singleton();

        let mut velocity = Vector2::new(0.0, 0.0);

        if input.is_action_pressed(GodotString::from_str("ui_up")) {
            velocity.y -= 1.0;
        }

        if input.is_action_pressed(GodotString::from_str("ui_right")) {
            velocity.x += 1.0;
        }

        if input.is_action_pressed(GodotString::from_str("ui_down")) {
            velocity.y += 1.0;
        }

        if input.is_action_pressed(GodotString::from_str("ui_left")) {
            velocity.x -= 1.0;
        }

        if velocity.length() == 0.0 {
            return;
        }

        velocity = velocity.normalize() * self.speed;

        let global_position = owner.global_position();
        let position_delta = velocity * delta;
        let position = global_position + position_delta;

        match self.screen_size {
            Some(s) => {
                owner.set_global_position(Vector2::new(
                    position.x.max(0.0).min(s.width),
                    position.y.max(0.0).min(s.height),
                ));
            }
            None => {
                godot_print!("screen is not defined");
            }
        }
    }
}
