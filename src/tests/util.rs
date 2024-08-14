use rand::random;

pub fn generate_random_bytes() -> [u8; 32] {
    return random();
}
