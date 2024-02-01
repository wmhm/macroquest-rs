#![allow(non_snake_case)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use macroquest::log::trace;
use macroquest::plugin::Reason;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

macroquest::plugin::preamble!();

#[macroquest::plugin::main]
fn main(reason: Reason) {
    match reason {
        Reason::Load => {
            trace!(module = PKG_NAME, "module loaded");
        }
        Reason::Unload => {
            trace!(module = PKG_NAME, "module unloaded");
        }
    };
}
