#[crate_type = "lib"];
#[crate_id = "github.com/bjz/openal-rs#openal:0.1"];
#[comment = "OpenAL 1.1 bindings for Rust."];

#[feature(globs)];
#[feature(link_args)];
#[feature(macro_rules)];

pub mod al;
pub mod alc;

#[nolink]
#[link_args="-framework OpenAL"]
#[cfg(target_os = "macos")]
extern {}

#[nolink]
#[link_args="-lopenal"]
#[cfg(target_os = "linux")]
extern {}
