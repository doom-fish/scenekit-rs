use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use scenekit::read_texture_bytes;

fn device() -> MetalDevice {
    MetalDevice::system_default().expect("Metal device")
}

fn upload(texture: &apple_metal::MetalTexture, bytes: &[u8], bytes_per_row: usize) {
    unsafe {
        texture
            .replace_region_2d(
                bytes,
                bytes_per_row,
                (0, 0),
                (texture.width(), texture.height()),
                0,
            )
            .expect("upload texture bytes");
    }
}

#[test]
fn rgba16_float_textures_are_read_with_eight_bytes_per_pixel() {
    let device = device();
    let (width, height) = (5, 3);
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::SHADER_READ | texture_usage::RENDER_TARGET,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(width, height, pixel_format::RGBA16FLOAT)
        })
        .expect("RGBA16Float texture");
    let pattern: Vec<u8> = (0..width * height * 8)
        .map(|index| u8::try_from(index % 251).expect("byte"))
        .collect();
    upload(&texture, &pattern, width * 8);

    let bytes = read_texture_bytes(&texture).expect("read RGBA16Float");
    assert_eq!(bytes.len(), width * height * 8);
    assert_eq!(bytes, pattern);
}

#[test]
fn bgra10_xr_textures_are_read_with_eight_bytes_per_pixel() {
    let device = device();
    let Some(texture) = device.new_texture(TextureDescriptor {
        usage: texture_usage::SHADER_READ,
        storage_mode: storage_mode::SHARED,
        ..TextureDescriptor::new_2d(4, 4, pixel_format::BGRA10_XR)
    }) else {
        eprintln!("skipping: this GPU has no BGRA10_XR textures");
        return;
    };
    let pattern: Vec<u8> = (0_usize..4 * 4 * 8)
        .map(|index| u8::try_from(index * 7 % 256).expect("byte"))
        .collect();
    upload(&texture, &pattern, 4 * 8);
    let bytes = read_texture_bytes(&texture).expect("read BGRA10_XR");
    assert_eq!(bytes.len(), 4 * 4 * 8);
    assert_eq!(bytes, pattern);
}

#[test]
fn private_textures_are_rejected() {
    let device = device();
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::SHADER_READ | texture_usage::RENDER_TARGET,
            storage_mode: storage_mode::PRIVATE,
            ..TextureDescriptor::new_2d(8, 8, pixel_format::RGBA16FLOAT)
        })
        .expect("private texture");
    let error = read_texture_bytes(&texture).expect_err("private storage must be rejected");
    assert!(error.to_string().contains("storage mode"), "{error}");
}

#[test]
fn compressed_textures_are_rejected() {
    let device = device();
    let texture = [pixel_format::BC1_RGBA, pixel_format::ASTC_4X4_LDR]
        .into_iter()
        .find_map(|format| {
            device.new_texture(TextureDescriptor {
                usage: texture_usage::SHADER_READ,
                storage_mode: storage_mode::SHARED,
                ..TextureDescriptor::new_2d(16, 16, format)
            })
        });
    let Some(texture) = texture else {
        eprintln!("skipping: this GPU supports neither BC nor ASTC textures");
        return;
    };
    let error = read_texture_bytes(&texture).expect_err("compressed formats must be rejected");
    assert!(error.to_string().contains("compressed"), "{error}");
}

#[test]
fn depth_textures_are_rejected() {
    let device = device();
    let Some(texture) = device.new_texture(TextureDescriptor {
        usage: texture_usage::SHADER_READ,
        storage_mode: storage_mode::SHARED,
        ..TextureDescriptor::new_2d(8, 8, pixel_format::DEPTH32FLOAT)
    }) else {
        eprintln!("skipping: shared depth textures are unavailable");
        return;
    };
    let error = read_texture_bytes(&texture).expect_err("depth formats must be rejected");
    assert!(error.to_string().contains("depth"), "{error}");
}
