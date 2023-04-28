use actix_web::{HttpServer, App, middleware};
use dotenv::dotenv;
use std::env;
use tera::{Tera};
use tera_text_filters::snake_case;
use actix_identity::{IdentityService, CookieIdentityPolicy};
use actix_web_static_files;
use sendgrid::SGClient;

use epifront::handlers;
use epifront::AppData;
use crate::handlers::get_people_by_name;

use fluent_templates::{FluentLoader, static_loader};
// https://lib.rs/crates/fluent-templates

// Setup for serving static files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static_loader! {
    static LOCALES = {
        locales: "./i18n/",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8088"))
    };

    let cookie_secret_key = env::var("COOKIE_SECRET_KEY").expect("Unable to find secret key");

    let r = get_people_by_name("emi".to_string());

    println!("QUERY RESULT: {:?}", r);

    HttpServer::new(move || {
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto-reload with Tera");
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));

        let data = AppData {
            tmpl: tera,
        };

        let generated = generate();

        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::configure_services)
            .data(data.clone())
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_secret_key.as_bytes())
                .name("user-auth")
                .secure(false)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
