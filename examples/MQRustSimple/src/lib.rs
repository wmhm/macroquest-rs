#![allow(non_snake_case)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use macroquest::eq;
use macroquest::log::{debug, trace};

const VERSION: &str = "1.0";

#[derive(Debug, Default)]
#[macroquest::plugin::main]
struct MQRustSimple {}
