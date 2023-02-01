use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct Pipe(Arc<PipeInner>);

const WAIT_TIMEOUT: Duration = Duration::from_secs(3);

pub struct PipeInner {
    buf: Mutex<VecDeque<u8>>,
    cvar: Condvar,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe(Arc::new(PipeInner {
            buf: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }))
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let res = self.0.buf.lock().unwrap().write(buf);
        self.0.cvar.notify_all();
        res
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (mut guard, timeout) = self
            .0
            .cvar
            .wait_timeout_while(self.0.buf.lock().unwrap(), WAIT_TIMEOUT, |deq| {
                deq.is_empty()
            })
            .unwrap();
        assert!(!timeout.timed_out(), "timeout");
        guard.read(buf)
    }
}
