# Rust Solace Client

    docker-compose up -d
    # wait for solace to come up
    cargo run -- --config config.yaml provision.yaml

# Bugs

It is impossible to create an EventThreshold because you MUST set all values, and setting Both clearValue and clearPercent
results in a error from the appliance.

    "Conflicting attribute \"clearValue\" used with \"clearPercent\"."


## References

https://docs.solace.com/API-Developer-Online-Ref-Documentation/swagger-ui/index.html
https://github.com/swagger-api/swagger-codegen/blob/master/samples/client/petstore/rust/examples/client.rs