pub use super::*;

const STUB_MACRO: &str = r#"!STUB"#;
const TARGET_MACRO: &str = r#"!TARGET"#;

/**
 * A prettified creation screen
 */
pub async fn pretty_create(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let mut query_params = url.query_pairs();
    let dest_url = query_params
        .find(|(k, _)| k == globals::URL_KEY)
        .map(|(_, v)| v.to_string());

    match dest_url {
        Some(dest_url) => {
            let stub = create(req, ctx).await;
            match stub {
                Ok(stub) => {
                    let content = include_str!("static/ln.html")
                        .to_string()
                        .replace(STUB_MACRO, &stub)
                        .replace(TARGET_MACRO, &dest_url);
                    Response::from_html(content)
                }
                Err(e) => Response::ok(format!("{}", e)),
            }
        }
        None => Response::from_html(include_str!("static/create.html")),
    }
}
