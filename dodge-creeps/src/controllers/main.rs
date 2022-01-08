use gdnative::{
    api::{PathFollow2D, Position2D},
    prelude::*,
};

use crate::controllers::player;

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

fn connect_player(owner: &Node) {
    let player = unsafe {
        owner
            .get_node_as::<Node2D>("Player")
            .expect("couldn't get player node")
    };

    let target = unsafe { owner.get_node_as::<Node>(".").expect("couldn't get target") };

    player
        .connect("hit", target, "game_over", VariantArray::new_shared(), 0)
        .expect("couldn't connect hit");
}

#[methods]
impl Main {
    #[export]
    fn _ready(&self, owner: &Node) {
        connect_player(owner);
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        self.score = 0;

        let player = unsafe {
            owner
                .get_node_as_instance::<player::PlayerController>("Player")
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
        let spawn = unsafe {
            owner
                .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation")
                .expect("couldn't get spawn")
        };

        spawn.set_offset(rand::random());
    }
}
