use async_std::sync::Mutex;
use std::sync::Arc;
use futures::future::BoxFuture;
use std::future::Future;

type ProtocolBasePtr = Arc<dyn ProtocolBase>;

trait ProtocolBase {
}

type Constructor = Box<dyn Fn(u32) -> BoxFuture<'static, ProtocolBasePtr> + Send + Sync>;

struct ProtocolRegistry {
    protocol_constructors: Mutex<Vec<Constructor>>
}

impl ProtocolRegistry {
    async fn register<C, F>(&self, constructor: C)
    where
        C: 'static + Fn(u32) -> F + Send + Sync,
        F: 'static + Future<Output=Arc<dyn ProtocolBase + 'static>> + Send
    {
        let constructor = Box::new(move |x| Box::pin(constructor(x)));
        self.protocol_constructors.lock().await.push(constructor);
    }
}

fn main() {
}

