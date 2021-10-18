pub struct ThreadPool;
pub struct PoolCreationError;

impl ThreadPool {
    /// # Panics
    ///
    /// Panic if `size` is 0 or less
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if !size > 0 {
            return Err(PoolCreationError)
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}