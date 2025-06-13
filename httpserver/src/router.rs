use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};

use tokio::io::AsyncWriteExt;
pub struct Router;

impl Router {
    pub async fn route(
        req: HttpRequest,
        stream: &mut (impl AsyncWriteExt + Unpin),
    ) -> Result<(), Box<dyn std::error::Error>> {
        match req.method {
            http::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            resp.send_response_async(stream).await?;
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            resp.send_response_async(stream).await?;
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                resp.send_response_async(stream).await?;
            }
        }
        Ok(())
    }
}
