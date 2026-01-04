use std::time::Instant;

use godot::{
    classes::{Input, InputEvent, InputEventMouseButton, MultiMeshInstance3D, Performance, input},
    global::MouseButton,
    prelude::*,
};

use crate::mesh_instance::MeshInstance;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct World {
    base: Base<Node3D>,

    #[export(range=(-1.0, 1.0))]
    cut_off: f32,
    #[export]
    chunk_size: u32,
    #[export]
    max_ceilling: u32,
    #[export]
    world_seed: i32,
    multi_mesh_instance: OnReady<Gd<MultiMeshInstance3D>>,

    mesh_instance: OnReady<Gd<MeshInstance>>,

    #[export]
    colors: Array<Color>,
    data: Vec<bool>,
}

#[godot_api]
impl World {
    #[func]
    fn get_total_cubes(&self) -> u32 {
        self.data.len() as u32
    }

    fn generate_world_data(&mut self) {
        let mut rng = fastnoise_lite::FastNoiseLite::with_seed(self.world_seed);
        rng.set_frequency(Some(0.03));

        let chunk_size = self.chunk_size as usize;

        let start = Instant::now();
        for x in 0..chunk_size {
            for z in 0..chunk_size {
                for y in 0..self.max_ceilling as usize {
                    let index = x + y * chunk_size + z * chunk_size * self.max_ceilling as usize;

                    let rand = rng.get_noise_3d(x as f32, y as f32, z as f32);

                    if rand > self.cut_off {
                        self.data[index] = true;
                    }
                }
            }
        }
        let end = Instant::now().duration_since(start);

        godot_print!("end: {:?}", end);
    }
}

#[godot_api]
impl INode3D for World {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,

            cut_off: 0.5,
            chunk_size: 16,
            max_ceilling: 64,
            world_seed: 0,
            multi_mesh_instance: OnReady::from_node("MultiMeshInstance3D"),

            mesh_instance: OnReady::from_node("MeshInstance3D"),

            colors: Array::new(),
            data: Vec::new(),
        }
    }

    fn ready(&mut self) {
        let mut performance = Performance::singleton();
        let mut input = Input::singleton();

        performance.add_custom_monitor(
            "game/cubes",
            &Callable::from_object_method(&self.base(), "get_total_cubes"),
        );

        self.data =
            vec![
                false;
                self.chunk_size as usize * self.chunk_size as usize * self.max_ceilling as usize
            ];

        self.generate_world_data();

        let data = std::mem::take(&mut self.data);
        self.mesh_instance
            .bind_mut()
            .generate_mesh(data, self.chunk_size, self.max_ceilling);

        input.set_mouse_mode(input::MouseMode::CAPTURED);
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
