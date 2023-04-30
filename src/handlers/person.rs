use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::{Identity};
use uuid::Uuid;
use std::collections::BTreeMap;
use tera::{Tera, Context};

use crate::{AppData, generate_basic_context};
use crate::errors::CustomError;
use crate::graphql::{get_people_by_name};

#[get("/{lang}/person_by_name/{name}")]
pub async fn person_by_name(
    data: web::Data<AppData>,
    id: Identity,
    web::Path((lang, name)): web::Path<(String, String)>,
    
    req:HttpRequest) -> impl Responder {

    let (mut ctx, user, lang, path) = generate_basic_context(id, &lang, req.uri().path());

    let people = get_people_by_name(name)
        .expect("Unable to get people");

    ctx.insert("people", &people.person_by_name);

    println!("{:?}", &ctx);

    let rendered = data.tmpl.render("person/person_by_name.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}