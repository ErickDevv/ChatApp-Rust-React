use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

//TODO: Read from env variable or some file.
const SECRET_KEY: &str = "";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    user_id: String,
    exp: i64,
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

#[handler]
async fn signup(req: &mut Request, res: &mut Response) {
    let token = match req.parse_body::<User>().await {
        Ok(user) => {
            let token = generate_token(user.username.as_str());
            token
        }
        Err(_err) => {
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

    let render_token = match token {
        Ok(token) => Token { token },
        Err(_err) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };

    res.render(Json(render_token));
}

fn generate_token(user_id: &str) -> Result<String, String> {
    let claims = JwtClaims {
        user_id: user_id.to_owned(),
        exp: (OffsetDateTime::now_utc() + Duration::days(15)).unix_timestamp(),
    };

    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    ) {
        Ok(token) => token,
        Err(err) => return Err(err.to_string()),
    };

    Ok(token)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(Router::new().path("signup").post(signup));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}
