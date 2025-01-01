use auric::render::visuals;

#[test]
fn test_visuals_vertex_data() {
    let mut visual_data = visuals::VisualData::new();
    visual_data.update(vec![0.1, 0.2, 0.3, 0.4]);

    let vertex_data = visual_data.create_vertex_data();
    assert_eq!(vertex_data.len(), 12); // 4 vertices with 3 components each (x, y, z)
}

