use worker::*;
mod globals;
mod utils;

mod actions;
pub use actions::*;
mod api;
pub use api::*;
mod pretty;
pub use pretty::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);

    let router = Router::new();
    router
        .get("/", |_, _| globals::default_redirect())
        .get_async("/api/pretty/create", pretty_create)
        .get_async("/api/create", api_create)
        .get_async("/api/list", list)
        .get_async("/:slug", poll)
        .run(req, env)
        .await
}
