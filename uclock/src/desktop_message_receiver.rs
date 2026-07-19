use tokio::sync::broadcast;
use zbus::interface;

use crate::Message;

pub struct MessageReceiver {
    pub tx: broadcast::Sender<Message>,
}

#[interface(name = "com.utools.uclock")]
impl MessageReceiver {
    pub async fn toggle(&self) -> zbus::fdo::Result<()> {
        let _ = self.tx.send(Message::Toggle);
        Ok(())
    }
}
