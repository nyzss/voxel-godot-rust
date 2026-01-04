use godot::{
    classes::{
        CollisionShape3D, FastNoiseLite, Input, InputEvent, InputEventMouseButton,
        MultiMeshInstance3D, Performance, StaticBody3D, input,
    },
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
    world_size: Vector3,
    multi_mesh_instance: OnReady<Gd<MultiMeshInstance3D>>,

    mesh_instance: OnReady<Gd<MeshInstance>>,

    #[export]
    colors: Array<Color>,
    data: Vec<Vector3>,
}

#[godot_api]
impl World {
    #[func]
    fn get_total_cubes(&self) -> u32 {
        self.data.len() as u32
    }

    fn generate_world_data(&mut self) {
        let rng = FastNoiseLite::new_gd();

        for x in 0..self.world_size.x as usize {
            for y in 0..self.world_size.y as usize {
                for z in 0..self.world_size.z as usize {
                    let (x, y, z) = (x as f32, y as f32, z as f32);
                    let rand = rng.get_noise_3d(x, y, z);

                    if rand > self.cut_off {
                        self.data.push(Vector3::new(x, y, z));
                    }
                }
            }
        }
    }

    fn _generate_world_mesh(&mut self) {
        let mut multimesh = self.multi_mesh_instance.get_multimesh().unwrap();
        multimesh.set_instance_count(self.data.len() as i32);

        let positions: Vec<Vector3> = std::mem::take(&mut self.data);
        for (i, pos) in positions.into_iter().enumerate() {
            multimesh.set_instance_transform(i as i32, Transform3D::new(Basis::default(), pos));
            multimesh.set_instance_color(
                i as i32,
                self.colors
                    .get(fastrand::usize(0..self.colors.len()))
                    .unwrap(),
            );

            let mut collision_node = StaticBody3D::new_alloc();
            let mut collision_shape = CollisionShape3D::new_alloc();

            collision_shape.set_shape(
                &multimesh
                    .get_mesh()
                    .unwrap()
                    .create_trimesh_shape()
                    .unwrap(),
            );

            collision_node.add_child(&collision_shape);
            collision_node.set_position(pos);

            self.base_mut().add_child(&collision_node);
        }
    }
}

#[godot_api]
impl INode3D for World {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,

            cut_off: 0.5,
            world_size: Vector3::new(16., 16., 16.),
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

        self.generate_world_data();

        let data = std::mem::take(&mut self.data);
        self.mesh_instance.bind_mut().generate_mesh(data);

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
