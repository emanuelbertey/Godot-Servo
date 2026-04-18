use godot::prelude::*;

struct GodotServo;

#[gdextension]
unsafe impl ExtensionLibrary for GodotServo {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::MainLoop {
            godot_print!("Hello, world!");
        }
    }
}
