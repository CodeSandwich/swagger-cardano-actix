extern crate actix_net;
extern crate actix_web;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use actix_net::server::Server;
use actix_web::{actix, server, App, FutureResponse, HttpResponse, Json, Query, Responder, State};
use actix_web::actix::Addr;
use actix_web::error::Error as ActixError;
use actix_web::server::StopServer;
use futures::{Async, Future, Poll};
use std::io::{BufRead, stdin};
use std::sync::{Arc, mpsc::sync_channel, Mutex};
use std::thread;

type CounterState = Arc<Mutex<u64>>;

fn main() {
    let state = Arc::new(Mutex::new(0));
    let server_handler = ServerHandler::start_server(state);
    stdin()
        .lock()
        .lines()
        .filter_map(|res| res.ok())
        .filter(|line| line == "stop")
        .next();
    server_handler.stop();
}

struct ServerHandler {
    addr: Addr<Server>,
}

impl ServerHandler {
    fn start_server(state: CounterState) -> Self {
        let (sender, receiver) = sync_channel::<Addr<Server>>(1);
        thread::spawn(move || {
            let system = actix::System::new("actix system");
            let server_addr = server::new(move || App::with_state(state.clone())
                .prefix("api")
                .scope("v1", |scope| scope
                    .resource("/next-update", |r| r.get().with(next_update_v1))
                    .resource("/restart-node", |r| r.get().with(restart_node_v1))
                    .resource("/counter", |r| r.get().with(counter_v1))
                ))
                .system_exit()
                .bind("127.0.0.1:8088")
                .unwrap()
                .start();
            sender.send(server_addr)
                .unwrap();
            system.run();
        });
        let addr = receiver.recv()
            .unwrap();
        ServerHandler { addr }
    }

    fn stop(self) {
        self.addr.send(StopServer { graceful: true })
            .wait()
            .unwrap()
            .unwrap();
    }
}

fn next_update_v1(_: ()) -> impl Responder {
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

struct CounterFuture {
    state: CounterState,
}

impl Future for CounterFuture {
    type Item = String;
    type Error = ActixError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut counter = self.state.lock()
            .unwrap_or_else(|e| e.into_inner());
        *counter += 1;
        let message = format!("Call no. {}", counter);
        println!("{}", message);
        Ok(Async::Ready(message))
    }
}

fn counter_v1(state: State<CounterState>) -> impl Responder {
    let future = CounterFuture {
        state: state.clone(),
    };
    Box::new(future) as FutureResponse<_>
}
