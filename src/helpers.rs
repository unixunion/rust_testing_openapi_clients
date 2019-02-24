
use solace_semp_client::apis::configuration::BasicAuth;
use colored::*;
use log::{info};

// generate a credential for basicauth
pub fn gencred(username: String, password: String) -> BasicAuth {
    info!("{}", "generating credentials".green());
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}