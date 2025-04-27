use std::process::Command;

#[derive(Default)]
pub struct Tmux{
    listener: Option<String>,
}

pub struct SetBuffer<'server>{
    server: &'server Tmux,
    name: Option<&'server str>,
}

impl Tmux {
    pub fn set_buffer(&self) -> SetBuffer {
        SetBuffer{
            server: self,
            name: None,
        }
    }
}

impl<'server> SetBuffer<'server> {
    pub fn set_name<'arg:'server>(&mut self, name:&'arg str) -> Result<(), ()> {
        if let Some(_) = self.name {
            return Err(());
        }
        self.name = Some(name);
        Ok(())
    }

    pub fn run(self, content:&str) -> Result<(), ()> {
        let mut cmd_holder = Command::new("tmux");
        let holder = cmd_holder.arg("set-buffer");
        match self.name {
            Some(name) => {
                holder.arg("-b").arg(name);
            }
            None => {}
        }
        println!("{:?}", holder);
        let ret = holder.arg(content).output();
        match ret {
            Ok(ok) => {
                Ok(())
            }
            Err(err) => {
                Err(())
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tmux = Tmux::default();
        let mut buffer = tmux.set_buffer();
        buffer.set_name("named-buffer");
        buffer.run("name buffer content");
    }
}

