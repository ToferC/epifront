use actix_session::UserSession;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web, ResponseError};
use actix_identity::Identity;
use crate::{AppData, generate_basic_context};
use crate::graphql::get_capability_by_name_and_level;


use super::CapabilityForm;

#[post("/{lang}/capability_search")]
pub async fn capability_search(
    web::Path(lang): web::Path<String>,
    data: web::Data<AppData>,
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
    if form.name.is_empty() {
        println!("Form is empty");
        return HttpResponse::Found().header("Location", format!("/{}/", &lang)).finish()
    };
    
    // query graphql API
    let results = get_capability_by_name_and_level(
        form.name.to_lowercase().trim().to_string(),
        form.level.clone(),
        bearer.clone(),
    )
    .expect("Unable to find capabilities");
             
    ctx.insert("capabilities", &results.capabilities_by_name_and_level);
    ctx.insert("name", &form.name.to_owned());
    ctx.insert("level", &form.level);

    let rendered = data.tmpl.render("capability/capability_search_results.html", &ctx).unwrap();
    HttpResponse::Ok()
        .header("Bearer", bearer)
        .body(rendered)
}