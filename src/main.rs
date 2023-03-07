use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

lazy_static! {
    static ref MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("0", "零");
        map.insert("1", "一");
        map.insert("2", "二");
        map.insert("3", "三");
        map.insert("4", "四");
        map.insert("5", "五");
        map.insert("6", "六");
        map.insert("7", "七");
        map.insert("8", "八");
        map.insert("9", "九");
        map
    };
    static ref UNIT: Vec<&'static str> =
        vec!["", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千"];
    static ref RE: Regex = Regex::new(r"零{2,}").unwrap();
    static ref TOO_LARGE: &'static str = "数字不可以大于一千亿！";
    static ref ZERO: &'static str = "零";
    static ref MAX_NUMBER: i64 = 100000000000;
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Please input your number: \n");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please input a valid number\n");
                continue;
            }
        };

        let result = Conversion::number_to_zhcn(number);
        println!("Result is: {:?} \n", result);
    }
}

pub struct Conversion {
    pub id: i32,
}
impl Conversion {
    /// Converts a number to Chinese text and outputs it.
    /// The number must be less than a billion.
    ///
    /// # Example
    /// ```
    /// assert_eq!(number_to_zhcn(0), "零");
    /// assert_eq!(number_to_zhcn(123456), "十二万三千四百五十六");
    /// assert_eq!(number_to_zhcn(100010001), "一亿零一万零一");
    /// assert_eq!(number_to_zhcn(100000000001), "数字不可以大于一千亿！");
    /// assert_eq!(number_to_zhcn(100000000000), "一千亿");
    /// ```

    pub fn number_to_zhcn(number: i64) -> String {
        if number > *MAX_NUMBER {
            return TOO_LARGE.to_string();
        }
        if number == 0 {
            return ZERO.to_string();
        }

        let num_to_str = number.to_string();
        let mut index = 1;
        let mut cn_to_read = String::new();
        for i in num_to_str.chars() {
            let value = MAP.get(i.to_string().as_str()).unwrap_or(&"");
            let current_index = num_to_str.len() - index;
            let un = UNIT[current_index];
            let new_str = match *value {
                "零" => {
                    if current_index % 4 == 0 {
                        format!("{}", un)
                    } else if current_index < 4 {
                        match num_to_str[index..num_to_str.len()].parse::<i64>() {
                            Ok(n) if n == 0 => String::new(),
                            Err(_) => String::new(),
                            _ => {
                                format!("{}", value)
                            }
                        }
                    } else {
                        format!("{}", value)
                    }
                }
                "一" if index == 1 && un == "十" => format!("{}", un),
                "二" if index == 1 && un != "十" && num_to_str.len() > 1 => {
                    format!("{}{}", "两", un)
                }
                _ => format!("{}{}", value, un),
            };
            cn_to_read = format!("{}{}", cn_to_read, new_str);
            index += 1;
        }

        cn_to_read = RE.replace_all(&cn_to_read, "零").to_string();
        cn_to_read = cn_to_read
            .replace("零万", "万")
            .replace("零亿", "亿")
            .replace("亿万", "亿");

        cn_to_read
    }
}

#[cfg(test)]
mod tests {
    use crate::Conversion;

    #[test]
    fn test_zero() {
        assert_eq!(Conversion::number_to_zhcn(0), "零");
    }

    #[test]
    fn test_123456() {
        assert_eq!(Conversion::number_to_zhcn(123456), "十二万三千四百五十六");
    }

    #[test]
    fn test_100010001() {
        assert_eq!(Conversion::number_to_zhcn(100010001), "一亿零一万零一");
    }

    #[test]
    fn test_one_hundred_billion() {
        assert_eq!(Conversion::number_to_zhcn(100000000000), "一千亿");
    }

    #[test]
    fn test_greater_than_one_hundred_billion() {
        assert_eq!(
            Conversion::number_to_zhcn(100000000001),
            "数字不可以大于一千亿！"
        );
    }
}
