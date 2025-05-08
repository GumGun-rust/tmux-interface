mod set_buffer;
mod show_buffer;
mod targets;

pub use targets::Target;


#[derive(Default)]
pub struct Tmux{
    listener: Option<String>,
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

