use hyper::rt::Executor;

#[derive(Clone)]
pub struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn(fut);
    }
}
