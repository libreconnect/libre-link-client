
![GitHub last commit](https://img.shields.io/github/last-commit/libreconnect/libre-link-client)
![GitHub closed issues](https://img.shields.io/github/issues-closed/libreconnect/libre-link-client)
![GitHub contributors](https://img.shields.io/github/contributors/libreconnect/libre-link-client)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/libreconnect/libre-link-client)

# LibreLink Client

Welcome to the LibreLink Client project. This project is a Rust library that provides an interface for interacting with the LibreLink API, a platform that allows users to monitor and manage their glucose levels.

With LibreLink Client, you can easily retrieve glucose data, manage connections, and much more, all directly from your Rust code. Whether you're building a healthcare application, a research tool, or simply want to explore your own glucose data, LibreLink Client is here to help.

## Installation

### Quick Start

```sh
$ cargo add librelink-client
```

### Initialization

There are two ways to initialise the LibreLink client:
- From credentials
- From a token

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = LibreLinkClient::from_token(token);

  let data = client.get_glucose_history(2,3).await?;

  Ok(())
}
```

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibreLinkClient::new(Credentials {
      username: "username".to_string(),
      password: "password".to_string(),
    }).await;

    if let Ok(client) = client {
      let _ = client.get_glucose_history(2, 3).await?;
    }

    Ok(())
}
```

The creation of the client from credentials is asynchronous because the client must make an API call to retrieve the token that is required for the rest of the use of the `LibreLinkClient`.

## Documentation

Checkout our [documentation crates.io](https://docs.rs/librelink-client/0.1.1/librelink_client/index.html).