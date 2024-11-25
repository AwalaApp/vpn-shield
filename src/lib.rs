use worker::*;

#[event(fetch)]
async fn main(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
