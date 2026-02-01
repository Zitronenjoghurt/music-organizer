use std::future::Future;
use std::pin::Pin;
use tokio::task::AbortHandle;

pub enum AsyncState<T, E> {
    Idle,
    Loading,
    Ready(T),
    Error(E),
    Failed,
}

pub type TaskRoutine<I, T, E> =
    Box<dyn Fn(I) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>> + Send + Sync>;

pub struct AsyncTask<I, T, E> {
    tokio: tokio::runtime::Handle,
    state: AsyncState<T, E>,
    routine: TaskRoutine<I, T, E>,
    rx: Option<tokio::sync::mpsc::Receiver<Result<T, E>>>,
    abort: Option<AbortHandle>,
}

impl<I, T, E> AsyncTask<I, T, E>
where
    T: Send + 'static,
    E: Send + 'static,
    I: Send + 'static,
{
    pub fn new<F, Fut>(tokio: &tokio::runtime::Handle, f: F) -> Self
    where
        F: Fn(I) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
    {
        Self {
            state: AsyncState::Idle,
            tokio: tokio.clone(),
            routine: Box::new(move |input| Box::pin(f(input))),
            rx: None,
            abort: None,
        }
    }

    pub fn set_routine<F, Fut>(&mut self, f: F)
    where
        F: Fn(I) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
    {
        self.routine = Box::new(move |input| Box::pin(f(input)));
    }

    pub fn trigger(&mut self, input: I) {
        if self.is_loading() {
            return;
        }

        self.cleanup_abort();

        let future = (self.routine)(input);

        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let join = self.tokio.spawn(async move {
            let _ = tx.send(future.await).await;
        });

        self.state = AsyncState::Loading;
        self.rx = Some(rx);
        self.abort = Some(join.abort_handle());
    }

    pub fn get(&mut self) -> Option<&T> {
        self.poll();
        if let AsyncState::Ready(val) = &self.state {
            Some(val)
        } else {
            None
        }
    }

    pub fn take_error(&mut self) -> Option<E> {
        self.poll();
        if let AsyncState::Error(_) = self.state {
            let old = std::mem::replace(&mut self.state, AsyncState::Failed);
            if let AsyncState::Error(e) = old {
                return Some(e);
            }
        }
        None
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.state, AsyncState::Loading)
    }

    pub fn can_trigger(&self) -> bool {
        !self.is_loading()
    }

    fn poll(&mut self) {
        if let Some(rx) = &mut self.rx {
            match rx.try_recv() {
                Ok(Ok(val)) => {
                    self.state = AsyncState::Ready(val);
                    self.cleanup_connection();
                }
                Ok(Err(err)) => {
                    self.state = AsyncState::Error(err);
                    self.cleanup_connection();
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                    self.state = AsyncState::Idle;
                    self.cleanup_connection();
                }
                _ => {}
            }
        }
    }

    fn cleanup_connection(&mut self) {
        self.rx = None;
        self.abort = None;
    }

    fn cleanup_abort(&mut self) {
        if let Some(h) = self.abort.take() {
            h.abort();
        }
        self.rx = None;
    }
}
