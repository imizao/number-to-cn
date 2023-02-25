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
    let str = number.to_string();
    let mut str_arr: Vec<&str> = str.split("").collect();
    str_arr.reverse();

    let mut name = String::new();
    let mut index = 0;
    let re = Regex::new(r"零{2,}").unwrap();

    for i in str_arr.iter().filter(|i| !i.is_empty()) {
        // Get the value of `i`, if there is none, default to `""`
        let value = map.get(i).unwrap_or(&"");
        let un = match value {
            &"零" if index % 4 != 0 || index < 4 => "",
            _ => unit[index],
        };
        // `一十` -> `十`; `二` -> `两`
        let new_str = match value {
            &"零" if index < 4 && name.len() < 1 => String::new(),
            &"一" if index == str.len() - 1 && un == "十" => format!("{}", un),
            &"二" if index == str.len() - 1 => format!("{}{}", "两", un),
            _ => format!("{}{}", value, un),
        };
        name = format!("{}{}", new_str, name);
        index += 1;
    }
    name = re.replace_all(&name, "零").to_string();
    name = name.replace("零万", "万")
                .replace("零亿", "亿")
                .replace("亿万", "亿");

    name = name.trim_start().to_string();

    name
}
