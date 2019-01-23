extern crate actix_net;
extern crate actix_web;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use actix_net::server::Server;
use actix_web::{actix, server, App, HttpRequest, HttpResponse, Json, Query, Responder};
use actix_web::actix::Addr;
use actix_web::server::{HttpServer, PauseServer, ResumeServer, StopServer};
use futures::Future;
use std::io::{BufRead, stdin};
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    let server_addr_1 = start_server();
    for line in stdin().lock().lines() {
        match line.as_ref().map(|s| s.as_ref()) {
            Ok("pause 1") => println!("{:?}", server_addr_1.send(PauseServer).wait()),
            Ok("resume 1") => println!("{:?}", server_addr_1.send(ResumeServer).wait()),
            Ok("stop 1") => println!("{:?}", server_addr_1.send(StopServer { graceful: true }).wait()),
            _ => println!("Unknown command"),
        }
    }
}

fn start_server() -> Addr<Server> {
    let (sender, receiver) = sync_channel::<Addr<Server>>(1);
    thread::spawn(move || {
        let system = actix::System::new("no idea what's the point");
        let server_addr = server::new(|| App::new()
            .prefix("api")
            .scope("v1", |scope| scope
                .resource("/next-update", |r| r.get().f(next_update_v1))
                .resource("/restart-node", |r| r.get().with(restart_node_v1))
            ))
            .system_exit()
            .bind("127.0.0.1:8088")
            .unwrap()
            .start();
        sender.send(server_addr)
            .unwrap();
        system.run();
    });
    receiver.recv()
        .unwrap()
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
