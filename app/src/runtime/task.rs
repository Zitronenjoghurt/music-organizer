pub enum AsyncState<T, E> {
    Idle,
    Loading,
    Ready(T),
    Error(E),
}

impl<T, E> AsyncState<T, E> {
    pub fn ready(&self) -> Option<&T> {
        match self {
            Self::Ready(value) => Some(value),
            _ => None,
        }
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }
}

pub struct AsyncTask<T, E> {
    state: AsyncState<T, E>,
    rx: Option<tokio::sync::mpsc::Receiver<Result<T, E>>>,
    abort: Option<tokio::task::AbortHandle>,
}

impl<T: Send + 'static, E: Send + 'static> Default for AsyncTask<T, E> {
    fn default() -> Self {
        Self {
            state: AsyncState::Idle,
            rx: None,
            abort: None,
        }
    }
}

impl<T: Send + 'static, E: Send + 'static> AsyncTask<T, E> {
    pub fn get(&mut self) -> &AsyncState<T, E> {
        let Some(rx) = &mut self.rx else {
            return &self.state;
        };

        match rx.try_recv() {
            Ok(Ok(value)) => {
                self.state = AsyncState::Ready(value);
                self.rx = None;
                self.abort = None;
            }
            Ok(Err(error)) => {
                self.state = AsyncState::Error(error);
                self.rx = None;
                self.abort = None;
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                self.state = AsyncState::Idle;
                self.rx = None;
                self.abort = None;
            }
        }

        &self.state
    }

    pub fn start<F, Fut>(&mut self, handle: &tokio::runtime::Handle, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send,
    {
        self.abort();
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let join = handle.spawn(async move {
            let _ = tx.send(f().await).await;
        });
        self.state = AsyncState::Loading;
        self.rx = Some(rx);
        self.abort = Some(join.abort_handle());
    }

    pub fn abort(&mut self) {
        if let Some(h) = self.abort.take() {
            h.abort();
        }
        self.rx = None;
        self.state = AsyncState::Idle;
    }
}
