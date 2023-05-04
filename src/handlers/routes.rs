use actix_web::web;

use crate::handlers::{
    // base
    index,
    raw_index,

    //about,
    toggle_language,
    toggle_language_index,
    toggle_language_two,
    toggle_language_three,

    // admin
    // errors
    internal_server_error,
    not_found,

    // person
    person_by_id,
    person_by_name,

    // login
    login_form_input,
    login_handler,
    

};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(raw_index);

    // login
    config.service(login_handler);
    config.service(login_form_input);

    // person
    config.service(person_by_id);
    config.service(person_by_name);
    
    //config.service(about);
    config.service(toggle_language);
    config.service(toggle_language_index);
    config.service(toggle_language_two);
    config.service(toggle_language_three);

    // errors
    config.service(internal_server_error);
    config.service(not_found);

}
