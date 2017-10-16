extern crate regex;
extern crate reqwest;
#[macro_use] extern crate hyper;
#[macro_use] extern crate log;
extern crate xml;
extern crate xmltree;
extern crate chrono;

pub mod inn;
pub mod kpp;
pub mod bik;
pub mod common;
pub mod fns_service;
#[cfg(test)] mod unit_test;