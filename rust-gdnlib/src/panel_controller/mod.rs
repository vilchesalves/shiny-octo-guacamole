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
    fn _on_Button_pressed(&self, _owner: &Panel) {
        let l = _owner.get_node("Label").expect("didn't get label");
        let l = unsafe { l.assume_safe() };
        let l = l.cast::<Label>().expect("couldnt cast");

        l.set_text("The button. Pressed!");

        godot_print!("button press");
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
