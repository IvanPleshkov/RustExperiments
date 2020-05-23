extern crate pest;
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use std::ffi::CString;

#[derive(Parser)]
#[grammar = "tyro.pest"]
struct TyroPestParser;
