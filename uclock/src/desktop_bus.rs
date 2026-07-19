use crate::{desktop_message_receiver::MessageReceiver, DBUS_TX};

pub async fn init_dbus() {
    if let Ok(conn) = zbus::Connection::session().await {
        let iface = MessageReceiver {
            tx: DBUS_TX.clone(),
        };
        if conn
            .object_server()
            .at("/com/utools/uclock", iface)
            .await
            .is_ok()
        {
            let _ = conn.request_name("com.utools.uclock").await;
            std::future::pending::<()>().await;
        }
    }
}
