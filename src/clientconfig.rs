/*

config structs and methods relateing to the clients connection towards the appliances

*/

//use solace_semp_client::apis::configuration::Configuration;
//use solace_semp_client::apis::configuration::BasicAuth;

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

//pub fn create_client_config(hyperclient: Client<HttpConnector>, auth: BasicAuth) -> Configuration<hyper::client::Connect>  {
//    Configuration {
//        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
//        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
//        client: hyperclient,
//        basic_auth: Some(auth),
//        oauth_access_token: None,
//        api_key: None,
//    }
//}


pub fn readconfig(config: String) -> Result<SolaceApiConfig, Box<std::error::Error>> {
    // read the config file
    let file = std::fs::File::open(config)?;
    let config_data: SolaceApiConfig = serde_yaml::from_reader(file)?;
    println!("Read SolaceApiConfig: {:?}", config_data);
    Ok(config_data)
}
