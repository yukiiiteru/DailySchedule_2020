//! 管道 [`Pipe`]

use super::*;
use alloc::collections::VecDeque;

pub struct Pipe {
    buffer: Arc<Mutex<VecDeque<u8>>>,
    direction: PipeEnd,
}

#[derive(Clone, PartialEq)]
pub enum PipeEnd {
    Read,
    Write,
}

impl Pipe {
    pub fn create_pair() -> (Pipe, Pipe) {
        let inner: VecDeque<u8> = VecDeque::new();
        let data = Arc::new(Mutex::new(inner));
        (
            Pipe {
                buffer: data.clone(),
                direction: PipeEnd::Read,
            },
            Pipe {
                buffer: data.clone(),
                direction: PipeEnd::Write,
            },
        )
    }
}

impl INode for Pipe {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if self.direction == PipeEnd::Write {
            Err(FsError::NotSupported)
        } else if offset != 0 {
            Err(FsError::NotSupported)
        } else if self.buffer.lock().len() == 0 {
            Ok(0)
        } else {
            let mut pipe_buffer = self.buffer.lock();
            for (i, byte) in buf.iter_mut().enumerate() {
                if let Some(b) = pipe_buffer.pop_front() {
                    *byte = b;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if self.direction == PipeEnd::Read {
            Err(FsError::NotSupported)
        } else if offset != 0 {
            Err(FsError::NotSupported)
        } else {
            let mut data = self.buffer.lock();
            for c in buf {
                data.push_back(*c);
            }
            Ok(buf.len())
        }
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}
