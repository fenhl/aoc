#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

#[macro_use] extern crate aoc_runner_derive;

mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

aoc_lib! { year = 2019 }
