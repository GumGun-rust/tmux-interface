mod set_buffer;
mod show_buffer;
mod targets;

pub use targets::Target;

use std::io::Error as IOError;
use std::process::ExitStatus;
use std::process::Output;

use thiserror::Error as TEError;

const VEC_U8_TO_STR:&str = "Bytes should be valid utf8";

#[derive(Default)]
pub struct Tmux{
    listener: Option<String>,
}

#[derive(Debug, TEError)]
pub enum Error{
    #[error("tmux returned error: {:?}", 0)]
    TmuxError(TmuxError),
    #[error("failed to call tmux: {0}")]
    ExternError(IOError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TmuxError {
    exit_status: ExitStatus,
    stderr: String,
}

impl TmuxError {
    pub fn new(exit_status:ExitStatus, stderr:String) -> Self {
        Self{
            exit_status,
            stderr
        }
    }
}

fn interpret(raw_result:Result<Output, IOError>) -> Result<Output, Error> {
    let output = raw_result.map_err(|error|{Error::ExternError(error)})?;
    if output.status.success() {
        Ok(output)
    } else {
        let error_string = String::from_utf8(output.stderr).expect(VEC_U8_TO_STR);
        Err(Error::TmuxError(TmuxError::new(output.status,error_string)))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn set_fetch_buffer() {
        let content = "name buffer content";
        let tmux = Tmux::default();
        let mut buffer = tmux.set_buffer();
        buffer.set_name("named-buffer");
        buffer.run(content);

        let mut buffer = tmux.show_buffer();
        buffer.set_name("named-buffer");
        let holder = buffer.run().unwrap();
        assert_eq!(content, holder);
    }
    */
}

