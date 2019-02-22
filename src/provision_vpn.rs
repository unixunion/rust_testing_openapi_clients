
use solace_semp_client::models::MsgVpn;
use solace_semp_client::apis::client::APIClient;
use hyper::client::HttpConnector;
use futures::AndThen;
use tokio_core::reactor::Core;
use futures::future::Future;
use std::error::Error;
use serde_json::to_value;
use std::ptr::null;

//
//Struct VpnProvisionFile {
//
//}



//pub fn provision(filename: String, api: APIClient<HttpConnector>, core: &mut Core) {
pub fn provision(filename: String) -> Result<MsgVpn, &'static str> {

    match readconfig(filename.to_owned()) {
        //Ok(vpn) => {
//            let resp = api
//                .default_api()
//                .create_msg_vpn(vpn, Vec::new())
//                .and_then(|vpn| {
//                    print!("{:?}", vpn);
//                    futures::future::ok(())
//                });
//            core.run(resp).expect("Failed request");
          //  vpn
        Ok(vpn) => {
//            tvpn = MsgVpn::new();
            Ok(vpn)
        },
        Err(e) => {
            println!("{}", e);
            Err("error")
        }
    }
//        Err(e) => {
//            println!("{}: {}", "Error reading yaml", e);
//        }
    //}
}


// Read a config
pub fn readconfig(config: String) -> Result<MsgVpn, Box<std::error::Error>> {
    // read the config file
    let file = std::fs::File::open(config)?;
    let config_data: MsgVpn = serde_yaml::from_reader(file)?;
    println!("Reading provision: {:?}", config_data);
    Ok(config_data)
}


//pub fn create_message_vpn(vpn: MsgVpn, api: APIClient<HttpConnector>) {
//
//}