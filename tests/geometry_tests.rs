use scenekit::Geometry;

#[test]
fn test_geometry_constructors_create_objects() {
    assert!(Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).is_some());
    assert!(Geometry::sphere(1.0).is_some());
    assert!(Geometry::cylinder(0.25, 2.0).is_some());
    assert!(Geometry::cone(0.0, 0.5, 1.0).is_some());
    assert!(Geometry::plane(2.0, 3.0).is_some());
    assert!(Geometry::floor().is_some());
    assert!(Geometry::text("Hello", 0.1).is_some());
}
