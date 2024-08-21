use std::process::Command;
fn main() {
    // use std::path::Command; to compile io.c through gcc with no lib or start into io.o
    // then, link io.o with output.o and start.o
    Command::new("gcc")
        .args(&["-c", "io.c", "-o", "io.o", "-nostartfiles", "-nostdlib"])
        .status()
        .unwrap();
    Command::new("ld")
        .args(&["-o", "bf",  "output.o", "io.o", "start.o"])
        .status()
        .unwrap();
}