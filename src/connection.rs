use bb8::ManageConnection;
use tokio::net;

#[derive(Clone)]
pub struct KafkaConnectionManager {

}

impl ManageConnection for KafkaConnectionManager {
    type Connection = net::TcpStream;
    type Error = ();

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        todo!()
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        todo!()
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        todo!()
    }
}