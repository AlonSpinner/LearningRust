use std::fs::File;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Parameters {
    bob_mass: f64,
    string_length: f64,
    gravity: f64,
}

fn read_parameters(filename : &str) -> Result<Parameters, Box<dyn Error>> {
    let file = File::open(filename)?;
    let params: Parameters = serde_yaml::from_reader(file)?;
    Ok(params)
}

#[test]
fn test_read_parameters() {
    let result = read_parameters("params.yaml");

    assert!(result.is_ok(), "Failed to read parameters from file.");

    let params = result.unwrap();
    
    // assert the values are as expected
    assert_eq!(params.bob_mass, 1.0);
    assert_eq!(params.string_length, 1.0);
    assert_eq!(params.gravity, 9.81);
}
