pub fn xor_encrypt(data: &[u8], password: &str) -> Vec<u8> {
    data.iter()
        .zip(password.bytes().cycle())
        .map(|(b, p)| b ^ p)
        .collect()
}
