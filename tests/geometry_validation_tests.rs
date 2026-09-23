use scenekit::{
    Geometry, GeometryElement, GeometryPrimitiveType, GeometrySource, GeometrySourceLayout,
    GeometrySourceSemantic, Matrix4, Node, Skinner, Vector3,
};

fn triangle_vertices() -> GeometrySource {
    GeometrySource::with_vertices(&[
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    ])
    .expect("vertex source")
}

fn u16_bytes(indices: &[u16]) -> Vec<u8> {
    indices
        .iter()
        .flat_map(|index| index.to_ne_bytes())
        .collect()
}

fn u32_bytes(indices: &[u32]) -> Vec<u8> {
    indices
        .iter()
        .flat_map(|index| index.to_ne_bytes())
        .collect()
}

fn f32_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn element(data: Option<&[u8]>, kind: GeometryPrimitiveType, count: usize, width: usize) -> bool {
    GeometryElement::with_data(data, kind, count, width).is_ok()
}

#[test]
fn elements_reject_unsupported_index_widths() {
    let data = [0_u8; 24];
    for width in [0, 3, 8, 16] {
        let error = GeometryElement::with_data(Some(&data), GeometryPrimitiveType::Point, 1, width)
            .expect_err("unsupported index width");
        assert!(error.to_string().contains("bytes_per_index"), "{error}");
    }
}

#[test]
fn element_data_must_match_the_primitive_count() {
    let triangle = u16_bytes(&[0, 1, 2]);
    assert!(element(
        Some(&triangle),
        GeometryPrimitiveType::Triangles,
        1,
        2
    ));
    assert!(!element(
        Some(&triangle),
        GeometryPrimitiveType::Triangles,
        2,
        2
    ));
    assert!(!element(
        Some(&triangle[..4]),
        GeometryPrimitiveType::Triangles,
        1,
        2
    ));
    let mut padded = triangle.clone();
    padded.extend([0, 0]);
    assert!(!element(
        Some(&padded),
        GeometryPrimitiveType::Triangles,
        1,
        2
    ));

    assert!(element(
        Some(&u16_bytes(&[0, 1, 2, 1])),
        GeometryPrimitiveType::TriangleStrip,
        2,
        2
    ));
    assert!(!element(
        Some(&u16_bytes(&[0, 1, 2])),
        GeometryPrimitiveType::TriangleStrip,
        2,
        2
    ));
    assert!(element(
        Some(&u16_bytes(&[0, 1])),
        GeometryPrimitiveType::Line,
        1,
        2
    ));
    assert!(!element(
        Some(&u16_bytes(&[0, 1])),
        GeometryPrimitiveType::Line,
        2,
        2
    ));
    assert!(element(
        Some(&[0, 1, 2]),
        GeometryPrimitiveType::Point,
        3,
        1
    ));
    assert!(element(
        Some(&u32_bytes(&[0, 1, 2])),
        GeometryPrimitiveType::Triangles,
        1,
        4
    ));
    assert!(!element(
        Some(&triangle),
        GeometryPrimitiveType::Triangles,
        usize::MAX / 2,
        2
    ));
    assert!(element(None, GeometryPrimitiveType::Triangles, 1, 2));
    assert!(element(Some(&[]), GeometryPrimitiveType::Triangles, 0, 2));
}

#[test]
fn polygon_elements_are_validated_from_their_vertex_counts() {
    let quad_and_triangle = u16_bytes(&[4, 3, 0, 1, 2, 3, 0, 2, 4]);
    assert!(element(
        Some(&quad_and_triangle),
        GeometryPrimitiveType::Polygon,
        2,
        2
    ));
    assert!(!element(
        Some(&quad_and_triangle[..quad_and_triangle.len() - 2]),
        GeometryPrimitiveType::Polygon,
        2,
        2
    ));
    let error = GeometryElement::with_data(
        Some(&u16_bytes(&[2, 0, 1])),
        GeometryPrimitiveType::Polygon,
        1,
        2,
    )
    .expect_err("two-vertex polygon");
    assert!(error.to_string().contains("at least 3"), "{error}");
    assert!(!element(
        Some(&u16_bytes(&[3])),
        GeometryPrimitiveType::Polygon,
        2,
        2
    ));
    assert!(!element(None, GeometryPrimitiveType::Polygon, 1, 2));
}

#[test]
fn geometry_rejects_indices_beyond_the_vertex_count() {
    let vertices = triangle_vertices();
    let good = GeometryElement::with_data(
        Some(&u16_bytes(&[0, 1, 2])),
        GeometryPrimitiveType::Triangles,
        1,
        2,
    )
    .expect("element");
    assert!(Geometry::with_sources_elements(&[&vertices], &[&good]).is_ok());

    let bad = GeometryElement::with_data(
        Some(&u16_bytes(&[0, 1, 3])),
        GeometryPrimitiveType::Triangles,
        1,
        2,
    )
    .expect("element");
    let error = Geometry::with_sources_elements(&[&vertices], &[&good, &bad])
        .expect_err("index 3 with three vertices");
    assert!(error.to_string().contains("geometry element 1"), "{error}");
    assert!(error.to_string().contains("vertex 3"), "{error}");

    let wide = GeometryElement::with_data(
        Some(&u32_bytes(&[0, 1, 70_000])),
        GeometryPrimitiveType::Triangles,
        1,
        4,
    )
    .expect("element");
    assert!(Geometry::with_sources_elements(&[&vertices], &[&wide]).is_err());

    let polygon = GeometryElement::with_data(
        Some(&u16_bytes(&[3, 0, 1, 5])),
        GeometryPrimitiveType::Polygon,
        1,
        2,
    )
    .expect("element");
    assert!(Geometry::with_sources_elements(&[&vertices], &[&polygon]).is_err());
}

#[test]
fn implicit_elements_need_enough_vertices() {
    let vertices = triangle_vertices();
    let one = GeometryElement::with_data(None, GeometryPrimitiveType::Triangles, 1, 2)
        .expect("implicit element");
    assert!(Geometry::with_sources_elements(&[&vertices], &[&one]).is_ok());
    let two = GeometryElement::with_data(None, GeometryPrimitiveType::Triangles, 2, 2)
        .expect("implicit element");
    assert!(Geometry::with_sources_elements(&[&vertices], &[&two]).is_err());
}

#[test]
fn the_smallest_source_limits_the_indices() {
    let vertices = triangle_vertices();
    let normals = GeometrySource::with_normals(&[Vector3::new(0.0, 0.0, 1.0); 2]).expect("normals");
    let element = GeometryElement::with_data(
        Some(&u16_bytes(&[0, 1, 2])),
        GeometryPrimitiveType::Triangles,
        1,
        2,
    )
    .expect("element");
    assert!(Geometry::with_sources_elements(&[&vertices, &normals], &[&element]).is_err());
    assert!(Geometry::with_sources_elements(&[&vertices], &[&element]).is_ok());
}

#[test]
fn generic_sources_validate_their_layout() {
    let floats = f32_bytes(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    let layout = GeometrySourceLayout::packed(GeometrySourceSemantic::Vertex, 2, true, 3, 4);
    assert_eq!(layout.required_len().expect("layout"), 24);
    let source = GeometrySource::with_data(&floats, layout).expect("source");
    assert_eq!(source.vector_count(), 2);

    assert!(GeometrySource::with_data(&floats[..20], layout).is_err());
    let invalid = [
        GeometrySourceLayout {
            vector_count: 0,
            ..layout
        },
        GeometrySourceLayout {
            components_per_vector: 5,
            data_stride: 20,
            ..layout
        },
        GeometrySourceLayout {
            bytes_per_component: 2,
            ..layout
        },
        GeometrySourceLayout {
            uses_float_components: false,
            bytes_per_component: 8,
            data_stride: 24,
            ..layout
        },
        GeometrySourceLayout {
            data_stride: 8,
            ..layout
        },
        GeometrySourceLayout {
            data_offset: 4,
            ..layout
        },
        GeometrySourceLayout {
            vector_count: usize::MAX,
            ..layout
        },
    ];
    for layout in invalid {
        assert!(
            GeometrySource::with_data(&floats, layout).is_err(),
            "{layout:?} was accepted"
        );
    }

    let interleaved = GeometrySourceLayout {
        semantic: GeometrySourceSemantic::Texcoord,
        vector_count: 2,
        uses_float_components: true,
        components_per_vector: 2,
        bytes_per_component: 4,
        data_offset: 4,
        data_stride: 12,
    };
    assert_eq!(interleaved.required_len().expect("layout"), 24);
    assert!(GeometrySource::with_data(&floats, interleaved).is_ok());
}

fn skinning_sources(indices: &[u8], weights: &[f32]) -> (GeometrySource, GeometrySource) {
    let weights = GeometrySource::with_data(
        &f32_bytes(weights),
        GeometrySourceLayout::packed(
            GeometrySourceSemantic::BoneWeights,
            weights.len(),
            true,
            1,
            4,
        ),
    )
    .expect("weights");
    let indices = GeometrySource::with_data(
        indices,
        GeometrySourceLayout::packed(
            GeometrySourceSemantic::BoneIndices,
            indices.len(),
            false,
            1,
            1,
        ),
    )
    .expect("indices");
    (weights, indices)
}

#[test]
fn skinners_are_built_from_generic_sources_and_validated() {
    let vertices = triangle_vertices();
    let element = GeometryElement::with_data(
        Some(&u16_bytes(&[0, 1, 2])),
        GeometryPrimitiveType::Triangles,
        1,
        2,
    )
    .expect("element");
    let geometry = Geometry::with_sources_elements(&[&vertices], &[&element]).expect("geometry");
    let root_bone = Node::new().expect("bone");
    let tip_bone = Node::new().expect("bone");
    let bones = [&root_bone, &tip_bone];

    let (weights, indices) = skinning_sources(&[0, 1, 0], &[1.0, 1.0, 1.0]);
    let skinner = Skinner::new(&geometry, &bones, None, &weights, &indices).expect("skinner");
    let node = Node::with_geometry(Some(&geometry)).expect("node");
    node.set_skinner(Some(&skinner));
    assert!(node.skinner().is_some());

    let (weights, indices) = skinning_sources(&[0, 2, 0], &[1.0, 1.0, 1.0]);
    let error = Skinner::new(&geometry, &bones, None, &weights, &indices)
        .expect_err("bone index 2 with two bones");
    assert!(error.to_string().contains("out of range"), "{error}");

    let (weights, _) = skinning_sources(&[0, 0], &[1.0, 1.0]);
    let (_, indices) = skinning_sources(&[0, 1, 0], &[1.0, 1.0, 1.0]);
    assert!(Skinner::new(&geometry, &bones, None, &weights, &indices).is_err());

    let (weights, indices) = skinning_sources(&[0, 0], &[1.0, 1.0]);
    let error = Skinner::new(&geometry, &bones, None, &weights, &indices)
        .expect_err("two weights for three vertices");
    assert!(error.to_string().contains("3 vertices"), "{error}");

    let (weights, indices) = skinning_sources(&[0, 1, 0], &[1.0, 1.0, 1.0]);
    assert!(Skinner::new(&geometry, &[], None, &weights, &indices).is_err());
    let identity = [Matrix4::identity(); 2];
    assert!(Skinner::new(&geometry, &bones, Some(&identity), &weights, &indices).is_ok());
    assert!(Skinner::new(&geometry, &bones, Some(&identity[..1]), &weights, &indices).is_err());
    assert!(Skinner::new(&geometry, &bones, None, &indices, &indices).is_err());
}
