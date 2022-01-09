use std::f64::consts::PI;

use gdnative::{
    api::{PathFollow2D, Position2D, RigidBody2D},
    prelude::*,
};
use rand::Rng;

use super::PlayerController;

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
}

fn connect_internal_scenes(owner: &Node) {
    let player = unsafe {
        owner
            .get_node_as::<Node2D>("Player")
            .expect("couldn't get player node")
    };

    let target = unsafe { owner.get_node_as::<Node>(".").expect("couldn't get target") };

    player
        .connect("hit", target, "game_over", VariantArray::new_shared(), 0)
        .expect("couldn't connect hit");

    let start_timer = unsafe {
        owner
            .get_node_as::<Timer>("StartTimer")
            .expect("couldn't find start timer")
    };

    start_timer
        .connect(
            "timeout",
            target,
            "_on_starttimer_timeout",
            VariantArray::new_shared(),
            0,
        )
        .expect("couldn't connect timeout");

    let score_timer = unsafe {
        owner
            .get_node_as::<Timer>("ScoreTimer")
            .expect("couldn't get score timer")
    };

    score_timer
        .connect(
            "timeout",
            target,
            "_on_scoretimer_timeout",
            VariantArray::new_shared(),
            0,
        )
        .expect("couldn't connect timeout");

    let mob_timer = unsafe {
        owner
            .get_node_as::<Timer>("MobTimer")
            .expect("couldn't get mob timer")
    };

    mob_timer
        .connect(
            "timeout",
            target,
            "_on_mobtimer_timeout",
            VariantArray::new_shared(),
            0,
        )
        .expect("couldn't connect timeout");
}

#[methods]
impl Main {
    #[export]
    fn _ready(&mut self, owner: &Node) {
        connect_internal_scenes(owner);
        self.new_game(owner);
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        self.score = 0;

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
                c.start(&c_owner.as_ref(), start_position.position());
            })
            .ok()
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

        mob_spawn_location.set_offset(rand::random());

        // create mob instance
        let mob = unsafe { &self.mob.assume_safe() };
        let mob = mob
            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            .expect("couldn't instance mob.");
        let mob = unsafe { mob.assume_unique() }
            .cast::<RigidBody2D>()
            .expect("couldn't cast to RigidBody2D");

        // set mob momentum
        let mut rng = rand::thread_rng();
        mob.set_position(mob_spawn_location.position());
        mob.set_rotation(
            mob_spawn_location.rotation() // path rotation
                + PI / 2.0 // perpendicular
                + rng.gen_range((-PI / 4.0)..(PI / 4.0)), // randomized
        );
        mob.set_linear_velocity(Vector2::new(1.0, 0.0));

        // append mob to main scene
        owner.add_child(mob, false);
    }
}
