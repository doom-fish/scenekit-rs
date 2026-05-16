use scenekit::Geometry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _box = Geometry::box_geometry(1.0, 1.0, 1.0, 0.1).ok_or("missing box")?;
    let _sphere = Geometry::sphere(0.5).ok_or("missing sphere")?;
    let _cylinder = Geometry::cylinder(0.25, 2.0).ok_or("missing cylinder")?;
    let _cone = Geometry::cone(0.0, 0.5, 1.0).ok_or("missing cone")?;
    let _plane = Geometry::plane(3.0, 2.0).ok_or("missing plane")?;
    let _floor = Geometry::floor().ok_or("missing floor")?;
    let text = Geometry::text("SceneKit", 0.2).ok_or("missing text")?;
    assert!(text.first_material().is_some());
    println!("✅ geometry primitives created");
    Ok(())
}
