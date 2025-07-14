# Mp programming language
## keywords
| 关键字 | 说明 |
| --- | --- |
| `if` | 条件判断 |
| `else` | 条件判断 |
| `while` | 循环 |
| `let` | 变量声明 |
## 语法
Mp中没有`null`，只有`nil`。

### 语句
在Mp中，任何语句的值都是`nil`，且不能被赋值给变量。
| 语句 | 说明 |
| --- | --- |
|`let a = 10;` | 变量声明 |
| `fn add(a, b) { return a + b; }` | 函数定义 |
| `1+1;` | 以分号结尾的表达式 |
单独成行的[表达式](#表达式)的结尾会自动加上分号变成语句。
位于块表达式最后一行的表达式会自动包装为`return`语句。
### 表达式
任何表达式都有值（可能为`nil`）
| 表达式 | 说明 |
| --- | --- |
| `expr1 + expr2` | 加法 |
| `expr1 - expr2` | 减法 |
| `expr1 * expr2` | 乘法 |
| `expr1 / expr2` | 除法 |
| `expr1 % expr2` | 取模 |
| `expr1 == expr2` | 等于 |
| `expr1!= expr2` | 不等于 |
| `{expr1; expr2;...; exprn;}`| 语句块 |
| `if (expr) { expr1; } else { expr2; }` | 条件判断 |
| `while (expr) { expr1; }` | 循环 |
### 变量声明
```
let a = 10;
```
### 条件判断
```
if (a > 10) {
    print("a is greater than 10");
} else {
    print("a is less than or equal to 10");
}
```
### 循环
```
let i = 0;
while (i < 10) {
    print(i);
    i = i + 1;
}
```
`tip`：while循环将返回一个Vector，包含循环体的最后一个表达式的值。