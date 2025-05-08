use super::Tmux;
use super::Error;
use super::interpret;
use super::VEC_U8_TO_STR;

use std::process::Command;

pub struct ShowBuffer<'server>{
    server: &'server Tmux,
    name: Option<&'server str>,
}

impl Tmux {
    pub fn show_buffer(&self) -> ShowBuffer {
        ShowBuffer{
            server: self,
            name: None,
        }
    }
}

impl<'server> ShowBuffer<'server> {
    pub fn set_name<'arg:'server>(&mut self, name:&'arg str) -> Result<(), ()> {
        if self.name.is_some() {
            return Err(());
        }
        self.name = Some(name);
        Ok(())
    }

    pub fn run(self) -> Result<String, Error> {
        let mut cmd_holder = Command::new("tmux");
        let holder = cmd_holder.arg("show-buffer");

        //specify listener
        self.server.listener.as_ref().map(|listener|{holder.arg("-L").arg(listener)});

        //add buffer name if present
        self.name.map(|name|{holder.arg("-b").arg(name)});

        let ret = holder.output();
        let output = interpret(ret)?;

        let string = String::from_utf8(output.stdout).expect(VEC_U8_TO_STR);
        Ok(string)
    }
}


