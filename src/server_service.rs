use crate::{Error, ServerResult};
use actix_net::server::Server;
use actix_web::{actix, server};
use actix_web::actix::Addr;
use actix_web::server::{IntoHttpHandler, StopServer};
use futures::Future;
use std::net::ToSocketAddrs;
use std::sync::mpsc::sync_channel;
use std::thread;

pub struct ServerService {
    addr: Addr<Server>,
}

impl ServerService {
    pub fn start<A, F, H>(address: A, handler: F) -> ServerResult<Self>
        where A: ToSocketAddrs + Send + 'static,
              F: Fn() -> H + Send + Clone + 'static,
              H: IntoHttpHandler + 'static {
        let (sender, receiver) = sync_channel::<ServerResult<ServerService>>(0);
        thread::spawn(move || {
            let actix_system = actix::System::new("actix system");
            let server_handler = start_server_curr_actix_system(address, handler);
            let _ = sender.send(server_handler);
            actix_system.run();
        });
        receiver.recv().unwrap()
    }

    pub fn stop(self) {
        self.addr.send(StopServer { graceful: true })
            .wait()
            .unwrap()
            .unwrap();
    }
}

fn start_server_curr_actix_system<F, H>(address: impl ToSocketAddrs, handler: F) -> ServerResult<ServerService>
    where F: Fn() -> H + Send + Clone + 'static, H: IntoHttpHandler + 'static {
    let addr = server::new(handler)
        .system_exit()
        .bind(address)
        .map_err(Error::from_bind_error)?
        .start();
    Ok(ServerService { addr })
}
