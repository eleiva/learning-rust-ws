mod router;

use log::Level::Error as ErrorLevel;
use log::{debug, error, log_enabled, trace};
use ws::util::{Timeout, Token};
use ws::{CloseCode, Error, Handler, Handshake, Message, OpCode, Sender, WebSocket};

struct Server {
    out: Sender,
    ping_timeout: Option<Timeout>,
    client_unresponsive_timeout: Option<Timeout>,
}

const PING: Token = Token(0);
const CLIENT_UNRESPONSIVE: Token = Token(1);

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        self.out.timeout(5_000, PING)
    }

    fn on_shutdown(&mut self) {
        debug!("Handler received WebSocket shutdown request.");
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {

        debug!("Received message {:?}", msg);

        let data = crate::router::route(msg.to_string());
        
        let json_data = serde_json::to_string(&data).unwrap();

        let message = Message::text(json_data);

        debug!("Sent  {:?}", message);

        let _ = self.out.send(message);

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        debug!("Connection closing due to ({:?}) {}", code, reason);

        if let Some(timeout) = self.ping_timeout.take() {
            self.out.cancel(timeout).unwrap()
        }
    }

    fn on_error(&mut self, err: ws::Error) {
        // Ignore connection reset errors by default, but allow library clients to see them by
        // overriding this method if they want
        if let ws::ErrorKind::Io(ref err) = err.kind {
            if let Some(104) = err.raw_os_error() {
                return;
            }
        }

        error!("{:?}", err);
        if !log_enabled!(ErrorLevel) {
            println!(
                "Encountered an error: {}\nEnable a logger to see more information.",
                err
            );
        }
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        debug!("Handler received request:\n{}", req);
        ws::Response::from_request(req)
    }

    fn on_response(&mut self, res: &ws::Response) -> ws::Result<()> {
        debug!("Handler received response:\n{}", res);
        Ok(())
    }

    fn on_timeout(&mut self, event: ws::util::Token) -> ws::Result<()> {
        debug!("Handler received timeout token: {:?}", event);
        match event {
            PING => {
                println!("Pinging the client");
                self.out.ping("".into())?;
                self.out.timeout(5_000, PING)
            }
            CLIENT_UNRESPONSIVE => {
                println!("Client is unresponsive, closing the connection");
                self.client_unresponsive_timeout.take();

                if let Some(timeout) = self.ping_timeout.take() {
                    println!("timeout: {:?}", timeout);
                    self.out.cancel(timeout)?;
                    println!("canceled");
                }
                self.out.close(CloseCode::Away)
            }
            _ => Err(Error::new(
                ws::ErrorKind::Internal,
                "Invalid timeout token encountered!",
            )),
        }
    }

    fn on_new_timeout(
        &mut self,
        event: ws::util::Token,
        timeout: ws::util::Timeout,
    ) -> ws::Result<()> {
        // default implementation discards the timeout handle
        match event {
            PING => {
                if let Some(timeout) = self.ping_timeout.take() {
                    self.out.cancel(timeout)?
                }

                match self.client_unresponsive_timeout {
                    Some(_) => self.ping_timeout = Some(timeout),
                    None => self.ping_timeout = None,
                }
            }

            CLIENT_UNRESPONSIVE => {
                if let Some(timeout) = self.client_unresponsive_timeout.take() {
                    self.out.cancel(timeout)?
                }
                self.client_unresponsive_timeout = Some(timeout)
            }

            _ => {
                debug!("Unknown event: {:?}", event)
            }
        }

        Ok(())
    }

    fn on_frame(&mut self, frame: ws::Frame) -> ws::Result<Option<ws::Frame>> {
        debug!("Handler received: {}", frame);

        if frame.opcode() == OpCode::Pong {
            debug!("Received a pong");
            // Reset the CLIENT_UNRESPONSIVE timeout
            self.out.timeout(15_000, CLIENT_UNRESPONSIVE)?;
        }

        // default implementation doesn't allow for reserved bits to be set
        if frame.has_rsv1() || frame.has_rsv2() || frame.has_rsv3() {
            Err(ws::Error::new(
                ws::ErrorKind::Protocol,
                "Encountered frame with reserved bits set.",
            ))
        } else {
            Ok(Some(frame))
        }
    }

    fn on_send_frame(&mut self, frame: ws::Frame) -> ws::Result<Option<ws::Frame>> {
        trace!("Handler will send: {}", frame);
        // default implementation doesn't allow for reserved bits to be set
        if frame.has_rsv1() || frame.has_rsv2() || frame.has_rsv3() {
            Err(ws::Error::new(
                ws::ErrorKind::Protocol,
                "Encountered frame with reserved bits set.",
            ))
        } else {
            Ok(Some(frame))
        }
    }

    fn build_request(&mut self, url: &url::Url) -> ws::Result<ws::Request> {
        trace!("Handler is building request to {}.", url);
        ws::Request::from_url(url)
    }
}

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:3012";

    println!("Ws running at {}", addr);

    let server = WebSocket::new(|out| Server {
        out,
        ping_timeout: None,
        client_unresponsive_timeout: None,
    })
    .unwrap();

    server.listen(addr).unwrap();

}
