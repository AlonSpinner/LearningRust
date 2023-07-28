use std::fs::File;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub bob_mass: f64,
    pub string_length: f64
}

pub fn read_parameters(filename : &str) -> Result<Parameters, Box<dyn Error>> {
    let file = File::open(filename)?;
    let params: Parameters = serde_yaml::from_reader(file)?;
    Ok(params)
}
