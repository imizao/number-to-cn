
## Converts a number to Chinese text and outputs it.
### The number must be less than a billion.

```
number_to_zhcn(0); // 零
number_to_zhcn(123456); // 十二万三千四百五十六
number_to_zhcn(100010001); // 一亿零一万零一
number_to_zhcn(100000000001); // 数字不可以大于一千亿！
number_to_zhcn(100000000000); // 一千亿

```

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