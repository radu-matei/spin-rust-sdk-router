use spin_sdk::{http_component, http::{Request, Response}};
use spin_sdk_router::{router, Params};

#[http_component]
fn handle_example(req: Request) -> anyhow::Result<Response> {
    (router! {
        GET "/hello/:planet" => api::hello_planet,
        _   "/*"             => |_req, params| {
            let capture = params.wildcard().unwrap_or_default();
            Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .body(Some(format!("{capture}").into()))
                .unwrap())
        }
    })(req)
}

mod api {
    use super::*;

    // /hello/:planet
    pub fn hello_planet(_req: Request, params: Params) -> anyhow::Result<Response> {
        let planet = params.get("planet").expect("PLANET");

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(format!("{planet}").into()))
            .unwrap())
    }
}