use pendulum::utils::read_parameters;

#[test]
fn test_read_parameters() {
    let result = read_parameters("params.yaml");

    assert!(result.is_ok(), "Failed to read parameters from file.");

    let params = result.unwrap();
    
    // assert the values are as expected
    assert_eq!(params.bob_mass, 1.0);
    assert_eq!(params.string_length, 1.0);
}
