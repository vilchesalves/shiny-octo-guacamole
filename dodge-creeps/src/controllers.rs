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

fn get_direction(input: &Input) -> Vector2 {
    let mut direction = Vector2::new(0.0, 0.0);

    if input.is_action_pressed(GodotString::from_str("ui_up")) {
        direction.y -= 1.0;
    }

    if input.is_action_pressed(GodotString::from_str("ui_right")) {
        direction.x += 1.0;
    }

    if input.is_action_pressed(GodotString::from_str("ui_down")) {
        direction.y += 1.0;
    }

    if input.is_action_pressed(GodotString::from_str("ui_left")) {
        direction.x -= 1.0;
    }

    match direction.length() == 0.0 {
        false => direction.normalize(),
        true => direction,
    }
}

fn move_owner(owner: &Area2D, direction_delta: Vector2, limit: Size2) {
    let global_position = owner.global_position();
    let position = global_position + direction_delta;

    owner.set_global_position(Vector2::new(
        position.x.max(0.0).min(limit.width),
        position.y.max(0.0).min(limit.height),
    ));
}

#[methods]
impl PlayerController {
    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        self.screen_size = Some(owner.get_viewport_rect().size);
    }

    #[export]
    fn _process(&self, owner: &Area2D, delta: f32) {
        let direction = get_direction(Input::godot_singleton());

        move_owner(
            owner,
            direction * self.speed * delta,
            self.screen_size.expect("screen size not defined"),
        );
    }
}
