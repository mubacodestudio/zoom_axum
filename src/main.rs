use std::{fs, sync::Arc};

use axum::{
    extract::Path,
    response::{Html, Redirect},
    routing::get,
    Router,
};
use dotenv::dotenv;
use minijinja::{context, Environment};
use tokio::{net::TcpListener, sync::RwLock};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{handshake::server::Request, Message},
    WebSocketStream,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");
    let base_url = Arc::new(base_url);

    let app = Router::new()
        .route("/", get(move || redirect_handler(base_url.clone())))
        .route("/:room", get(get_html));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn redirect_handler(base_url: Arc<String>) -> Redirect {
    let id = Uuid::new_v4();
    Redirect::to(&format!("{}{}", base_url, id))
}

async fn get_html(Path(room): Path<String>) -> Html<String> {
    let template_content =
        fs::read_to_string("src/profile.html").expect("Failed to read template file");

    let mut env = Environment::new();
    env.add_template("profile", &template_content)
        .expect("Failed to add template");
    let rendered = env
        .get_template("profile")
        .expect("Template not found")
        .render(context! { room })
        .expect("Failed to render template");
    Html(rendered)
}
