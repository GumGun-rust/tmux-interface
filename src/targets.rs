use std::fmt::Error as FMTError;
use std::fmt::Formatter;
use std::fmt::Display;
use std::str::FromStr;

use derive_more::From;
use derive_more::Display as DMDisplay;


#[crabtime::function]
fn gen_conversions(target_type: String) {
    crabtime::output! {
        impl From<{{target_type}}> for u64 {
            fn from(value:{{target_type}}) -> Self {
                value.0
            }
        }
        impl {{target_type}} {
            pub fn as_target(&self) -> Target {
                Target::{{target_type}}(*self)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, DMDisplay, From, PartialEq)]
#[display("${}", self.0)]
pub struct Session(u64);
gen_conversions!(Session);


#[derive(Clone, Copy, Debug, DMDisplay, From, PartialEq)]
#[display("@{}", self.0)]
pub struct Window(u64);
gen_conversions!(Window);


#[derive(Clone, Copy, Debug, DMDisplay, From, PartialEq)]
#[display("%{}", self.0)]
pub struct Pane(u64);
gen_conversions!(Pane);


#[derive(Clone, Debug, PartialEq)]
pub enum Target{
    Session(Session),
    Window(Window),
    Pane(Pane),
}


impl FromStr for Target {
    type Err = ();
    fn from_str(base:&str) -> Result<Self, Self::Err> {
        match &base[..1] {
            "$" => {
                let holder = u64::from_str(&base[1..]).map_err(|_|{})?;
                Ok(Target::Session(Session(holder)))
            }
            "@" => {
                let holder = u64::from_str(&base[1..]).map_err(|_|{})?;
                Ok(Target::Window(Window(holder)))
            }
            "%" => {
                let holder = u64::from_str(&base[1..]).map_err(|_|{})?;
                Ok(Target::Pane(Pane(holder)))
            }
            _ => {
                Err(())
            }
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FMTError> {
        match self {
            Self::Session(window) => window.fmt(f),
            Self::Window(window) => window.fmt(f),
            Self::Pane(window) => window.fmt(f),
        }
    }
}

#[test]
fn equallity() {
    assert_eq!(Err(()), Target::from_str("3()"));

    /* to String */
    assert_eq!("$2", Session(2).to_string());
    assert_eq!("@2", Window(2).to_string());
    assert_eq!("%2", Pane(2).to_string());

    /* into u64 and back */
    assert_eq!(Session(2), Session::from(u64::from(Session(2))));
    assert_eq!(Pane(2), Pane::from(u64::from(Session(2))));
    assert_eq!(Window(2), Window::from(u64::from(Session(2))));

    /* from string */
    assert_eq!(Ok(Target::Session(Session(2))), Target::from_str("$2"));
    assert_eq!(Ok(Target::Window(Window(2))), Target::from_str("@2"));
    assert_eq!(Ok(Target::Pane(Pane(2))), Target::from_str("%2"));

}

