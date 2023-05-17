use actix_session::UserSession;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::{Identity};


use crate::{AppData, generate_basic_context};
use crate::graphql::{get_role_by_id};

#[get("/{lang}/role/{role_id}")]
pub async fn role_by_id(
    data: web::Data<AppData>,
    id: Identity,
    web::Path((lang, role_id)): web::Path<(String, String)>,
    
    req:HttpRequest) -> impl Responder {

    let (mut ctx, user, lang, path) = generate_basic_context(id, &lang, req.uri().path());

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let r = get_role_by_id(role_id, bearer)
        .expect("Unable to get people");

    ctx.insert("role", &r.role_by_id);

    let rendered = data.tmpl.render("role/role.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}