use super::Tmux;

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

    pub fn run(self) -> Result<String, ()> {
        let mut cmd_holder = Command::new("tmux");
        let holder = cmd_holder.arg("show-buffer");

        //specify listener
        self.server.listener.as_ref().map(|listener|{holder.arg("-L").arg(listener)});

        //add buffer if present
        self.name.map(|name|{holder.arg("-b").arg(name)});

        let ret = holder.output();
        match ret {
            Ok(ok) => {
                let string = String::from_utf8(ok.stdout).expect("Our bytes should be valid utf8");
                Ok(string)
            }
            Err(err) => {
                Err(())
            }
        }
    }
}


