mod tokiort;
#[allow(unused)]
pub use tokiort::{TokioExecutor, TokioIo, TokioTimer};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::Context;
    use hyper::rt::{Executor, Timer};
    use std::task::Context;
    use tokio::test;
    use tokio::time::Duration;

    #[test]
    async fn test_tokio_executor() {
        let executor = TokioExecutor;
        let fut = async {
            println!("Hello, world!");
        };
        executor.execute(fut);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // #[tokio::test]
    // async fn test_tokio_timer() {
    //     let timer = TokioTimer::new();
    //     let sleep = timer.sleep(tokio::time::Duration::from_millis(100));
    //     tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    //     assert!(sleep
    //         .poll(&mut tokio::task::Context::from_waker(
    //             tokio::task::noop_waker_ref()
    //         ))
    //         .is_pending());
    //     tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    //     assert!(sleep
    //         .poll(&mut tokio::task::Context::from_waker(
    //             tokio::task::noop_waker_ref()
    //         ))
    //         .is_ready());
    // }

    // #[test]
    // async fn test_tokio_io() {
    //     let io = TokioIo(tokio::io::BufWriter::new(tokio::io::BufReader::new(
    //         tokio::io::empty(),
    //     )));
    //     let mut buf = [0; 10];
    //     let mut cursor = hyper::rt::ReadBufCursor::new(&mut buf);
    //     assert!(io
    //         .poll_read(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             cursor
    //         )
    //         .is_pending());
    //     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    //     assert!(io
    //         .poll_read(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             cursor
    //         )
    //         .is_ready());
    // }

    // #[tokio::test]
    // async fn test_tokio_io_async_read() {
    //     let io = TokioIo(tokio::io::BufWriter::new(tokio::io::BufReader::new(
    //         tokio::io::empty(),
    //     )));
    //     let mut buf = [0; 10];
    //     let mut cursor = hyper::rt::ReadBufCursor::new(&mut buf);
    //     assert!(io
    //         .poll_read(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             cursor
    //         )
    //         .is_pending());
    //     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    //     assert!(io
    //         .poll_read(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             cursor
    //         )
    //         .is_ready());
    // }

    // #[tokio::test]
    // async fn test_tokio_io_async_write() {
    //     let io = TokioIo(tokio::io::BufWriter::new(tokio::io::BufReader::new(
    //         tokio::io::empty(),
    //     )));
    //     let mut buf = [0; 10];
    //     let mut cursor = hyper::rt::ReadBufCursor::new(&mut buf);
    //     assert!(io
    //         .poll_write(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             &buf
    //         )
    //         .is_pending());
    //     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    //     assert!(io
    //         .poll_write(
    //             &mut tokio::task::Context::from_waker(tokio::task::noop_waker_ref()),
    //             &buf
    //         )
    //         .is_ready());
    // }
}
