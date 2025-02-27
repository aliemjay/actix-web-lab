# actix-web-lab

> Experimental extractors, middleware, and other extras for possible inclusion in Actix Web.

[![crates.io](https://img.shields.io/crates/v/actix-web-lab?label=latest)](https://crates.io/crates/actix-web-lab)
[![Documentation](https://docs.rs/actix-web-lab/badge.svg)](https://docs.rs/actix-web-lab/0.6.1)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/actix-web-lab.svg)
<br />
[![dependency status](https://deps.rs/crate/actix-web-lab/0.6.1/status.svg)](https://deps.rs/crate/actix-web-lab/0.6.1)
[![Download](https://img.shields.io/crates/d/actix-web-lab.svg)](https://crates.io/crates/actix-web-lab)
[![CircleCI](https://circleci.com/gh/robjtede/actix-web-lab/tree/main.svg?style=shield)](https://circleci.com/gh/robjtede/actix-web-lab/tree/main)

## Features
### Responders
- `Csv`: efficient CSV streaming [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/respond/struct.Csv.html)
- `NdJson`: efficient NDJSON streaming [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/respond/struct.NdJson.html)

### Middleware
- `from_fn`: use an async function as a middleware [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/middleware/fn.from_fn.html)

### Extractors
- `LazyData`: app data/state initialized on first use [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/extract/struct.LazyData.html)
- `Json`: simplified JSON extractor with const-generic limits [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/extract/struct.Json.html)

### Services
- `Redirect`: simple redirects [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/web/struct.Redirect.html)

### Route Guards
- `Acceptable`: verifies that an `Accept` header is present and it contains a compatible MIME type. [(docs)](https://docs.rs/actix-web-lab/latest/actix_web_lab/guard/struct.Acceptable.html)


## Things To Know About This Crate

- It will never reach v1.0.
- Minimum Supported Rust Version (MSRV) is latest stable at the time of each release.
- Breaking changes will likely happen on every 0.x version bump.
- Documentation will probably be limited for some items.
- Items that graduate to the main Actix Web crate will be deprecated for at least one minor
  version of this crate.
- It will often be as easy as dropping the `_lab` suffix from imports when items graduate.
