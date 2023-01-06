use blake2::{Blake2s256, Digest};

pub fn get_avatar_url(email: &str, size: i32) -> String {
    // create a Blake2b512 object
    let mut hasher = Blake2s256::new();
    // write input message
    hasher.update(email.as_bytes());
    // read hash digest and consume hasher
    let digest = base64::encode(hasher.finalize());
    format!(
        "https://avatars.dicebear.com/api/bottts/{}.svg?size={}",
        digest, size
    )
}
