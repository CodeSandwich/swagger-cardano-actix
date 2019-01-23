extern crate actix_web;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use actix_web::{server, App, HttpRequest, HttpResponse, Json, Query, Responder};

fn main() {
    server::new(|| App::new()
            .prefix("api")
            .scope("v1", |scope| scope
                .resource("/next-update", |r| r.get().f(next_update_v1))
                .resource("/restart-node", |r| r.get().with(restart_node_v1))
            ))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

fn next_update_v1(_: &HttpRequest) -> impl Responder {
    Json(json!({
          "data": {
            "applicationName": "string",
            "version": 0
          },
          "meta": {
            "pagination": {}
          },
          "status": "success"
        }))
}

#[derive(Deserialize)]
struct RestartNodeV1Params {
    #[serde(default)]
    pub force_ntp_check: bool,
}

fn restart_node_v1(params: Query<RestartNodeV1Params>) -> impl Responder {
    println!("Restart! force_ntp_check = {}", params.force_ntp_check);
    HttpResponse::Ok()
}
