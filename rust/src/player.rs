use godot::{
    classes::{
        Camera3D, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion,
        input,
    },
    global::{deg_to_rad, move_toward},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,

    #[export]
    speed: f32,
    #[export]
    jump_velocity: f32,
    flying: bool,

    eye_camera: OnReady<Gd<Camera3D>>,
    head: OnReady<Gd<Node3D>>,

    #[export]
    mouse_sensitivity: f32, // radians per pixel
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 5.0,
            jump_velocity: 4.5,
            flying: false,

            head: OnReady::from_node("Head"),
            eye_camera: OnReady::from_node("Head/EyeCamera"),

            mouse_sensitivity: 0.001,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut final_speed = self.speed;

        if input.is_action_just_pressed("flying") {
            self.flying = !self.flying;
        }

        let mut velocity = self.base().get_velocity();
        if !self.base().is_on_floor() {
            if self.flying {
                velocity = Vector3::ZERO;
            } else {
                velocity += self.base().get_gravity() * delta as f32;
            }
        }
        if input.is_action_pressed("jump") && (self.flying || self.base().is_on_floor()) {
            velocity.y = self.jump_velocity;
        }

        if input.is_action_pressed("sprint") {
            final_speed *= 4.0;
        }

        let input_dir = input.get_vector("move_left", "move_right", "move_up", "move_down");

        let direction = (self.eye_camera.get_global_transform().basis
            * Vector3::new(input_dir.x, 0., input_dir.y))
        .try_normalized();

        if let Some(direction) = direction {
            if self.flying {
                velocity = direction * final_speed;
            } else {
                velocity.x = direction.x * final_speed;
                velocity.z = direction.z * final_speed;
            }
        } else {
            velocity.x = move_toward(velocity.x as f64, 0.0, final_speed as f64) as f32;
            velocity.z = move_toward(velocity.z as f64, 0.0, final_speed as f64) as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let input = Input::singleton();

        if let Ok(event) = event.try_cast::<InputEventMouseMotion>()
            && input.get_mouse_mode() == input::MouseMode::CAPTURED
        {
            let relative = event.get_relative() * self.mouse_sensitivity;

            self.head.rotate_y(-relative.x);
            self.eye_camera.rotate_x(-relative.y);

            let rotation = self.eye_camera.get_rotation().clamp(
                Vector3::new(deg_to_rad(-90.) as f32, 0., 0.),
                Vector3::new(deg_to_rad(90.) as f32, 0., 0.),
            );

            self.eye_camera.set_rotation(rotation);
        }
    }
}
