use super::Tmux;
use super::Target;

use std::process::Command;

pub struct SetBuffer<'server>{
    target: Option<Target>,
    append: bool,
    server: &'server Tmux,
    name: Option<&'server str>,
}

impl Tmux {
    pub fn set_buffer(&self) -> SetBuffer {
        SetBuffer{
            append: false,
            target: None,
            server: self,
            name: None,
        }
    }
}

impl<'server> SetBuffer<'server> {
    pub fn set_name<'arg:'server>(&mut self, name:&'arg str) -> Result<(), ()> {
        if self.name.is_some() {
            return Err(());
        }
        self.name = Some(name);
        Ok(())
    }

    pub fn append(&mut self) -> &mut Self {
        self.append = true;
        self
    }

    pub fn run(self, content:&str) -> Result<(), ()> {
        let mut cmd_holder = Command::new("tmux");
        let holder = cmd_holder.arg("set-buffer");

        //specify listener
        self.server.listener.as_ref().map(|listener|{holder.arg("-L").arg(listener)});

        //add buffer if present
        self.name.map(|name|{holder.arg("-b").arg(name)});

        let ret = holder.arg(content).output();
        match ret {
            Ok(_ok) => {
                Ok(())
            }
            Err(_err) => {
                Err(())
            }
        }
    }
}



#[test]
fn set_fetch_named_buffer() {
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

#[test]
fn set_fetch_nameless_buffer() {
    let content = "name_less_buffer";
    let tmux = Tmux::default();
    let mut buffer = tmux.set_buffer();
    buffer.run(content);

    let mut buffer = tmux.show_buffer();
    let holder = buffer.run().unwrap();
    assert_eq!(content, holder);
}
