use anyhow::Result;
use rand::prelude::*;
use zxcvbn::zxcvbn;

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // 是否包含大写字母。
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz"; // 是否包含小写字母。
const NUMBERS: &[u8] = b"1234567890"; // 是否包含数字。
const SYMBOLS: &[u8] = b"#$%&'()*+,-./:;<=>?@[]{}~"; // 是否包含符号。;

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(*chars.choose(&mut rng).unwrap()); // 添加一个随机的大写字母到密码的开头。
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(*chars.choose(&mut rng).unwrap()); // 添加一个随机的小写字母到密码的开头。
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*chars.choose(&mut rng).unwrap()); // 添加一个随机数字到密码的开头。
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS); // 这里只包含ASCII符号，可以根据需要扩展。;
        password.push(*chars.choose(&mut rng).unwrap()); // 添加一个随机符号到密码的开头。
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("the chars won't be empty"); // 选择一个字符，如果chars为空，则返回' '。
        password.push(*c); // 添加字符到密码中。
    }
    password.shuffle(&mut rng); // 随机化密码。
    let password = String::from_utf8(password)?;
    println!("{}", password); // 打印密码。

    //output password strength in strerr
    let estimate = zxcvbn(&password, &[])?; // 评估密码强度。
    eprintln!("the strength is {}", estimate.score()); // 打印密码强度。;
    Ok(())
}
