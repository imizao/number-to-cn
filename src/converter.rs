use hashbrown::HashMap;
use lazy_static::lazy_static;

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

pub struct Converter;

impl Converter {
    /// Converts a number to Chinese text and outputs it.
    /// The absolute value of the number must not be greater than one hundred billion.
    ///
    /// # Example
    /// ```
    /// use test_demo::converter::Converter;  // 首先导入 Converter
    /// assert_eq!(Converter::number_to_zhcn(0), "零");
    /// assert_eq!(Converter::number_to_zhcn(123456), "十二万三千四百五十六");
    /// assert_eq!(Converter::number_to_zhcn(100010001), "一亿零一万零一");
    /// assert_eq!(Converter::number_to_zhcn(100000000001), "数字不可以大于一千亿！");
    /// assert_eq!(Converter::number_to_zhcn(100000000000), "一千亿");
    /// assert_eq!(Converter::number_to_zhcn(-123), "负一百二十三");
    /// ```
    pub fn number_to_zhcn(number: i64) -> String {
        if number > *MAX_NUMBER || number < -*MAX_NUMBER {
            return TOO_LARGE.to_string();
        }
        if number == 0 {
            return ZERO.to_string();
        }

        let negative = number < 0;
        let abs_number = if negative { -number } else { number };

        let num_to_str: Vec<char> = abs_number.to_string().chars().collect();
        let mut cn_to_vec: Vec<String> = vec![];
        let mut zero_flag = false;

        for (index, &digit) in num_to_str.iter().enumerate() {
            let value = *MAP.get::<str>(&digit.to_string()).unwrap_or(&"");
            let current_index = num_to_str.len() - index - 1;
            let unit = UNIT[current_index];

            match (value, unit, index, num_to_str.len()) {
                ("零", _, _, _) if current_index % 4 == 0 => {
                    // 处理“万”或“亿”位上的零
                    cn_to_vec.push(unit.to_string());
                }
                ("零", _, _, _) => {
                    // 处理普通零
                    zero_flag = true;
                }
                ("一", "十", 0, _) => {
                    // 处理“一十”的情况
                    cn_to_vec.push(unit.to_string());
                }
                ("二", _, _, _) => {
                    // 处理“二”替换为“两”的情况
                    let replace_two = match (unit, index, num_to_str.len()) {
                        ("十", _, _) => false,                          // 十位上的“二”不替换
                        (_, 0, len) if len > 1 && unit != "十" => true, // 最高位的“二”替换为“两”
                        ("百", _, _) | ("千", _, _) => true, // 百位和千位的“二”替换为“两”
                        _ => false,                          // 其他情况不替换
                    };

                    if zero_flag {
                        cn_to_vec.push(ZERO.to_string());
                        zero_flag = false;
                    }

                    if replace_two {
                        cn_to_vec.push(format!("两{}", unit));
                    } else {
                        cn_to_vec.push(format!("{}{}", value, unit));
                    }
                }
                _ => {
                    // 处理普通数字
                    if zero_flag {
                        cn_to_vec.push(ZERO.to_string());
                        zero_flag = false;
                    }
                    cn_to_vec.push(format!("{}{}", value, unit));
                }
            }
        }

        // 拼接结果并替换“亿万”为“亿”
        let result = cn_to_vec.join("").replace("亿万", "亿");

        if negative {
            format!("负{}", result)
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_zhcn() {
        assert_eq!(Converter::number_to_zhcn(0), "零");
        assert_eq!(Converter::number_to_zhcn(20), "二十");
        assert_eq!(Converter::number_to_zhcn(10), "十");
        assert_eq!(Converter::number_to_zhcn(11), "十一");
        assert_eq!(Converter::number_to_zhcn(19), "十九");
        assert_eq!(Converter::number_to_zhcn(99), "九十九");
        assert_eq!(Converter::number_to_zhcn(101), "一百零一");
        assert_eq!(Converter::number_to_zhcn(1001), "一千零一");
        assert_eq!(Converter::number_to_zhcn(10000), "一万");
        assert_eq!(Converter::number_to_zhcn(10001), "一万零一");
        assert_eq!(Converter::number_to_zhcn(100000000), "一亿");
        assert_eq!(Converter::number_to_zhcn(100000001), "一亿零一");
        assert_eq!(Converter::number_to_zhcn(100010000), "一亿零一万");
        assert_eq!(Converter::number_to_zhcn(100100000), "一亿零一十万");
        assert_eq!(Converter::number_to_zhcn(2), "二");
        assert_eq!(Converter::number_to_zhcn(12), "十二");
        assert_eq!(Converter::number_to_zhcn(22), "二十二");
        assert_eq!(Converter::number_to_zhcn(200), "两百");
        assert_eq!(Converter::number_to_zhcn(2000), "两千");
        assert_eq!(Converter::number_to_zhcn(20000), "两万");
        assert_eq!(Converter::number_to_zhcn(200000000), "两亿");
        assert_eq!(Converter::number_to_zhcn(123456), "十二万三千四百五十六");
        assert_eq!(Converter::number_to_zhcn(202300), "二十万两千三百");
        assert_eq!(Converter::number_to_zhcn(2000001), "两百万零一");
        assert_eq!(Converter::number_to_zhcn(100010001), "一亿零一万零一");
        assert_eq!(
            Converter::number_to_zhcn(20202020202),
            "两百零二亿零两百零二万零两百零二"
        );
        assert_eq!(Converter::number_to_zhcn(100000000000), "一千亿");
        assert_eq!(
            Converter::number_to_zhcn(100000000001),
            "数字不可以大于一千亿！"
        );
        assert_eq!(Converter::number_to_zhcn(-1), "负一");
        assert_eq!(Converter::number_to_zhcn(-10), "负十");
        assert_eq!(Converter::number_to_zhcn(-101), "负一百零一");
        assert_eq!(Converter::number_to_zhcn(-1001), "负一千零一");
        assert_eq!(Converter::number_to_zhcn(-100010001), "负一亿零一万零一");
        assert_eq!(Converter::number_to_zhcn(-100000000000), "负一千亿");
        assert_eq!(
            Converter::number_to_zhcn(-100000000001),
            "数字不可以大于一千亿！"
        );
        assert_eq!(
            Converter::number_to_zhcn(i64::MAX),
            "数字不可以大于一千亿！"
        );
        assert_eq!(
            Converter::number_to_zhcn(i64::MIN),
            "数字不可以大于一千亿！"
        );
    }
}
