pub fn create_direction_vector(angle: f32) -> [f32; 2] {
    let forward_vector = vecmath::vec2_normalized([0f32, 1f32]);

    [
        forward_vector[0] * angle.cos() - forward_vector[1] * angle.sin(),
        forward_vector[0] * angle.sin() + forward_vector[1] * angle.cos(),
    ]
}
