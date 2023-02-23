use hashbrown::HashMap;
use regex::Regex;


fn main() {
    assert_eq!(number_to_zhcn(123456), "一十二万三千四百五十六");
    assert_eq!(number_to_zhcn(211133456), "二亿一千一百一十三万三千四百五十六");
    assert_eq!(number_to_zhcn(10013000), "一千零一万三千");
    assert_eq!(number_to_zhcn(102013000), "一亿零二百零一万三千");
    assert_eq!(number_to_zhcn(1000123000), "一十亿零一十二万三千");
    assert_eq!(number_to_zhcn(1007890000), "一十亿零七百八十九万");
    assert_eq!(number_to_zhcn(1000000000001), "数字不可以大于一千亿！");
    assert_eq!(number_to_zhcn(100000000000), "一千亿");
}

fn number_to_zhcn(number: i64) -> String {
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
        "", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千"
    ];
    let num: i64 = 100000000000;
    if number > num {
        println!("数字不可以大于一千亿！");
        return "数字不可以大于一千亿！".to_string();
    }
    let mut name = String::new();
    let str = number.to_string();
    let mut str_arr: Vec<&str> = str.split("").collect();
    str_arr.reverse();
    let mut index = 0;
    let re = Regex::new(r"零{2,}").unwrap();
    for i in str_arr.iter() {
        if !i.is_empty() {
            // let value = match i {
            //     &"0" => {
            //         if index % 4 == 0 {
            //             // 如果是万、亿等级，不加零
            //             String::new()
            //         } else if index > 0 && index % 4 != 0 && str_arr[index - 1] != "0" {
            //             // 如果是十位，且前一位不是 0，不加零
            //             String::new()
            //         } else {
            //             // 其它情况都加零
            //             map.get(i).unwrap_or(&"").to_string()
            //         }
            //     }
            //     _ => map.get(i).unwrap_or(&"").to_string(),
            // };
            let value = map.get(i).unwrap_or(&"").to_string();
            let un = if value.contains("零") && index < 4 {
                String::new()
            } else if value.contains("零") && index % 4 != 0 {
                String::new()
            } else {
                unit[index].to_string()
            };
            let new_str = if value.contains("零") && index < 4 {
                String::new()
            } else {
                format!("{}{}", value, un).to_string()
            };
            name = format!("{}{}", new_str, name);
            index += 1;
        }
    }
    name = re.replace_all(&name, "零").to_string();
    name = name.replace("零万", "万");
    name = name.replace("零亿", "亿");
    name = name.replace("亿万", "亿");
    name = name.trim_start().to_string();
    
    println!("{name}");
    name
}