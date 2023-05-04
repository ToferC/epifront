// example auth: https://github.com/actix/actix-extras/blob/master/actix-identity/src/lib.rs

use std::{env, sync::Mutex};

use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use actix_session::{Session, UserSession};
use actix_identity::{Identity};

use crate::{AppData, generate_basic_context, extract_identity_data, APP_NAME, graphql};

use super::{EmailForm, LoginForm, RegisterForm, VerifyForm, PasswordForm};

#[get("/{lang}/log_in")]
pub async fn login_handler(
    web::Path(lang): web::Path<String>,
    data: web::Data<AppData>,
    
    req:HttpRequest,
    id: Identity,
) -> impl Responder {

    let (ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let rendered = data.tmpl.render("authentication/log_in.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/log_in")]
pub async fn login_form_input(
    web::Path(lang): web::Path<String>,
    data: web::Data<Mutex<AppData>>,
    _req: HttpRequest, 
    form: web::Form<LoginForm>,
    _session: Session,
    id: Identity,
) -> impl Responder {

    // validate form has data or re-load form
    if form.email.is_empty() || form.password.is_empty() {
        println!("Form is empty");
        return HttpResponse::Found().header("Location", format!("/{}/log_in", &lang)).finish()
    };
    
    let bearer = graphql::login(form.email.to_lowercase().trim().to_string(), form.password.clone())
        .expect("Unable to get login").sign_in;

    // Add received string to AppData
    data.lock().unwrap().bearer = bearer.clone();
             
    return HttpResponse::Found()
        .header("Location", "/")
        .header("Bearer", bearer)
        .finish()
}

#[get("/{lang}/log_out")]
pub async fn logout(
     web::Path(lang): web::Path<String>,
    _data: web::Data<AppData>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    println!("Handling Post Request: {:?}", req);

    let session = req.get_session();

    session.clear();
    id.forget();

    HttpResponse::Found().header("Location", format!("/{}", &lang)).finish()
}