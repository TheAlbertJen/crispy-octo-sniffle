use std::{path::Path, fs, io::stdin, io::BufReader};
use std::io::BufRead;
use evalexpr;
use evalexpr::eval;


fn main() {
    // if there was an argument to point towards file or directory do that
    if std::env::args().len() == 3 && std::env::args().nth(1).unwrap() == "-d" {
        // verify directory exists, and can be read from
        let input_directory = &std::env::args().nth(2).unwrap();
        let test_directory = Path::new(input_directory);
        if test_directory.try_exists().expect("Error reading test directory.") {
            let files = fs::read_dir(test_directory).unwrap();
            for file in files {
                let path = file.as_ref().unwrap().path();
                let mut line_number = 1;
                let mut total_tests = 0;
                let mut passed_tests = 0;
                let mut failed_tests = 0;
                // read file line by line?
                let reader = BufReader::new(fs::File::open(path).unwrap());
                for line in reader.lines() {
                    // evaluate expressions, ignore invalid cases
                    match parse(&line.unwrap()) {
                        Ok(result) => {
                            total_tests += 1;
                            if result {
                                passed_tests += 1
                            } else {
                                failed_tests += 1;
                            }
                        },
                        // I thought about outputting parse errors, but ignoring them in the file parsing
                        // allows a rudimentary commenting system (any line not that is not an expression
                        // is ignored and can be used for commenting tests
                        Err(_) => {} // println!("Parse error - {filename}:line {line_number}"),
                    }
                    line_number += 1;
                }
                println!("{filename} - Total: {total_tests}, Passed: {passed_tests}, Failed: {failed_tests}",
                    filename=file.unwrap().file_name().to_str().unwrap());
            };
        } else {
            println!("Test directory does not exist.")
        }

    } else {
        // else console input

        let mut line = String::new();
        loop {
            line = String::new();
            println!("Enter expression (\"STOP\" to stop):");
            stdin().read_line(&mut line).unwrap();
            if String::from(line.trim()).to_uppercase() == "STOP" {
                break;
            }
            match parse(&line) {
                Ok(result) => println!("Test: {expr} {result}", expr = line),
                Err(error) => println!("Parse error: {error}")
            }

        }
    }

}
fn parse(input: &str) -> Result<bool, &'static str> {
    if input.contains("==") ||
        input.contains("!=") ||
        input.contains("<") ||
        input.contains(">") {
        match eval(&input) {
            Ok(result) => Ok(result.as_boolean().unwrap()),
            Err(_) => Err("Missing or invalid comparator."),
        }
    } else {
        Err("Invalid test expression.")
    }
}
