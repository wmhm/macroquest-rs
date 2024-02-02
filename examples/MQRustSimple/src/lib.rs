#![allow(non_snake_case)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

#[derive(Debug, Default)]
#[macroquest::plugin::main]
struct MQRustSimple {}
