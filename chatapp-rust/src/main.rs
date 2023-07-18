use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[handler]
async fn signup(req: &mut Request, res: &mut Response) {
    let user = req.parse_body::<User>().await;
    res.status_code(StatusCode::CREATED);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(Router::new().path("signup").post(signup));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}
