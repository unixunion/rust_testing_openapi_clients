
//extern crate solace_semp_client;
//extern crate colored;

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use serde_json;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnsResponse;
use solace_semp_client::apis::configuration::BasicAuth;
use futures::Async;
use hyper::client::HttpConnector;
use std::env;
use clap::{Arg, App, SubCommand};


// generate a credential
fn gencred(username: String, password: String) -> BasicAuth {
    println!("{}", "generating credentials".green());
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}


fn main() {

    println!("{}", "Solace Provisioner".yellow());

    // Handle args
    let matches = App::new("Solace Provisioner")
        .version("1.0")
        .author("Kegan Holtzhausen <marzubus@gmail.com>")
        .about("Creates solace managed objects")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("The provision plan")
            .required(true)
            .index(1))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("default.yaml");

    // Try autocomete matches methods here to see issue: https://github.com/intellij-rust/intellij-rust/issues/2525
    let provisionplan = matches.value_of("INPUT").unwrap_or("provision.yaml");


    println!("{}{}", "config file: ".white(), config.to_owned().green());
    println!("{}{}", "provision plan: ".white(), provisionplan.to_owned().blue());

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let hyperclient = Client::new(&handle);
    let auth = gencred("admin".to_owned(), "admin".to_owned());

    // the configuration for the APIClient
    let configuration = Configuration {
        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
        client: hyperclient,
        basic_auth: Some(auth),
        oauth_access_token: None,
        api_key: None,
    };

    // the API Client from swagger spec
    let client = APIClient::new(configuration);

    // SEMP where
    let mut wherevec: Vec<String> = Vec::new();
    wherevec.push(String::from("msgVpnName==*"));

    // SEMP selector
    let mut selectvec: Vec<String> = Vec::new();
    selectvec.push(String::from(""));

    // a type mapping
    //type ShowVpnResponse = Box<Future<Item=MsgVpnsResponse, Error=Error<serde_json::Value>>>;

    println!("{}", "Composing request".green());
    let  resp = client
        .msg_vpn_api()
        .get_msg_vpns(10, "", wherevec, selectvec)
        .and_then(|vpn| {
            print!("{:?}", vpn);
            futures::future::ok(())
        });


    println!("{}", "Making request".green());
    core.run(resp).expect("Failed request");
    println!("{}", "Requests made".yellow());



    println!("Completed requests");



}
