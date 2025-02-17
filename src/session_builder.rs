use crate::connection::{HeartBeat, OwnedCredentials};
use crate::header::{Header, HeaderList};
use crate::option_setter::OptionSetter;

use crate::session::Session;
use std::io;
use std::net::ToSocketAddrs;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;

#[derive(Clone)]
pub struct SessionConfig {
    pub host: String,
    pub port: u16,
    pub credentials: Option<OwnedCredentials>,
    pub heartbeat: HeartBeat,
    pub headers: HeaderList,
}

pub struct SessionBuilder {
    pub config: SessionConfig,
}

impl SessionBuilder {
    pub fn new(host: &str, port: u16) -> SessionBuilder {
        let config = SessionConfig {
            host: host.to_owned(),
            port: port,
            credentials: None,
            heartbeat: HeartBeat(0, 0),
            headers: header_list![
             "host" => host,
             "accept-version" => "1.2",
             "content-length" => "0"
            ],
        };
        SessionBuilder { config: config }
    }

    #[allow(dead_code)]
    pub fn start<'b, 'c>(self, hdl: Handle) -> ::std::io::Result<Session> {
        let address = (&self.config.host as &str, self.config.port)
            .to_socket_addrs()?
            .nth(0)
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "address provided resolved to nothing",
            ))?;
        Ok(Session::new(
            self.config,
            TcpStream::connect(&address, &hdl),
            hdl,
        ))
    }

    #[allow(dead_code)]
    pub fn with<'b, T>(self, option_setter: T) -> SessionBuilder
    where
        T: OptionSetter<SessionBuilder>,
    {
        option_setter.set_option(self)
    }
}
