use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut a: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut b: [i32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut filenumber = 0;
    let filename = "tb.scs";
    let filename_mdl = "tset.mdl";
    let mut newfilename;

    // **** B -> tudo a 1 | A -> varia 1 a 1 *****************************
    for i in b.iter_mut() {
        *i = 1;
    }

    for i in 0..16 {
        a[i] = 1;
        newfilename = format!("{}", filenumber);
        gen_netlist(filename, newfilename.as_str(), &a, &b);
        gen_mdl(
            filename_mdl,
            newfilename.as_str(),
            format!("A{}", i).as_str(),
        );
        a[i] = 0;
        filenumber = filenumber + 1;
    }

    // **** B -> tudo a 1 | A -> varia 2 a 2 *****************************
    for a1 in 0..16 {
        a[a1] = 1;
        for a2 in 0..16 {
            if a2 == a1 {
                continue;
            }
            a[a2] = 1;
            newfilename = format!("{}", filenumber);
            gen_netlist(filename, newfilename.as_str(), &a, &b);
            gen_mdl(
                filename_mdl,
                newfilename.as_str(),
                format!("A{}", a1).as_str(),
            );
            a[a2] = 0;
            filenumber = filenumber + 1;
        }
        a[a1] = 0;
    }

    for i in 0..3 {
        read_meas_file(format!("{}.measure", i).as_str());
    }
}

fn gen_mdl(filename: &str, newfilename: &str, bit: &str) {
    let mut pre_pre_risea = String::from("    real rise_inA");
    let pre_risea = String::from("=cross(sig=V(");
    let post_risea = "), dir='rise, n=1, thresh=Supply/2, start=0)";
    let newline_risea: String;
    let newline_riseb = String::from(
        "    real rise_inB=cross(sig=V(B15), dir='rise, n=1, thresh=Supply/2, start=0)",
    );

    pre_pre_risea.push_str(pre_risea.as_str());
    pre_pre_risea.push_str(format!("{}", bit).as_str());
    pre_pre_risea.push_str(post_risea);
    newline_risea = pre_pre_risea.clone();

    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/max_delay/docs/mdl/";
    let file = fs::read_to_string(format!("{}{}", path, filename))
        .expect("Something went wrong reading the file");
    let mut new_file = File::create(format!("{}{}.mdl", path, newfilename))
        .expect("Something went wrong creating the file");

    let mut line_number = 0;
    for line in file.lines() {
        if line_number == 2 {
            new_file
                .write_all(format!("{}\n", newline_risea).as_bytes())
                .expect("Something went wrong writing a line");
        } else if line_number == 3 {
            new_file
                .write_all(format!("{}\n", newline_riseb).as_bytes())
                .expect("Something went wrong writing a line");
        } else {
            new_file
                .write_all(format!("{}\n", line).as_bytes())
                .expect("Something went wrong writing a line");
        }
        line_number = line_number + 1;
    }
}

fn gen_netlist(filename: &str, newfilename: &str, a: &[i32], b: &[i32]) {
    let mut newline = String::from("parameters ");
    let mut cnt = 0;

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

    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/max_delay/docs/scs/";
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

fn read_meas_file(filename: &str) {
    let path = "/Users/armando/Documents/mac/my_dir/github/vlsi/max_delay/docs/meas/";
    let file = fs::read_to_string(format!("{}{}", path, filename))
        .expect("Something went wrong reading the file");
    let mut token_num = 0;
    let mut data = String::new();

    for token in file.split("=") {
        if token_num >= 1 {
            data.push_str(token);
        }

        token_num = token_num + 1;
    }

    let mut token_num = 0;

    let mut rfile = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}{}", path, "results.txt"))
        .unwrap();

    for token in data.split("\n") {
        if token_num == 0 {
            rfile
                .write_all(
                    format!("***{}***\ntphl_E                {}\n", filename, token).as_bytes(),
                )
                .expect("Something went wrong writing a line");
        } else {
            rfile
                .write_all(format!("{}\n", token).as_bytes())
                .expect("Something went wrong writing a line");
        }

        if token_num > 3 {
            break;
        }
        token_num = token_num + 1;
    }
}
