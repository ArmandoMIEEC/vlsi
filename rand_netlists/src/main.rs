use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut a: u16;
    let mut b: u16;
    let mut rng = rand::thread_rng();
    let mut change_bit = 0;
    let mut mask = 0b1000000000000000;

    let filename = "tb.scs";
    let filename_mdl = "del.mdl";

    a = rng.gen_range(0..65534);
    b = rng.gen_range(0..a);

    for i in 0..16 {
        let bit = a & mask != 0;
        if bit {
            change_bit = i;
            break;
        }
        mask >>= 1;
    }

    for elem in 1..250 {
        let newfilename = format!("{}", elem);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        gen_mdl(
            filename_mdl,
            newfilename.as_str(),
            format!("A{}", change_bit).as_str(),
        );

        a = rng.gen_range(0..65534);
        b = rng.gen_range(0..a);

        for i in 0..16 {
            let bit = a & mask != 0;
            if bit {
                change_bit = i;
                break;
            }
            mask >>= 1;
        }

        println!("a:{:b}, b:{:b}, change_bit:{}", a, b, change_bit);
    }
}

fn gen_mdl(filename: &str, newfilename: &str, bit: &str) {
    let mut pre_newline_rise = String::from("    real rise_in=cross(sig=V(");
    let post_newline_rise = "), dir='rise, n=1, thresh=Supply/2, start=0)";
    let mut pre_newline_fall = String::from("    real fall_in=cross(sig=V(");
    let post_newline_fall = "), dir='fall, n=1, thresh=Supply/2, start=0)";
    let newline_rise: String;
    let newline_fall: String;

    pre_newline_rise.push_str(format!("{}", bit).as_str());
    pre_newline_rise.push_str(post_newline_rise);
    newline_rise = pre_newline_rise.clone();
    pre_newline_fall.push_str(format!("{}", bit).as_str());
    pre_newline_fall.push_str(post_newline_fall);
    newline_fall = pre_newline_fall.clone();

    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/rand_netlists/docs/mdl/";
    let file = fs::read_to_string(format!("{}{}", path, filename))
        .expect("Something went wrong reading the file");
    let mut new_file = File::create(format!("{}{}.mdl", path, newfilename))
        .expect("Something went wrong creating the file");

    let mut line_number = 0;
    for line in file.lines() {
        if line_number == 2 {
            new_file
                .write_all(format!("{}\n", newline_rise).as_bytes())
                .expect("Something went wrong writing a line");
        } else if line_number == 3 {
            new_file
                .write_all(format!("{}\n", newline_fall).as_bytes())
                .expect("Something went wrong writing a line");
        } else {
            new_file
                .write_all(format!("{}\n", line).as_bytes())
                .expect("Something went wrong writing a line");
        }
        line_number = line_number + 1;
    }
}

fn gen_netlist(filename: &str, newfilename: &str, an: &u16, bn: &u16) {
    let mut a: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut b: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut newline = String::from("parameters ");
    let mut cnt = 0;
    let mut mask = 0b1000000000000000;

    for i in 0..16 {
        let bit = *an & mask != 0;
        a[i] = bit as i32;
        mask >>= 1;
    }

    mask = 0b1000000000000000;

    for i in 0..16 {
        let bit = bn & mask != 0;
        b[i] = bit as i32;
        mask >>= 1;
    }

    for elem in a.iter() {
        newline.push_str(format!("s{}={} ", cnt, elem).as_str());
        cnt = cnt + 1;
    }

    for elem in b.iter() {
        if cnt == 31 {
            newline.push_str(format!("s{}={}", cnt, elem).as_str());
            break;
        }
        newline.push_str(format!("s{}={} ", cnt, elem).as_str());
        cnt = cnt + 1;
    }

    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/rand_netlists/docs/scs/";
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
