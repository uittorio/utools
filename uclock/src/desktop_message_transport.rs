use once_cell::sync::Lazy;
use tokio::sync::broadcast;

use crate::message::Message;

// why do we use broadcast and why static?
// static -> because the cosmic subscription system requires to access to the "sender" from a closure that run outside the app context.
// broadcast -> the subscription is called multiple times acting as multiple receivers and the standard mscp will cause one receiver to block the subcription fn.
pub static DBUS_TX: Lazy<broadcast::Sender<Message>> = Lazy::new(|| {
    let (tx, _rx) = broadcast::channel(10);
    tx
});
