// Copyright 2021 Oxide Computer Company
/*!
 * Example use of Dropshot for matching wildcard paths to serve static content.
 */

use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use dropshot::{endpoint, Path, Query};
use http::{Response, StatusCode};
use hyper::Body;
use schemars::JsonSchema;
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), String> {
    /*
     * We must specify a configuration with a bind address.  We'll use 127.0.0.1
     * since it's available and won't expose this server outside the host.  We
     * request port 0, which allows the operating system to pick any available
     * port.
     */
    let mut config_dropshot: ConfigDropshot = Default::default();

    let socket_address = SocketAddrV4::new("127.0.0.1".parse().unwrap(), 3987);

    config_dropshot.bind_address = socket_address.into();

    /*
     * For simplicity, we'll configure an "info"-level logger that writes to
     * stderr assuming that it's a terminal.
     */
    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Critical,
    };
    let log = config_logging
        .to_logger("example-basic")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    /*
     * Build a description of the API.
     */
    let mut api = ApiDescription::new();
    api.register(index).unwrap();

    /*
     * Set up the server.
     */
    let server = HttpServerStarter::new(&config_dropshot, api, (), &log)
        .map_err(|error| format!("failed to create server: {}", error))?
        .start();

    /*
     * Wait for the server to stop.  Note that there's not any code to shut down
     * this server, so we should never get past this point.
     */
    server.await
}

#[derive(Deserialize, JsonSchema)]
struct PathParams {
    path_part: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    query: String
}

/**
 * Return static content.for all paths.
 */
#[endpoint {
    method = GET,
    path = "/{path_part}"
}]
async fn index(
    _rqctx: &RequestContext<()>,
    path: Path<PathParams>,
    query: Query<QueryParams>
) -> Result<Response<Body>, HttpError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/html")
        .body(
            format!(
                "<HTML><HEAD>Hello, {} {}!</HEAD></HTML>",
                path.into_inner().path_part,
                query.into_inner().query
            )
            .into(),
        )?)
}