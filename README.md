
## 小于一千亿的数字转中文文字并输出
```
number_to_zhcn(123456); // 一十二万三千四百五十六
number_to_zhcn(211133456); // 二亿一千一百一十三万三千四百五十六
number_to_zhcn(10013000); // 一千零一万三千
number_to_zhcn(102013000); // 一亿零二百零一万三千
number_to_zhcn(1000123000); // 一十亿零一十二万三千
number_to_zhcn(1007890000); // 一十亿零七百八十九万
number_to_zhcn(100000000000); // 一千亿
number_to_zhcn(100000123100); // 数字不可以大于一千亿！
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
