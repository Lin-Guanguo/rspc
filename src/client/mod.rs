mod error;

use error::ClientError;
use tokio::net;

struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
}
