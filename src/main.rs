use std::process::Command;

fn main() {

    // spawn()
    println!("Running via 'spawn()'");
    let mut out = Command::new("ls")
        .arg("-l")
        .arg("-a") 
        .spawn()
        .expect("ls command failed to start");
    out.wait().expect("Failing while waiting");
    println!("{:?}", out);

    // output()
    println!("");
    println!("Running via 'output()'");
    let out2 = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("ls command failed to start");
    println!("");
    println!("{:?}", out2);

   // status()
    println!("");
    println!("Running via 'status()'");
    let out3 = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .status()
        .expect("ls command failed to start");
    println!("");
    println!("{:?}", out3);
}
