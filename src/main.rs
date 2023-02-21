use hashbrown::HashMap;

fn main() {
    number_to_zhcn(123456);
    number_to_zhcn(211133456);
    number_to_zhcn(10013000);
    number_to_zhcn(1000123000);
    number_to_zhcn(1007890000);
    number_to_zhcn(100000000000);
    number_to_zhcn(100000123100);
}

fn number_to_zhcn(number: i64) {
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
        return println!("数字不可以大于一千亿！");
    }
    let mut name = String::from("");
    let str = number.to_string();
    let mut str_arr: Vec<&str> = str.split("").collect();
    str_arr.reverse();
    let mut index = 0;
    for i in str_arr.iter() {
        for (key, value) in map.iter() {
            if i == key {
                let un = if i.to_string() == "0".to_string() && index % 4 != 0 {
                    String::from("")
                } else {
                    unit[index].to_string()
                };
                name = format!("{}{un}{name}",value.to_string());
                index += 1;
            }
        }
    }
    name = name.replace("零零零", "零");
    name = name.replace("零零", "零");
    name = name.replace("零万", "万");
    name = name.replace("零亿", "亿");
    name = name.replace("万零", "万");
    name = name.replace("亿零", "亿");
    name = name.replace("亿万", "亿");
    println!("{name}");
}
