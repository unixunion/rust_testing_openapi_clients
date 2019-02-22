
//extern crate solace_semp_client;
//extern crate colored;

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use serde::{Serialize, Deserialize};
use serde_json;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnsResponse;
use solace_semp_client::apis::configuration::BasicAuth;
use futures::Async;
use hyper::client::HttpConnector;
use std::env;
use clap::{Arg, App, SubCommand};
use serde_yaml;
use log::{info, trace, warn};
use std::process::exit;

mod clientconfig;
mod helpers;
mod provision_vpn;

fn main() {

    //println!("{}", "Solace Provisioner".yellow());
    info!(target: "solace-provision", "{} {}", "Solace".red(), "Provisioner".blue());

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

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");

    // Try autocomete matches methods here to see issue: https://github.com/intellij-rust/intellij-rust/issues/2525
    let provision_plan_file_name = matches.value_of("INPUT").unwrap_or("provision.yaml");

    println!("{}{}", "using config file: ".white(), config_file_name.to_owned().green());
    println!("{}{}", "using provision plan: ".white(), provision_plan_file_name.to_owned().blue());
    

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let hyperclient = Client::new(&handle);
    let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

    // the configuration for the APIClient
    let mut configuration = Configuration {
        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
        client: hyperclient,
        basic_auth: Some(auth),
        oauth_access_token: None,
        api_key: None,
    };


    match clientconfig::readconfig(config_file_name.to_owned()) {
        Ok(yaml_str) => {
            configuration.base_path = yaml_str.host;
            let auth = helpers::gencred(yaml_str.username, yaml_str.password);
            configuration.basic_auth=Some(auth);
        },
        Err(e) => {
            println!("{}: {}", "Error reading yaml".red(), e);
            exit(1);
        }
    }

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
    let resp = client
        .msg_vpn_api()
        .get_msg_vpns(10, "", wherevec, selectvec)
        .and_then(|vpn| {
            print!("{:?}", vpn);
            futures::future::ok(())
        });

    println!("{}", "Making request".green());
    core.run(resp).expect("Failed request");
    println!("{}", "Requests made".yellow());


    println!("{}", "Creating VPNS");
    //provision_vpn::provision(provision_plan_file_name.to_owned(), client, &core);
    match provision_vpn::provision(provision_plan_file_name.to_owned()) {
        Ok(vpn) => {
            let resp = client
                .default_api()
                .create_msg_vpn(vpn, Vec::new())
                .and_then(|vpn| {
                    print!("{:?}", vpn);
                    futures::future::ok(())
                });
            core.run(resp).expect("Failed request");
        },
        Err(e) => {
            println!("provision error: {}", e);
        }
    }



    println!("Completed requests");



}
