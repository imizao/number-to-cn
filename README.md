
## Converts a number to Chinese text and outputs it.
### The absolute value of the number must not be greater than one hundred billion.

```
number_to_zhcn(0);     // 零
number_to_zhcn(20);    // 二十
number_to_zhcn(123456);  // 十二万三千四百五十六
number_to_zhcn(202300);  // 二十万两千三百
number_to_zhcn(2000001); // 两百万零一
number_to_zhcn(100010001); // 一亿零一万零一
number_to_zhcn(20202020202); // 两百零二亿零两百零二万零两百零二
number_to_zhcn(100000000000); // 一千亿
number_to_zhcn(100000000001); // 数字不可以大于一千亿！
```

### Conversion rules

- Accepts integers from `-100000000000` to `100000000000`, inclusive.
- Negative numbers start with `负`; zero is `零`.
- Consecutive zero digits are combined; trailing zeros are omitted.
- Preserves cross-group readings such as `10001000` → `一千万零一千`
  and `100010001` → `一亿零一万零一`.
- Uses `两` in hundreds/thousands and for a leading `2` before `万`/`亿`;
  uses `二` in tens and ones. An initial `一十` is shortened to `十`.
- The converter uses only the Rust standard library.

### Clone

```
git clone git@github.com:imizao/number-to-cn.git
```

### Run

```
cd number-to-cn
cargo run
```
`Please input your number:`

### Test

```
cargo test
```
