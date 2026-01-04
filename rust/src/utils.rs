use godot::builtin::Vector3;

pub fn build_index(x: usize, y: usize, z: usize, chunk_size: usize, max_ceilling: usize) -> usize {
    x + y * chunk_size + z * chunk_size * max_ceilling
}

pub fn build_vector(i: usize, chunk_size: usize, max_ceilling: usize) -> Vector3 {
    let x = i % chunk_size;
    let y = (i / chunk_size) % chunk_size;
    let z = ((i / chunk_size) / max_ceilling) % max_ceilling;

    let (x, y, z) = (x as f32, y as f32, z as f32);

    Vector3::new(x, y, z)
}
