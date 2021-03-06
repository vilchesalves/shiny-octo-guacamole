use gdnative::api::{AnimatedSprite, Area2D, CollisionShape2D, RigidBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register)]
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

    fn register(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hit",
            args: &[],
        });
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

fn move_owner(owner: &Area2D, direction_delta: &Vector2, limit: &Size2) {
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
    fn _ready(&mut self, owner: TRef<Area2D>) {
        self.screen_size = Some(owner.get_viewport_rect().size);

        owner
            .connect(
                "body_entered",
                owner,
                "_on_player_body_entered",
                VariantArray::new_shared(),
                0,
            )
            .expect("Failed to connect");
    }

    #[export]
    fn _process(&self, owner: &Area2D, delta: f32) {
        let direction = get_direction(Input::godot_singleton());

        self.animate_sprite(owner, &direction);

        move_owner(
            owner,
            &(direction * self.speed * delta),
            &self.screen_size.expect("screen size not defined"),
        );
    }

    #[export]
    fn _on_player_body_entered(&self, owner: &Area2D, _body: Ref<RigidBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);

        unsafe {
            owner
                .get_node_as::<CollisionShape2D>("CollisionShape2D")
                .expect("couldn't find node.")
        }
        .set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, owner: &Area2D, start_position: Vector2) {
        owner.set_global_position(start_position);
        owner.show();
        unsafe {
            owner
                .get_node_as::<CollisionShape2D>("CollisionShape2D")
                .expect("couldn't find node.")
        }
        .set_deferred("disabled", false);
    }

    fn animate_sprite(&self, owner: &Area2D, direction: &Vector2) {
        let sprite = unsafe { owner.get_node_as::<AnimatedSprite>("AnimatedSprite") };
        let sprite = sprite.expect("couldn't locate sprite");

        if direction.length() > 0.0 {
            sprite.set_animation(if direction.x == 0.0 { "up" } else { "walk" });
            sprite.play("", false);
        } else {
            sprite.stop();
        }

        sprite.set_flip_h(direction.x < 0.0);
        sprite.set_flip_v(direction.y > 0.0);
    }
}
