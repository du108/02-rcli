use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_file_input;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "decode", about = "Decode base64 data")]
    Decode(DecodeOpts), // 定义一个枚举类型，包含DecodeOpts结构体作为成员
    #[command(name = "encode", about = "Encode data to base64")]
    Encode(EncodeOpts), // 定义一个枚举类型，包含EncodeOpts结构体作为成员
}

#[derive(Parser, Debug)]
pub struct DecodeOpts {
    // 定义一个结构体，包含需要解析的参数和选项
    #[arg(long, value_parser = verify_file_input, default_value = "-")]
    // 定义一个短选项和长选项，用于指定输入文件路径
    pub input: String, // 定义一个String类型的成员，用于存储输入文件路径
    #[arg(long, value_parser = parse_base64_format, default_value = "StanDand")]
    pub format: Base64Format,
}
#[derive(Parser, Debug)]
pub struct EncodeOpts {
    // 定义一个结构体，包含需要解析的参数和选项
    #[arg(long, value_parser = verify_file_input, default_value = "-")]
    pub input: String, // 定义一个String类型的成员，用于存储输入文件路径
    #[arg(long, value_parser = parse_base64_format, default_value = "StanDand")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    StanDand,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, &'static str> {
    // 实现解析base64格式的函数
    format.parse() // 将输入的字符串转换为Base64Format枚举类型
}

impl FromStr for Base64Format {
    // 实现FromStr trait，用于将字符串转换为Base64Format枚举类型
    type Err = &'static str; // 定义错误类型为静态字符串引用
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 实现from_str函数，用于将字符串转换为Base64Format枚举类型
        match s.to_lowercase().as_str() {
            // 将输入的字符串转换为大写，并使用模式匹配进行匹配
            "standand" => Ok(Base64Format::StanDand), // 如果匹配成功，返回Base64Format::StanDand枚举类型
            "urlsafe" => Ok(Base64Format::UrlSafe), // 如果匹配成功，返回Base64Format::UrlSafe枚举类型
            _ => Err("Invalid base64 format"), // 如果匹配失败，返回错误信息"Invalid base64 format"
        }
    }
}

impl From<&str> for Base64Format {
    fn from(s: &str) -> Self {
        // 实现from函数，用于将字符串转换为Base64Format枚举类型，并返回Base64Format::StanDand枚举类型作为默认值
        s.parse().unwrap_or(Base64Format::StanDand)
    }
}

impl fmt::Display for Base64Format {
    // 实现fmt::Display trait，用于将Base64Format枚举类型转换为字符串
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 实现fmt函数，用于将Base64Format枚举类型转换为字符串
        match self {
            // 使用模式匹配进行匹配，根据不同的枚举类型返回不同的字符串
            Base64Format::StanDand => write!(f, "STANDARD"), // 如果匹配成功，返回字符串"STANDARD"
            Base64Format::UrlSafe => write!(f, "URL_SAFE"),  // 如果匹配成功，返回字符串"URL_SAFE"
        }
    }
}
