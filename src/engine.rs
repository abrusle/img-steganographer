
pub mod encoding;
pub mod decoding;

pub fn is_img_supported(extension: &str) -> bool {
    match extension {
        "png" => true,
        _ => false,
    }
}
