use actix_web::{middleware, get, web, App, HttpServer, Responder, HttpRequest, cookie::Key};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
mod views;
mod schema;
mod data;

async fn index(req: HttpRequest, session: Session) -> &'static str {
    println!("REQ: {req:?}");
    if let Ok(Some(count)) = session.get::<i32>("counter") {
        session.insert("counter", count + 1);
    } else {
        session.insert("counter", 1);
    }
    "Hello world!";
    println!("seesssion {:#?}", session.get::<i32>("counter").unwrap());
    "REQ: {req:?}"
    
}

#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    

    println!("Hello, world!");
    // closure new to insert app
    HttpServer::new(|| {
        let app = App::new()
        .wrap(middleware::Logger::default())
        .wrap(
            // create cookie based session middleware
            SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .build()
        )
        .configure(views::views_factory);
    
        return app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
