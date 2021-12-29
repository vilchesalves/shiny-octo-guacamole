use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct TimerExample;

fn get_cast<'a, T>(owner: TRef<'a, Node2D>, path: &str) -> TRef<'a, T>
where
    T: SubClass<Node>,
{
    let node = owner.get_node(path).expect("couldn't get node");
    let node = unsafe { node.assume_safe() };
    let node = node.cast::<T>().expect("couldn't cast");

    node
}

#[methods]
impl TimerExample {
    fn new(_owner: &Node2D) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        let timer = get_cast::<Timer>(owner, "Timer");

        timer
            .connect(
                "timeout",
                owner,
                "_on_timer_timeout",
                VariantArray::new_shared(),
                0,
            )
            .expect("couldn't connect");

        godot_print!("Ready!");
    }

    #[export]
    fn _on_timer_timeout(&self, owner: TRef<Node2D>) {
        let sprite = get_cast::<Sprite>(owner, "Sprite");

        let is_visible = sprite.is_visible();

        sprite.set_visible(!is_visible);

        godot_print!("Timedout!");
    }
}
