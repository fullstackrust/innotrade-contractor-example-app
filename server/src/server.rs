extern crate actix_web;
use self::actix_web::{fs, HttpRequest, HttpResponse, Responder};

use isomorphic_app::App;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const CSS_PLACEHOLDER: &str = "#CSS_PATH#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

fn index(req: &HttpRequest) -> impl Responder {
    let app = App::new(
        req.query()
            .get("init")
            .map(|string| string.parse().expect("bad param"))
            .unwrap_or(1001),
    );
    let state = app.store.borrow();

    let html = format!("{}", include_str!("./index.html"));
    let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
    let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

    // Development
    #[cfg(debug_assertions)]
    let html = html.replacen(CSS_PLACEHOLDER, "app.css", 1);

    // Production
    #[cfg(not(debug_assertions))]
    let html = html.replacen(CSS_PLACEHOLDER, "app.min.css", 1);

    HttpResponse::Ok().content_type("text/html").body(html)
}

pub fn serve() {
    let server = actix_web::server::new(|| {
        let app = actix_web::App::new();
        let app = app.resource("/", |r| r.f(index));

        // Development
        #[cfg(debug_assertions)]
        let app = app.handler("/", fs::StaticFiles::new("../client/build").unwrap());

        // Production
        #[cfg(not(debug_assertions))]
        let app = app.handler("/", fs::StaticFiles::new("../client/dist").unwrap());

        app
    });

    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let server = server.bind("0.0.0.0:7878").unwrap();

    println!("Listening on port 7878");
    server.run();
}
