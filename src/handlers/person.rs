use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::{Identity};


use crate::{AppData, generate_basic_context};
use crate::graphql::{get_people_by_name, get_person_by_id};

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

    let rendered = data.tmpl.render("person/person_by_name.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[get("/{lang}/person/{person_id}")]
pub async fn person_by_id(
    data: web::Data<AppData>,
    id: Identity,
    web::Path((lang, person_id)): web::Path<(String, String)>,
    
    req:HttpRequest) -> impl Responder {

    let (mut ctx, user, lang, path) = generate_basic_context(id, &lang, req.uri().path());

    let r = get_person_by_id(person_id)
        .expect("Unable to get people");

    ctx.insert("person", &r.person_by_id);

    let rendered = data.tmpl.render("person/person.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}