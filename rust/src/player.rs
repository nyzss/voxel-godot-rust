use godot::{
    classes::{CharacterBody3D, ICharacterBody3D, Input},
    global::move_toward,
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
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 5.0,
            jump_velocity: 4.5,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut velocity = self.base().get_velocity();
        if !self.base().is_on_floor() {
            velocity += self.base().get_gravity() * delta as f32;
        }
        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            velocity.y = self.jump_velocity;
        }

        let input_dir = input.get_vector("move_left", "move_right", "move_up", "move_down");
        let mut direction =
            self.base().get_transform().basis * Vector3::new(input_dir.x, 0., input_dir.y);
        if direction.length() > 0.0 {
            direction = direction.normalized();

            velocity.x = direction.x * self.speed;
            velocity.z = direction.z * self.speed;
        } else {
            velocity.x = move_toward(velocity.x as f64, 0. as f64, self.speed as f64) as f32;
            velocity.z = move_toward(velocity.z as f64, 0. as f64, self.speed as f64) as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
