use godot::{
    classes::{CsgBox3D, FastNoiseLite, Input, InputEvent, InputEventMouseButton, input},
    global::MouseButton,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
struct World {
    base: Base<Node3D>,

    #[export(range=(-1.0, 1.0))]
    cut_off: f32,
    #[export]
    world_size: Vector3,
    default_cube: OnReady<Gd<CsgBox3D>>,
}

#[godot_api]
impl INode3D for World {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,

            cut_off: 0.5,
            world_size: Vector3::new(16., 16., 16.),
            default_cube: OnReady::from_node("DefaultCube"),
        }
    }

    fn ready(&mut self) {
        let mut input = Input::singleton();

        input.set_mouse_mode(input::MouseMode::CAPTURED);

        let rng = FastNoiseLite::new_gd();

        for x in 0..self.world_size.x as usize {
            for y in 0..self.world_size.y as usize {
                for z in 0..self.world_size.z as usize {
                    let (x, y, z) = (x as f32, y as f32, z as f32);
                    let rand = rng.get_noise_3d(x, y, z);

                    if rand > self.cut_off {
                        let mut dup_cube = self
                            .default_cube
                            .duplicate()
                            .unwrap()
                            .try_cast::<CsgBox3D>()
                            .unwrap();

                        let position = Vector3::new(x, y, z);
                        dup_cube.set_position(position);

                        self.base_mut().add_child(&dup_cube);
                    }
                }
            }
        }

        self.default_cube.queue_free();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let mut input = Input::singleton();

        let mouse_mode = input.get_mouse_mode();
        if event.is_action_pressed("ui_cancel") {
            // Could simply quit but decided to change the mouse mode
            // self.base_mut().get_tree().unwrap().quit();
            input.set_mouse_mode(input::MouseMode::VISIBLE);
        }
        if mouse_mode == input::MouseMode::VISIBLE
            && let Ok(event) = event.try_cast::<InputEventMouseButton>()
            && event.get_button_index() == MouseButton::LEFT
            && event.is_pressed()
        {
            input.set_mouse_mode(input::MouseMode::CAPTURED);
        }
    }
}
