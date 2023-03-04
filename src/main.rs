use hashbrown::HashMap;
use regex::Regex;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please input your number: \n");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i64 = input.trim().parse().expect("Failed to parse number");
        let result = number_to_zhcn(number);
        println!("Result is: {:?} \n", result);
    }
}

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

fn number_to_zhcn(number: i64) -> String {
    let num: i64 = 100000000000;
    if number > num {
        return "数字不可以大于一千亿！".to_string();
    }
    if number == 0 {
        return "零".to_string();
    }
    let mut map = HashMap::with_capacity(10);
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

    let unit = vec![
        "", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千",
    ];
    // Split and reverse the string
    let num_to_str = number.to_string();
    let re = Regex::new(r"零{2,}").unwrap();
    let mut index = 1;
    let mut cn_to_read = String::new();
    for i in num_to_str.chars() {
        let value = map.get(i.to_string().as_str()).unwrap_or(&"");
        let current_index = num_to_str.len() - index;
        let un = unit[current_index];
        let new_str = match *value {
            "零" => {
                if (current_index) % 4 == 0 {
                    format!("{}", un)
                } else if current_index < 4 && cn_to_read.len() < 1 {
                    String::new()
                } else {
                    format!("{}", value)
                }
            }
            "一" if index == 1 && un == "十" => format!("{}", un),
            "二" if index == 1 && un != "十" && num_to_str.len() > 1 => format!("{}{}", "两", un),
            _ => format!("{}{}", value, un),
        };
        cn_to_read = format!("{}{}", cn_to_read, new_str);
        index += 1;
    }

    cn_to_read = re.replace_all(&cn_to_read, "零").to_string();
    cn_to_read = cn_to_read
        .replace("零万", "万")
        .replace("零亿", "亿")
        .replace("亿万", "亿");

    cn_to_read
}
