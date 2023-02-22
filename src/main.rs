use hashbrown::HashMap;

fn main() {
    assert_eq!(number_to_zhcn(123456), "一十二万三千四百五十六".to_string());
    assert_eq!(number_to_zhcn(211133456), "二亿一千一百一十三万三千四百五十六".to_string());
    assert_eq!(number_to_zhcn(10013000), "一千零一万三千".to_string());
    assert_eq!(number_to_zhcn(102013000), "一亿零二百零一万三千".to_string());
    assert_eq!(number_to_zhcn(1000123000), "一十亿零一十二万三千".to_string());
    assert_eq!(number_to_zhcn(1007890000), "一十亿零七百八十九万".to_string());
    assert_eq!(number_to_zhcn(100000000000), "一千亿".to_string());
    assert_eq!(number_to_zhcn(100000123100), "数字不可以大于一千亿！".to_string());
}

fn number_to_zhcn(number: i64) -> String {
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

    let unit = vec![
        " ", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千"
    ];
    let num: i64 = 100000000000;
    if number > num {
        return "数字不可以大于一千亿！".to_string();
    }
    let mut name = String::from("");
    let str = number.to_string();
    let mut str_arr: Vec<&str> = str.split("").collect();
    str_arr.reverse();
    let mut index = 0;
    for i in str_arr.iter() {
        for (key, value) in map.iter() {
            if i == key {
                let un = if i.to_string() == "0".to_string() && index % 4 != 0 || index == 0 {
                    String::from("")
                } else {
                    unit[index].to_string()
                };
                name = if name.is_empty() && value == &"零" && index < 4 {
                    String::from("")
                } else {
                    format!("{}{un}{name}", value.to_string())
                };
                index += 1;
            }
        }
    }
    name = name.replace("零零零", "零");
    name = name.replace("零零", "零");
    name = name.replace("零万", "万");
    name = name.replace("零亿", "亿");
    name = name.replace("亿万", "亿");

    // let name_arr: Vec<&str> = name.split("").collect();
    // let mut middle = &name_arr[1..name_arr.len() - 1];
    // let last_name = "零".to_string();

    // if let Some(&last_elem) = middle.last() {
    //     if *last_elem.to_string() == last_name {
    //         // 如果最后一个元素是 零，删除它
    //         middle = &name_arr[1..name_arr.len() - 2];
    //     }
    // }
    // let new_name: String = middle.into_iter().map(|c| c.to_string()).collect();
    println!("{name}");
    return name;
}
