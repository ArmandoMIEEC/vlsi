use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut a: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut b: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for param in a.iter_mut() {
        *param = 1;
    }

    let filenumber = 0;
    let filename = "tb.scs";
    let mut newfilename = format!("{}", filenumber);

    //A->tudo a 1 B->tudo a 0
    gen_netlist(filename, newfilename.as_str(), &a, &b);

    //A->tudo a 1 B->varia
    for filenumber in 1..15 {
        b[filenumber] = 1;
        newfilename = format!("{}", filenumber);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        b[filenumber] = 0;
    }

    for param in a.iter_mut() {
        *param = 0;
    }

    //A->tudo a 0 B->varia
    for filenumber in 1..15 {
        b[filenumber] = 1;
        newfilename = format!("{}", filenumber);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        b[filenumber] = 0;
    }

    //B->tudo a 0 A->varia
    for filenumber in 1..15 {
        a[filenumber] = 1;
        newfilename = format!("{}", filenumber);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        a[filenumber] = 0;
    }

    for param in a.iter_mut() {
        *param = 1;
    }

    //B->tudo a 1 A->varia
    for filenumber in 1..15 {
        a[filenumber] = 1;
        newfilename = format!("{}", filenumber);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        a[filenumber] = 0;
    }
}

fn gen_netlist(filename: &str, newfilename: &str, a: &[i32], b: &[i32]) {
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

    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/netlist_gen/docs/";
    let file = fs::read_to_string(format!("{}{}", path, filename))
        .expect("Something went wrong reading the file");
    let mut new_file = File::create(format!("{}{}.scs", path, newfilename))
        .expect("Something went wrong creating the file");

    let mut line_number = 0;
    for line in file.lines() {
        if line_number == 15 {
            new_file
                .write_all(format!("{}\n", newline).as_bytes())
                .expect("Something went wrong writing a line");
        } else {
            new_file
                .write_all(format!("{}\n", line).as_bytes())
                .expect("Something went wrong writing a line");
        }
        line_number = line_number + 1;
    }
}
