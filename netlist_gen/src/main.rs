use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut a: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let b: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for param in a.iter_mut() {
        *param = 1;
    }

    let mut newline = String::from("parameters ");
    let mut cnt = 0;

    for elem in a.iter() {
        newline.push_str(format!("s{}={}, ", cnt, elem).as_str());
        cnt = cnt + 1;
    }

    for elem in b.iter() {
        newline.push_str(format!("s{}={}, ", cnt, elem).as_str());
        if cnt == 31 {
            newline.push_str(format!("s{}={};", cnt, elem).as_str());
        }
        cnt = cnt + 1;
    }

    println!("{}", newline);

    let filename = "/Users/armando/Documents/mac/my_dir/github/vlsi/netlist_gen/docs/tb.scs";
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut new_file = File::create(
        "/Users/armando/Documents/mac/my_dir/github/vlsi/netlist_gen/docs/tb_auto.scs",
    )
    .expect("Something went wrong creating the file");

    let mut line_number = 0;
    for line in file.lines() {
        if line_number == 15 {
            println!("Found line 16:");
            println!("{}", line);
            new_file
                .write_all(format!("{}\n", newline).as_bytes())
                .expect("Something went wrong writing a line");
            println!("Replaced line 16:");
            println!("{}", newline);
        } else {
            new_file
                .write_all(format!("{}\n", line).as_bytes())
                .expect("Something went wrong writing a line");
        }

        line_number = line_number + 1;
    }
}
