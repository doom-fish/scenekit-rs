use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use apple_cf::cg::CGContext;
use scenekit::{Geometry, Node, Scene, SceneExportDelegate};

fn output_dir() -> PathBuf {
    let dir = PathBuf::from("target/test-output");
    fs::create_dir_all(&dir).expect("create test output directory");
    dir
}

fn textured_scene() -> Scene {
    let context = CGContext::new_rgba8(8, 4).expect("bitmap context");
    context.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
    context.fill_rect(0.0, 0.0, 8.0, 4.0);
    let image = context.snapshot_to_image().expect("image");

    let scene = Scene::new().expect("scene");
    let geometry = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).expect("box");
    geometry
        .first_material()
        .expect("material")
        .diffuse()
        .set_cg_image(&image);
    let node = Node::with_geometry(Some(&geometry)).expect("node");
    scene.root_node().add_child_node(&node);
    scene
}

#[test]
fn write_to_url_reports_a_failed_write_even_when_the_path_exists() {
    let occupied = output_dir().join(format!("occupied-{}.scn", std::process::id()));
    fs::create_dir_all(&occupied).expect("create directory at the export path");
    let scene = textured_scene();
    let result = scene.write_to_url(&occupied, None);
    fs::remove_dir_all(&occupied).expect("cleanup");
    let error = result.expect_err("a directory at the target path must not count as a write");
    assert!(!error.to_string().is_empty());
}

#[test]
fn write_to_url_rejects_paths_with_nul_bytes() {
    let scene = Scene::new().expect("scene");
    assert!(scene.write_to_url("bad\0path.scn", None).is_err());
}

#[test]
fn export_delegate_receives_the_image_it_must_write() {
    let scene = textured_scene();
    let written = Arc::new(Mutex::new(Vec::new()));
    let delegate = SceneExportDelegate::new({
        let written = Arc::clone(&written);
        move |image, document_url, _original_image_url| {
            let target = Path::new(document_url).with_file_name(format!(
                "export-texture-{}-{}.png",
                std::process::id(),
                written.lock().expect("written").len()
            ));
            image.save_png(&target).ok()?;
            written
                .lock()
                .expect("written")
                .push((image.width(), image.height(), target.clone()));
            Some(target.to_string_lossy().into_owned())
        }
    })
    .expect("export delegate");

    let document = output_dir().join(format!("export-{}.dae", std::process::id()));
    scene
        .write_to_url(&document, Some(&delegate))
        .expect("export scene");
    let received = written.lock().expect("written").clone();
    fs::remove_file(&document).ok();
    for (_, _, texture) in &received {
        assert!(texture.exists(), "the delegate wrote {}", texture.display());
        fs::remove_file(texture).ok();
    }
    assert!(
        !received.is_empty(),
        "SceneKit never asked the delegate to write the image"
    );
    assert!(received
        .iter()
        .all(|(width, height, _)| (*width, *height) == (8, 4)));
}
