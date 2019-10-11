use std::sync::mpsc;
use std::thread;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events<I> {
    rx: mpsc::Receiver<Event<I>>,
    input_handle: Option<thread::JoinHandle<()>>,
    tick_handle: Option<thread::JoinHandle<()>>,
}

pub type Handle<I> = fn(tx: mpsc::Sender<Event<I>>) -> fn();

impl<I: std::marker::Send> Events<I> {
    pub fn new(input_handle: Handle<I>, tick_handle: Handle<I>) -> Self {
        let (tx, rx) = mpsc::channel();
        let input_handle = Some({
            let tx = tx.clone();
            thread::spawn(input_handle(tx))
        });
        let tick_handle = Some({
            let tx = tx.clone();
            thread::spawn(tick_handle(tx))
        });

        Self {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<I>, mpsc::RecvError> {
        self.rx.recv()
    }
}

impl<I> Drop for Events<I> {
    fn drop(&mut self) {
        self.input_handle
            .take()
            .unwrap()
            .join()
            .expect("Couldn't join on the input handle thread");
        self.tick_handle
            .take()
            .unwrap()
            .join()
            .expect("Couldn't join on the tick handle thread");
    }
}
