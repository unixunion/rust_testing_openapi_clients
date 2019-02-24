# Solace Provision

This solace-provision tool is written in Rust, and is basically under construction. 

## Status

    Provision and Update for VPN's,

## Requirements

* Solace PubSub+ or SolOS-TR Appliance
* Solace's SEMP service running in TLS mode

## Local Development

### Start Solace

    docker-compose up -d
    
### TLS
    
Once the appliance is up, TLS must be enabled for SEMP. A rootCA and localhost cert is available under (certs/)[certs/], 
or you can follow Solace's documentation for setting it up.

* Configure TLS for SEMP: https://docs.solace.com/Configuring-and-Managing/TLS-SSL-Service-Connections.htm#managing_tls_ssl_service_1762742558_317096
* Generating CA and Certs: https://gist.github.com/fntlnz/cf14feb5a46b2eda428e000157447309
* You can run the CA+Cert commands in /usr/sw/jail/certs on the router, access it with `docker-compose exec solace bash`
* Combine the server.crt and server.key into a single pem `cat localhost.crt localhost.key >>localhost.pem`
* enable TLS for SEMP as described in Solace Docs
* add rootca cert on client host system which will run this code. e.g: keychain import into System chain on mac.

Testing TLS:

    curl -k --cacert ./certs/rootCa.crt https://localhost:8080/SEMP/v2/config 

# Compiling

    cargo build --release

# Provision / Update VPN

`solace-provision` can <i>create</i> or <i>update</i> existing VPN's. Running without `--update` assumes "create" behaviour. See `solace-provision --help` 
for more info.

## Configuring API Client

See [config.yaml](config.yaml) for appliance connection properties. Pass the confif file with; `--config config.yaml`

## VPN Provision Document

The vpn.yaml example contains all the possibile keys and values settable. 
See [vpn.yaml](vpn.yaml)
    
## Running

    solace-provision --config config.yaml --vpn vpn.yaml [--update]


## References

https://docs.solace.com/API-Developer-Online-Ref-Documentation/swagger-ui/index.html
https://github.com/swagger-api/swagger-codegen/blob/master/samples/client/petstore/rust/examples/client.rs

