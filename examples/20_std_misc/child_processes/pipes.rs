use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

// fn main() {
//     // Spawn the `wc` command
//     let mut cmd = if cfg!(target_family = "windows") {
//         let mut cmd = Command::new("powershell");
//         cmd.arg("-Command")
//             .arg("$input | Measure-Object -Line -Word -Character");
//         cmd
//     } else {
//         Command::new("wc")
//     };
//     let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
//         Err(why) => panic!("couldn't spawn wc: {}", why),
//         Ok(process) => process,
//     };

//     // Write a string to the `stdin` of `wc`.
//     //
//     // `stdin` has type `Option<ChildStdin>`, but since we know this instance
//     // must have one, we can directly `unwrap` it.
//     match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
//         Err(why) => panic!("couldn't write to wc stdin: {}", why),
//         Ok(_) => println!("sent pangram to wc"),
//     }

//     // Because `stdin` does not live after the above calls, it is `drop`ed,
//     // and the pipe is closed.
//     //
//     // This is very important, otherwise `wc` wouldn't start processing the
//     // input we just sent.

//     // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
//     let mut s = String::new();
//     match process.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}", why),
//         Ok(_) => print!("wc responded with:\n{}", s),
//     }
// }

fn main() -> io::Result<()> {
    // Spawn the `wc` command
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command")
            .arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };
    let process = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    // process.stdin.unwrap().write_all(PANGRAM.as_bytes())?;
    process
        .stdin
        .ok_or(io::Error::new(io::ErrorKind::Other, "failed to open stdin"))?
        .write_all(PANGRAM.as_bytes())?;
    println!("sent pangram to wc");

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    process
        .stdout
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "couldn't get handle to wc stdout",
        ))?
        .read_to_string(&mut s)?;
    print!("wc responded with:\n{}", s);

    io::Result::Ok(())
}
