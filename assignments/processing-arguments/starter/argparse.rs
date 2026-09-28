fn shift(args: &mut Vec<String>) {
    for i in 0 .. args.len() - 1 {
        args.swap(i, i + 1);
    }
    let _ = args.pop();
}

fn main() {
    let mut verbose = false;
    let mut args: Vec<String> = std::env::args().collect();
    let _progname = shift(&mut args);
    while args.len() > 0 {
        let mut arg_chars = args[0].chars();
        if let Some('-') = arg_chars.next() {
            match arg_chars.next() {
                Some('h') => {
                    eprintln!("my help message");
                    std::process::exit(0);
                },
                Some('v') => {
                    verbose = true;
                    shift(&mut args);
                }
                // More argument processing
                _ => {
                    eprintln!("unknown argument");
                    std::process::exit(1);
                }
            }
        } else {
            break;
        }
    }

    print!("Hello, world!");
    if verbose {
         print!(" [extra arguments: {:?}]", args);
    }
    println!();
}
