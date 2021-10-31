# Foxhole API

An unofficial rust wrapper for the [Foxhole War API](https://github.com/clapfoot/warapi).

[![MIT licensed][mit-badge]][mit-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs](https://docs.rs/foxhole-api/badge.svg)](https://docs.rs/foxhole-api)

[crates-badge]: https://img.shields.io/crates/v/foxhole-api.svg
[crates-url]: https://crates.io/crates/foxhole-api
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

## Dependencies

This library requires the use of [tokio](https://github.com/tokio-rs/tokio) as an async runtime.

## Usage

```rust
use foxhole_api::Client;

#[tokio::main]
async fn main() {
    // The default shard is Live-1
    let client = Client::default();

    let war_data = client.war_data().await.unwrap();
    let map_names = client.map_names().await.unwrap();
    let static_map_data = client.map_data_static("TheFingersHex".to_string()).await.unwrap();
    let dynamic_map_data = client.map_data_dynamic("TheFingersHex".to_string()).await.unwrap();
}
```

[Foxhole](https://www.foxholegame.com/) is a registered trademark of [Siege Camp](https://www.siegecamp.com/).
