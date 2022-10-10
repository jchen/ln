pub use super::*;

pub async fn api_create(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let stub = create(req, ctx).await?;
    Response::ok(stub)
}
