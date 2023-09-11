use tokio::sync::RwLock;

pub struct Cache<T> {
    pub(crate) inner: RwLock<Option<T>>,
}

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }
}
