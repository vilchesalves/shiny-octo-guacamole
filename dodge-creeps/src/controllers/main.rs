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

    let target = unsafe {
        owner.get_node_as::<Node>(".")
        .expect("couldn't get target")
    };
    
    player.connect(
        "hit",
        target,
        "game_over",
        VariantArray::new_shared(),
        0,
    ).expect("couldn't connect hit");
}

#[methods]
impl Main {
    #[export]
    fn _ready(&self, owner: &Node) {
        connect_player(owner);
    }
}
