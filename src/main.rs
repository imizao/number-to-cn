use hashbrown::HashMap;
use lazy_static::lazy_static;
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

pub(crate) struct Conversion;

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

        let num_to_str: Vec<char> = number.to_string().chars().collect();
        let mut cn_to_vec: Vec<String> = vec![];
        for (index, &digit) in num_to_str.iter().enumerate() {
            let value = *MAP.get::<str>(&digit.to_string()).unwrap_or(&"");
            let current_index = num_to_str.len() - index - 1;
            let unit = UNIT[current_index];

            let new_str = match (value, unit, index, num_to_str.len()) {
                ("零", _, _, _) if current_index % 4 == 0 => unit.to_string(),
                ("零", _, _, _) => {
                    let remaining: i64 = num_to_str[index..].iter().collect::<String>().parse().unwrap_or(0);
                    if remaining == 0 { String::new() } else { value.to_string() }
                }
                ("一", "十", 0, _) => unit.to_string(),
                ("二", _, 0, len) if unit != "十" && len > 1 => format!("两{}", unit),
                _ => format!("{}{}", value, unit),
            };

            if new_str != "零" || !cn_to_vec.ends_with(&[ZERO.to_string()]) {
                if new_str == "万" || new_str == "亿" {
                    cn_to_vec.pop();
                }
                cn_to_vec.push(new_str);
            }
        }
        cn_to_vec.join("").replace("亿万", "亿")
        
    }
}

#[cfg(test)]
mod tests {
    use crate::Conversion;

    #[test]
    fn test_number_to_zhcn() {
        assert_eq!(Conversion::number_to_zhcn(0), "零");
        assert_eq!(Conversion::number_to_zhcn(20), "二十");
        assert_eq!(Conversion::number_to_zhcn(123456), "十二万三千四百五十六");
        assert_eq!(Conversion::number_to_zhcn(2000001), "两百万零一");
        assert_eq!(Conversion::number_to_zhcn(100010001), "一亿零一万零一");
        assert_eq!(Conversion::number_to_zhcn(100000000000), "一千亿");
        assert_eq!(Conversion::number_to_zhcn(100000000001), "数字不可以大于一千亿！");
    }
}
