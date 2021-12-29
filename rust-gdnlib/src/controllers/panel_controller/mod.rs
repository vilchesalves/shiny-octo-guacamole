use gdnative::api::Panel;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Panel)]
pub struct PanelController {
    frames: u32,
    phys: u32,
}

#[methods]
impl PanelController {
    fn new(_owner: &Panel) -> Self {
        PanelController { frames: 0, phys: 0 }
    }

    #[export]
    fn _ready(&self, owner: TRef<Panel>) {
        godot_print!("Instance PanelController ready");
        owner.add_to_group("panelgroup", false);

        let b = owner.get_node("Button").expect("didnt get button");
        let b = unsafe { b.assume_safe() };
        let b = b.cast::<Button>().expect("couldnt cast");

        b.connect(
            "pressed",
            owner,
            "_on_Button_pressed",
            VariantArray::new_shared(),
            0,
        )
        .expect("couldnt connect");
    }

    #[export]
    fn _on_Button_pressed(&self, owner: &Panel) {
        let l = owner.get_node("Label").expect("didn't get label");
        let l = unsafe { l.assume_safe() };
        let l = l.cast::<Label>().expect("couldnt cast");

        l.set_text("The button. Pressed!");

        godot_print!("button press");

        let g = owner.get_tree().expect("couldnt get tree");
        let g = unsafe { g.assume_safe() };
        let g = g.cast::<SceneTree>().expect("couldnt cast");

        godot_print!("{:#?}", g);
        g.call_group(
            "panelgroup",
            "the_callback",
            &[Variant::from_str("I'm the argument")],
        );
    }

    #[export]
    fn the_callback(&self, _owner: &Panel, arg: GodotString) {
        godot_print!("I'm the callback, {}", arg);
    }

    #[export]
    fn _process(&mut self, _owner: &Panel, _delta: f32) {
        self.frames += 1;

        let l = _owner.get_node("Label").expect("didn't get label");
        let l = unsafe { l.assume_safe() };
        let l = l.cast::<Label>().expect("couldnt cast");

        let text = format!("frames: {}", self.frames);

        l.set_text(text);
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Panel, _delta: f64) {
        self.phys += 1;

        let l = _owner.get_node("Label2").expect("didn't get label");
        let l = unsafe { l.assume_safe() };
        let l = l.cast::<Label>().expect("couldnt cast");

        let text = format!("phys: {}", self.phys);

        l.set_text(text);
    }
}
