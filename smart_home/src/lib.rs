#![allow(dead_code)]

pub mod devices;
mod home;
mod room;

macro_rules! quick_display_and_error {
    ($struct_name: ident) => {
        impl Display for $struct_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match &self.err {
                    Some(err) => write!(f, "{} ERR: {}", stringify!($struct_name), err.to_string()),
                    None => write!(f, "{} OK", stringify!($struct_name)),
                }
            }
        }

        impl std::error::Error for $struct_name {}
    };
}

pub(crate) use quick_display_and_error;
