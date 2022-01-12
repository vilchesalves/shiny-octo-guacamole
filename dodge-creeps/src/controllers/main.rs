use std::f64::consts::PI;

use crate::controllers::PlayerController;
use gdnative::{
    api::{PathFollow2D, Position2D, RigidBody2D},
    prelude::*,
};
use rand::Rng;

mod services;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main {
    #[property]
    mob: Ref<PackedScene>,

    score: i64,
}

impl Main {
    fn new(_owner: &Node) -> Self {
        Self {
            mob: PackedScene::new().into_shared(),
            score: 0,
        }
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }
}

#[methods]
impl Main {
    #[export]
    fn _ready(&mut self, owner: &Node) {
        services::connect_internal_scenes(owner);
        self.new_game(owner);
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        self.reset_score();

        let player = unsafe {
            owner
                .get_node_as_instance::<PlayerController>("Player")
                .expect("couldn't find player")
        };
        let start_position = unsafe {
            owner
                .get_node_as::<Position2D>("StartPosition")
                .expect("couldn't find start position")
        };
        let start_timer = unsafe {
            owner
                .get_node_as::<Timer>("StartTimer")
                .expect("couldn't find start timer")
        };

        player
            .map(|c, c_owner| {
                c.start(c_owner.as_ref(), start_position.position());
            })
            .expect("couldn't find player class");

        start_timer.start(0.0);
    }

    #[export]
    fn game_over(&self, owner: &Node) {
        let score_timer = unsafe {
            owner
                .get_node_as::<Timer>("ScoreTimer")
                .expect("couldn't get score timer")
        };
        score_timer.stop();

        let mob_timer = unsafe {
            owner
                .get_node_as::<Timer>("MobTimer")
                .expect("couldn't get mob timer")
        };
        mob_timer.stop();
    }

    #[export]
    fn _on_starttimer_timeout(&self, owner: &Node) {
        let score_timer = unsafe {
            owner
                .get_node_as::<Timer>("ScoreTimer")
                .expect("couldn't get score timer")
        };
        score_timer.start(0.0);

        let mob_timer = unsafe {
            owner
                .get_node_as::<Timer>("MobTimer")
                .expect("couldn't get mob timer")
        };
        mob_timer.start(0.0);
    }

    #[export]
    fn _on_scoretimer_timeout(&mut self, _owner: &Node) {
        self.score += 1;
    }

    #[export]
    fn _on_mobtimer_timeout(&self, owner: &Node) {
        // set spawn offset
        let mob_spawn_location = unsafe {
            owner
                .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation")
                .expect("couldn't get spawn")
        };

        // create mob instance
        let mob = unsafe { &self.mob.assume_safe() };
        let mob = mob
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("couldn't instance mob.");
        let mob = unsafe { mob.assume_unique() }
            .cast::<RigidBody2D>()
            .expect("couldn't cast to RigidBody2D");
        let (mob, min_speed, max_speed) = services::get_mob_speeds(mob);

        // set mob momentum
        let mut rng = rand::thread_rng();
        mob_spawn_location.set_unit_offset(rng.gen());
        mob.set_position(mob_spawn_location.position());

        let rotation = mob_spawn_location.rotation() // path rotation
            + PI / 2.0 // perpendicular
            + rng.gen_range((-PI / 4.0)..(PI / 4.0)); // randomized

        mob.set_rotation(rotation);
        mob.set_linear_velocity(Vector2::from_angle_and_length(
            Angle::radians(rotation as f32),
            rng.gen_range(min_speed..max_speed),
        ));

        // append mob to main scene
        owner.add_child(mob, false);
    }
}
