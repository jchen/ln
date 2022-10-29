use super::*;

/**
 * Polls KV for a given param and returns the corresponding redirect.
 */
pub async fn poll(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug");
    let kv = ctx.kv(globals::URL_KV)?;
    match slug {
        Some(slug) => {
            let url = kv.get(slug).text().await?;
            match url {
                Some(url) => {
                    let url = Url::parse(&url);
                    match url {
                        Ok(url) => Response::redirect(url),
                        Err(_) => Response::ok("Invalid URL"),
                    }
                }
                None => globals::missing_redirect(),
            }
        }
        None => Response::ok("No slug was given"),
    }
}

/**
 * Handles the creation of a new link
 */
pub async fn create(req: Request, ctx: RouteContext<()>) -> Result<String> {
    let url = req.url()?;
    let mut query_params = url.query_pairs();

    // Get URLs and reverse-URLs KV
    let kv = ctx.kv(globals::URL_KV)?;
    let reverse_kv = ctx.kv(globals::REVERSE_URL_KV)?;

    // Finds the url=___ parameter
    let dest_url = query_params
        .find(|(k, _)| k == globals::URL_KEY)
        .map(|(_, v)| v.to_string());

    let dest_stub = query_params
        .find(|(k, _)| k == globals::STUB_KEY)
        .map(|(_, v)| v.to_string())
        .filter(|s| !s.is_empty());

    match dest_url {
        Some(dest_url) => {
            // Sees if the URL is already in the KV
            let existing_stub = reverse_kv.get(&dest_url).text().await?;
            match existing_stub {
                Some(stub) => {
                    match dest_stub {
                        Some(dest_stub) => {
                            // If the URL is already in the KV, we update to the new stub (preserving the old one), and update the reverse lookup
                            // This retains previously created links, but allows us to overwrite them.
                            let current_link = kv.get(&dest_stub).text().await?;
                            if let Some(current_link) = current_link {
                                // Overwrite the existing link if this stub exists even (this gives us ability to edit existing stubs)
                                kv.delete(&dest_stub).await?;
                                reverse_kv.delete(&current_link).await?;
                            }
                            // Puts the new stub -> url
                            kv.put(&dest_stub, &dest_url)?.execute().await?;
                            // Changes the canonical stub for url to be dest_stub
                            reverse_kv.put(&dest_url, &dest_stub)?.execute().await?;
                            Ok(dest_stub)
                        }
                        None => {
                            // If the URL is already in the KV, we just return the stub
                            Ok(stub)
                        }
                    }
                }
                None => {
                    let url = Url::parse(&dest_url);
                    match url {
                        Ok(_) => {
                            let mut stub = dest_stub.unwrap_or_else(utils::create_stub);
                            // If stub already exists, generate a new one
                            while kv.get(&stub).text().await?.is_some() {
                                stub = utils::create_stub();
                            }
                            kv.put(&stub, &dest_url)?.execute().await?;
                            reverse_kv.put(&dest_url, &stub)?.execute().await?;
                            Ok(stub)
                        }
                        Err(_) => Err(Error::from("A valid URL was not provided")),
                    }
                }
            }
        }
        None => Err(Error::from("No URL was provided")),
    }
}

/**
 * Lists all the links in the KV
 */
pub async fn list(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("links")
}
