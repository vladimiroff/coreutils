#![deny(warnings)]

extern crate coreutils;
extern crate extra;
extern crate syscall;

use std::env;
use std::io::{stderr, stdout, Error, Write};
use std::process::exit;
use coreutils::ArgParser;
use extra::option::OptionalExt;
use syscall::flag::SIGKILL;

const MAN_PAGE: &'static str = /* @MANSTART{shutdown} */ r#"
NAME
    shutdown - stop the system

SYNOPSIS
    shutdown [ -h | -help ]

DESCRIPTION
    Attempt to shutdown the system using ACPI. Failure will be logged to the terminal

OPTIONS
    -h
    --help
        display this help and exit
"#; /* @MANEND */

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stderr = stderr();
    let mut parser = ArgParser::new(1)
        .add_flag(&["h", "help"]);
    parser.parse(env::args());

    if parser.found("help") {
        stdout.write(MAN_PAGE.as_bytes()).try(&mut stderr);
        stdout.flush().try(&mut stderr);
        exit(0);
    }

    syscall::kill(1, SIGKILL).map_err(|err| Error::from_raw_os_error(err.errno)).try(&mut stderr);
}
