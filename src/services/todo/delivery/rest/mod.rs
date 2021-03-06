use crate::config::Config;
use crate::helpers::handler;
use crate::services::todo::controller::TodoController;

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct TodoRest {
    pub cnfg: Arc<Config>,
    pub todo_cnr: Arc<TodoController>,
}

pub fn init(cnfg: &Arc<Config>, todo_cnr: &Arc<TodoController>) -> Scope {
    let todo = TodoRest {
        cnfg: cnfg.clone(),
        todo_cnr: todo_cnr.clone(),
    };
    web::scope("/todo")
        .data(todo)
        .route("/info", web::get().to(info))
        .route("/send", web::get().to(send_mail))
        .route("/add", web::post().to(add_todo))
        .route("/complete", web::post().to(complete_todo))
}

fn info(data: web::Data<TodoRest>) -> HttpResponse {
    let info = data.todo_cnr.todo_info();
    let res = format!("Todo info: {}", info);
    handler::to_json(Ok(res))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct SendMailParams {
    #[validate(email)]
    email: String,
    #[validate(length(min = "1", max = "64"))]
    template_id: String,
}

fn send_mail(params: web::Query<SendMailParams>, data: web::Data<TodoRest>) -> HttpResponse {
    crate::validate_errors!(params);
    data.todo_cnr
        .send_mail(params.email.clone(), params.template_id.clone());
    handler::to_json(Ok("Mail sent"))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct AddTodoBody {
    #[validate(length(min = "1", max = "100"))]
    description: String,
}

fn add_todo(body: web::Json<AddTodoBody>, data: web::Data<TodoRest>) -> HttpResponse {
    crate::validate_errors!(body);
    let id = data.todo_cnr.add_todo(body.description.clone());
    let res = format!("Todo added: {}", id);
    handler::to_json(Ok(res))
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteTodoReq {
    id: uuid::Uuid,
}

fn complete_todo(req: web::Json<CompleteTodoReq>, data: web::Data<TodoRest>) -> HttpResponse {
    data.todo_cnr.complete_todo(req.id);
    handler::to_json(Ok("Todo completed"))
}
