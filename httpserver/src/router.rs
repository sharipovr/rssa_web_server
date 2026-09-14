// Router module inspects the incoming HTTP requests and figures out which handler to use
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // If GET request
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // Parse the URI
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // if the route begins with /api, invoke web service
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // Else invoke static page handler
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // If method is not GET request, return 404 page
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
