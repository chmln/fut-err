use futures;

pub struct LineStream<T: futures::io::AsyncRead + Unpin> {
    inner: T,
}

impl<T: futures::io::AsyncRead + Unpin> LineStream<T> {
    pub fn new(inner: T) -> Self {
        inner.poll_read();
        Self { inner }
    }
}

fn main() {
}
