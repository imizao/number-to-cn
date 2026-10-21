const DIGITS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
const UNITS: [&str; 12] = [
    "", "十", "百", "千", "万", "十", "百", "千", "亿", "十", "百", "千",
];
const MAX_NUMBER: i64 = 100_000_000_000;
const TOO_LARGE: &str = "数字不可以大于一千亿！";

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
        // Check the range before negating so i64::MIN cannot overflow.
        if !(-MAX_NUMBER..=MAX_NUMBER).contains(&number) {
            return TOO_LARGE.to_owned();
        }
        if number == 0 {
            return DIGITS[0].to_owned();
        }

        let magnitude = number.abs();
        let mut divisor = 1;
        let mut position = 0;
        while magnitude / divisor >= 10 {
            divisor *= 10;
            position += 1;
        }
        let highest_position = position;
        let mut result = String::new();
        if number < 0 {
            result.push('负');
        }
        let mut pending_zero = false;

        loop {
            let digit = (magnitude / divisor % 10) as usize;
            let unit = UNITS[position];
            if digit == 0 {
                if position % 4 != 0 {
                    pending_zero = true;
                } else if position > 0 && magnitude / divisor % 10_000 != 0 {
                    // Emit a group unit only when that four-digit group is nonzero.
                    // Keep pending_zero across the boundary to preserve existing readings.
                    result.push_str(unit);
                }
            } else {
                if pending_zero {
                    result.push('零');
                    pending_zero = false;
                }
                let omit_one = digit == 1 && unit == "十" && position == highest_position;
                let use_liang = digit == 2
                    && (position % 4 == 2
                        || position % 4 == 3
                        || (position == highest_position && position > 0 && unit != "十"));
                if !omit_one {
                    result.push_str(if use_liang { "两" } else { DIGITS[digit] });
                }
                result.push_str(unit);
            }

            if position == 0 {
                break;
            }
            divisor /= 10;
            position -= 1;
        }
        result
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

    #[test]
    fn preserves_cross_group_readings() {
        let cases = [
            (10_001_000, "一千万零一千"),
            (10_002_000, "一千万零两千"),
            (100_010_001, "一亿零一万零一"),
            (100_000_010, "一亿零一十"),
            (100_020_000, "一亿零二万"),
            (200_020_002, "两亿零二万零二"),
            (1_000_000_001, "十亿零一"),
            (99_999_999, "九千九百九十九万九千九百九十九"),
            (100_000_001, "一亿零一"),
            (99_999_999_999, "九百九十九亿九千九百九十九万九千九百九十九"),
        ];
        for (number, expected) in cases {
            assert_eq!(Converter::number_to_zhcn(number), expected, "{number}");
            assert_eq!(Converter::number_to_zhcn(-number), format!("负{expected}"));
        }
    }

    #[test]
    fn sparse_numbers_have_no_repeated_or_edge_zeros() {
        // Exercise every zero/nonzero placement up to twelve digits,
        // including empty groups and zeros on both sides of 万/亿.
        for mask in 1..4096u32 {
            for digit in [1, 2, 9] {
                let mut number = 0i64;
                let mut place = 1;
                for bit in 0..12 {
                    if mask & (1 << bit) != 0 {
                        number += digit * place;
                    }
                    place *= 10;
                }
                if number > MAX_NUMBER {
                    continue;
                }
                let text = Converter::number_to_zhcn(number);
                assert!(!text.contains("零零"), "{number}: {text}");
                assert!(!text.starts_with('零'), "{number}: {text}");
                assert!(!text.ends_with('零'), "{number}: {text}");
                assert!(!text.contains("亿万"), "{number}: {text}");
                assert_eq!(Converter::number_to_zhcn(-number), format!("负{text}"));
            }
        }
    }
}
