use gdnative::{prelude::*, api::RigidBody2D};

use crate::controllers::Mob;

pub fn connect_internal_scenes(owner: &Node) {
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

pub fn get_mob_speeds(mob: Ref<RigidBody2D, Unique>) -> (Ref<RigidBody2D, Unique>, f32, f32) {
    let instance = mob.cast_instance::<Mob>().expect("couldn't cast instance");

    let (min_speed, max_speed) = instance
        .map(|m, _m_owner| (m.min_speed, m.max_speed))
        .expect("couldn't map over instance");

    (instance.into_base(), min_speed, max_speed)
}
