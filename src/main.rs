
extern crate solace_semp_client;
extern crate colored;

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;

use hyper::Client;
//use hyper::rt::{self, Future, Stream};
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;

use colored::*;
use futures::{Future};
//use solace_semp_client::models::MsgVpnAclProfilesResponse;
use serde_json;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnsResponse;
//use hyper::header::{Headers, Host, Authorization};
use solace_semp_client::apis::configuration::BasicAuth;
use futures::Async;
//use hyper::Uri;
//use hyper::client::conn::ResponseFuture;
//use hyper::client::HttpConnector;

// : hyper::client::Client<C> : Option<BasicAuth>
//fn newconfig(client: Client, username: String, password: String) -> Configuration {
//
//    let auth = BasicAuth::from((username, Option(password)));
//
//    Configuration {
//        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
//        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
//        client: client,
//        basic_auth: auth,
//        oauth_access_token: None,
//        api_key: None,
//    };
//
//}

//trait MyClient {
//    fn post(&self, uri: Uri) -> ResponseFuture;
//}
//impl<C> MyClient for Client<C> {
//    fn post(&self, uri: Uri) -> ResponseFuture {
//        Client<C>::post (&self, uri)
//    }
//}

//enum MyClient {
//    Client (Client<HttpConnector>),
//}
//impl MyClient {
//    pub fn get(&self, uri: Uri) -> ResponseFuture {
//        match self {
//            Client (c) => c.get (uri),
//        }
//    }
//    pub fn post(&self, uri:Uri) -> ResponseFuture {
//        match self {
//            Client (c) => c.post (uri),
//        }
//    }
//}


// fn get_client(uri: Uri) -> MyClient { /* … */ }

fn gencred(username: String, password: String) -> BasicAuth {
    println!("{}", "generating credentials".green());
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}


fn main() {
    println!("{}", "starting up".yellow());
    let core = Core::new().unwrap();
    let handle = core.handle();

//    let host = Host::new("localhost", 8080);
//
//    let client = hyper::Client::configure()
//        .keep_alive(true)
//        .set_host(true)
//        .build(&handle);
//
//
//    let req = Request::builder()
//        .method("POST")
//        .uri("http://localhost:8080/")
//        .body(Body::from("Hallo!"))
//        .expect("request builder");
//
//    let future = client.request(req);
//
//    // auth
//    let mut headers = Headers::new();
//    headers.set(
//        Authorization(
//            Basic {
//                username: "admin".to_owned(),
//                password: "admin".to_owned()
//            }
//        )
//    );

    let hyperclient = Client::new(&handle);


    let auth = gencred("admin".to_owned(), "admin".to_owned());

    let configuration = Configuration {
        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
        client: hyperclient,
        basic_auth: Some(auth),
        oauth_access_token: None,
        api_key: None,
    };


//    let configuration = newconfig(hyperclient, "admin".to_owned(), "admin".to_owned());
    //let configuration = Configuration::new(hyperclient);

    //let auth = BasicAuth::from(("admin".to_owned(), Option("admin".to_owned())));

    //let configuration = newconfig(hyperclient, Option(auth));

    let client = APIClient::new(configuration);


    let mut wherevec: Vec<String> = Vec::new();
    wherevec.push(String::from("name"));

    let mut selectvec: Vec<String> = Vec::new();
    selectvec.push(String::from("default"));


    type ShowVpnResponse = Box<Future<Item=MsgVpnsResponse, Error=Error<serde_json::Value>>>;

    println!("{}", "Making request".green());
    let mut resp: ShowVpnResponse = client.msg_vpn_api().get_msg_vpns(10, "0", wherevec, selectvec);

    println!("{}", "Awaiting response".yellow());


    loop {
        match &resp.poll() {
            Ok(Async::NotReady) => println!("not ready"),
            Ok(Async::Ready(t)) => break,
            Err(e) => Err(e),
        };
    };


//    match resp.wait() {
//        Ok(_response) => {
//            println!("{}", "success".green());
//        }
//        Err(_) => {
//            println!("{}", "error".red());
//        }
//    }

    println!("Completed requests");

    //let resp: Box<Result<MsgVpnsResponse, Error=Error<serde_json::Value>>> = client.msg_vpn_api().get_msg_vpns(10, "0", wherevec, selectvec);

//    futures::future::ok(resp) {
//        Ok(json) => {
//            //pass
//        }
//    }
//
//    match resp {
//        ShowVpnResponse::Ok(_) => {
//
//        }
//        Err(_) => {
//            println!("{}", "Error getting vpn".red());
//        }
//    };


}
