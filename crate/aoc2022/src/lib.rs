#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

#[macro_use] extern crate aoc_runner_derive;

mod day01;

aoc_lib! { year = 2022 }
