/*

config structs and methods relating to the clients connection towards the appliances

*/


use hyper::Client;

use serde::{Serialize, Deserialize};
use serde_yaml;

use hyper::client::HttpConnector;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SolaceApiConfig {
    pub username: String,
    pub password: String,
    pub host: String
}


pub fn readconfig(config: String) -> Result<SolaceApiConfig, Box<std::error::Error>> {
    // read the config file
    let file = std::fs::File::open(config)?;
    let config_data: SolaceApiConfig = serde_yaml::from_reader(file)?;
    println!("Read SolaceApiConfig: {:?}", config_data);
    Ok(config_data)
}
