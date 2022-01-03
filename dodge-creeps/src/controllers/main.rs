use gdnative::api::Node;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main {
    #[property]
    mob: Ref<PackedScene>,
}

impl Main {
    fn new(_owner: &Node) -> Self {
        Self {
            mob: PackedScene::new().into_shared(),
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
    fn new_game(&self, owner: &Node) {}

    #[export]
    fn game_over(&self, owner: &Node) {
        let score_timer = unsafe {
            owner.get_node_as::<Timer>("ScoreTimer").expect("couldn't get score timer")
        };
        score_timer.stop();

        let mob_timer = unsafe {
            owner.get_node_as::<Timer>("MobTimer").expect("couldn't get mob timer")
        };
        mob_timer.stop();
    }
}
