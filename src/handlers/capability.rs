use actix_session::UserSession;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web, ResponseError};
use actix_identity::Identity;
use crate::{AppData, generate_basic_context};

use super::CapabilityForm;

#[post("/{lang}/capability_search")]
pub async fn capability_search(
    web::Path(lang): web::Path<String>,
    _data: web::Data<AppData>,
    req: HttpRequest, 
    form: web::Form<CapabilityForm>,
    id: Identity,
) -> impl Responder {

    let (mut ctx, user, lang, path) = generate_basic_context(id, &lang, req.uri().path());

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    // validate form has data or return to index
    if form.name.is_empty() || form.level.is_empty() {
        println!("Form is empty");
        return HttpResponse::Found().header("Location", format!("/{}/", &lang)).finish()
    };
    
    // query graphql API
    let results = graphql::capability_search(
        form.name.to_lowercase().trim().to_string(),
        form.level.to_uppercase().to_string(),
    )
    .expect("Unable to find capabilities");
             
    ctx.insert("capabilities", &results.capability_search_results);
    ctx.insert("name", name);
    ctx.insert("level", level);

    let rendered = data.tmpl.render("capability/capability_search_results.html", &ctx).unwrap();
    HttpResponse::Ok()
        .header("Bearer", bearer)
        .body(rendered)
}