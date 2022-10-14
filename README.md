<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/postman-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/postman-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/postman-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/postman-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/postman-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/postman-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/postman">
    <img src="https://img.shields.io/crates/d/postman?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/postman">
    <img src="https://img.shields.io/crates/v/postman?style=flat-square" alt="Crates.io" />
</a>

</p>

Postman client, generated from the OpenAPI spec.

# Usage

```rust
use postman::PostmanClient;
use postman::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let response = client
        .get_all_apis()
        .workspace("your workspace")
        .since("your since")
        .until("your until")
        .created_by("your created by")
        .updated_by("your updated by")
        .is_public(true)
        .name("your name")
        .summary("your summary")
        .description("your description")
        .sort("your sort")
        .direction("your direction")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `POSTMAN_API_KEY`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
postman = "1.0.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/postman)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*