
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use clap::{Arg, App};
use serde_yaml;
use log::{info};
use std::process::exit;
use solace_semp_client::models::MsgVpn;
use solace_semp_client::models::MsgVpnQueue;
use generics_yaml_deserializer::{generics_yaml_deserializer::Ptr, generics_yaml_deserializer::Outer};
use std::net::Shutdown::Read;

mod clientconfig;
mod helpers;


fn main() {

    //println!("{}", "Solace Provisioner".yellow());
    info!(target: "solace-provision", "{} {}", "Solace".red(), "Provisioner".blue());

    // Handle args
    let matches = App::new("Solace Provision")
        .version("1.0")
        .author("Kegan Holtzhausen <marzubus@gmail.com>")
        .about("Creates solace managed objects")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("vpn")
            .short("v")
            .long("vpn")
            .help("a VPN config file")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("queue")
            .short("q")
            .long("queue")
            .help("a Queue config file")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("update")
            .short("u")
            .long("update")
            .help("Update existing object")
            .required(false))
        .get_matches();

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");

    // Try autocomete matches methods here to see issue: https://github.com/intellij-rust/intellij-rust/issues/2525
    let vpn_file = matches.value_of("vpn").unwrap_or("undefined");
    let queue_file = matches.value_of("queue").unwrap_or("undefined");
    let update_mode = matches.is_present("update");

    println!("{}{}", "using config file: ".white(), config_file_name.to_owned().green());
    println!("{}{}", "using vpn file: ".white(), vpn_file.to_owned().blue());
    println!("{}{}", "using queue file: ".white(), queue_file.to_owned().blue());


    // configure the http client
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let hyperclient = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle)
            .unwrap()
        )
        .build(&handle);
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

    // read the configuration for the api client
    match clientconfig::readconfig(config_file_name.to_owned()) {
        Ok(yaml_str) => {
            configuration.base_path = yaml_str.host;
            let auth = helpers::gencred(yaml_str.username, yaml_str.password);
            configuration.basic_auth = Some(auth);
        },
        Err(e) => {
            println!("{}: {}", "Error reading yaml".red(), e);
            exit(1);
        }
    }

    // the API Client from swagger spec
    let client = APIClient::new(configuration);

//    // SEMP where
//    let mut wherevec: Vec<String> = Vec::new();
//    wherevec.push(String::from("msgVpnName==*"));
//
//    // SEMP selector
//    let mut selectvec: Vec<String> = Vec::new();
//    selectvec.push(String::from(""));
//
//
//    println!("{}", "Composing request".green());
//    let resp = client
//        .msg_vpn_api()
//        .get_msg_vpns(10, "", wherevec, selectvec)
//        .and_then(|vpn| {
//            print!("{:?}", vpn);
//            futures::future::ok(())
//        });
//
//
//    println!("{}", "Making request".green());
//    core.run(resp).expect("Failed request");
//    println!("{}", "Requests made".yellow());


    // VPN provision if file is passed
    if vpn_file.to_owned() != "undefined" {

        // read in the file
        let file = std::fs::File::open(vpn_file).unwrap();
        let deserialized_vpn: Outer<MsgVpn> = serde_yaml::from_reader(file).unwrap();

        // UPDATE existing VPN if --update is passed
        if update_mode {
            // update mode
            match deserialized_vpn.ptr {
                Ptr::Owned(vpn) => {
                    println!("{:?}", vpn);

                    let vpn_name = &vpn.msg_vpn_name();

                    let resp = client
                        .default_api()
                        .update_msg_vpn(&vpn_name.unwrap().to_owned(), *vpn, Vec::new())
                        .and_then(|vpn| {
                            println!("{:?}", vpn);
                            futures::future::ok(())
                        });
                    core.run(resp).expect("Failed request");
                },
                _ => unimplemented!()
//                Ptr::Ref(_) => { println!("error") },
//                Ptr::Owned(_) => { println!("error") }
            };
        } else {
            // NEW Vpn
            match deserialized_vpn.ptr {
                Ptr::Owned(vpn) => {
                    println!("{:?}", vpn);
                    let resp = client
                        .default_api()
                        .create_msg_vpn(*vpn, Vec::new())
                        .and_then(|vpn| {
                            println!("{:?}", vpn);
                            futures::future::ok(())
                        });
                    core.run(resp).expect("Failed request");
                },
                _ => unimplemented!()
//                Ptr::Ref(_) => { println!("error") },
//                Ptr::Owned(_) => { println!("error") }
            };
        }

    }

    // Provision Queue from file
    if queue_file.to_owned() != "undefined" {

    }



    println!("{}","Provision completed".purple());



}
