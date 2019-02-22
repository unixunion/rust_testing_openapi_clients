
use solace_semp_client::models::MsgVpn;
use solace_semp_client::apis::client::APIClient;
use hyper::client::HttpConnector;
use futures::AndThen;
use tokio_core::reactor::Core;
use futures::future::Future;
use std::error::Error;
use serde_json::to_value;
use std::ptr::null;



//pub fn provision(filename: String, api: APIClient<HttpConnector>, core: &mut Core) {
pub fn provision(filename: String) -> Result<MsgVpn, &'static str> {

    match readconfig(filename.to_owned()) {
        Ok(vpn) => {
            Ok(vpn)
        },
        Err(e) => {
            println!("{}", e);
            Err("error")
        }
    }
}


// Read a config
pub fn readconfig(config: String) -> Result<MsgVpn, Box<std::error::Error>> {
    // read the config file
    let file = std::fs::File::open(config)?;
    let config_data: MsgVpn = serde_yaml::from_reader(file)?;
    println!("Reading provision: {:?}", config_data);
    Ok(config_data)
}