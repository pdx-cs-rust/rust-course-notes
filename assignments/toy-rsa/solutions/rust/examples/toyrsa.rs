use std::error::Error;

fn makekey() {
    let (p, q) = toy_rsa::genkey();
    println!("{:x}:{:x} {:x}", p, q, p as u64 * q as u64);
}

fn read_arg(
    args: &mut dyn Iterator<Item = String>,
    message: &str,
) -> Result<String, Box<dyn Error>> {
    Ok(args.next().ok_or_else(|| message.to_string())?)
}

fn encrypt(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let key = read_arg(&mut args, "missing key")?;
    let key = u64::from_str_radix(&key, 16)?;
    let plain = read_arg(&mut args, "missing plain")?;
    let plain = u32::from_str_radix(&plain, 16)?;
    println!("{:x}", toy_rsa::encrypt(key, plain));
    Ok(())
}

fn decrypt(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let key = read_arg(&mut args, "missing key")?;
    let cipher = read_arg(&mut args, "missing cipher")?;
    let keys: Vec<&str> = key.split(':').collect();
    if keys.len() != 2 {
        return Err("malformed key".to_string().into());
    }
    let p = keys[0];
    let p = u32::from_str_radix(p, 16)?;
    let q = keys[1];
    let q = u32::from_str_radix(q, 16)?;
    let cipher = u64::from_str_radix(&cipher, 16)?;
    println!("{:x}", toy_rsa::decrypt((p, q), cipher));
    Ok(())
}

fn check(status: Result<(), Box<dyn Error>>) {
    if let Err(e) = status {
        eprintln!("toyrsa: missing or invalid argument: {}", e);
        std::process::exit(1)
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    match args.next() {
        Some(command) => match command.as_ref() {
            "genkey" => makekey(),
            "encrypt" => check(encrypt(args)),
            "decrypt" => check(decrypt(args)),
            _ => {
                eprintln!("toyrsa: unknown command: {}", command);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("toyrsa: usage:");
            eprintln!("    toyrsa genkey");
            eprintln!("    toyrsa encrypt <pubkey> <plain>");
            eprintln!("    toyrsa decrypt <privkey1:privkey2> <cipher>");
            std::process::exit(1);
        }
    }
}
