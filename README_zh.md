# MP 编程语言

[English Version](./README_en.md)

## 简介

MP 是一种简单易学的动态类型编程语言。它语法简洁、语义清晰，并提供完整的 IDE 支持（通过 LSP）。

## 特性

- **动态类型**：无需声明变量类型
- **内置类型**：数字（整数/浮点）、字符串、布尔值、数组、对象、函数、Nil
- **控制流**：if/else 条件语句、while 循环、break/continue
- **函数**：支持用户自定义函数和返回值
- **代码块**：带自动返回值的代码块表达式
- **结构体**：用户自定义复合类型
- **REPL**：交互式编程环境
- **格式化工具**：内置代码格式化器
- **LSP 支持**：完整的语言服务器协议实现，支持 IDE 集成

## 语法

### 关键字

| 关键字 | 说明 |
|--------|------|
| `if`   | 条件语句 |
| `else` | 备选分支 |
| `while`| 循环语句 |
| `let`  | 变量声明 |
| `fn`   | 函数定义 |
| `struct`| 结构体定义 |
| `break` | 提前退出循环 |
| `continue` | 跳到下一次循环 |
| `return` | 函数返回值 |

### 数据类型

| 类型 | 说明 | 示例 |
|------|------|------|
| Number | 整数或浮点数 | `42`, `3.14` |
| String | 文本序列 | `"hello"` |
| Boolean | 布尔值 | `true`, `false` |
| Array | 有序集合 | `[1, 2, 3]` |
| Object | 键值对 | `{"key": "value"}` |
| Function | 可调用函数 | `fn add(a, b) { ... }` |
| Struct | 用户定义类型 | `struct Person { name, age }` |
| Nil | 空值 | `nil` |

### 运算符

| 运算符 | 说明 |
|--------|------|
| `+` | 加法 / 字符串连接 |
| `-` | 减法 |
| `*` | 乘法 |
| `/` | 除法 |
| `==` | 等于 |
| `!=` | 不等于 |
| `>` | 大于 |
| `<` | 小于 |
| `>=` | 大于等于 |
| `<=` | 小于等于 |

### 内置函数

| 函数 | 说明 |
|------|------|
| `print(expr)` | 在控制台打印值 |
| `input()` | 从控制台读取字符串 |
| `int(value)` | 转换为整数 |
| `float(value)` | 转换为浮点数 |
| `str(value)` | 转换为字符串 |
| `len(collection)` | 获取字符串/数组/对象的长度 |
| `type(expr)` | 获取表达式的类型 |
| `random([min, max])` | 生成随机数 |
| `push(array, item)` | 向数组添加元素 |
| `pop(array)` | 移除数组最后一个元素 |
| `time()` | 获取当前时间戳 |

## 使用方法

### 运行 MP 程序

```bash
# 运行文件
mp filename.mp

# 启动 REPL
mp
```

### REPL 命令

| 命令 | 说明 |
|------|------|
| `exit` | 退出 REPL |
| `help` | 显示可用命令 |
| `clear` | 清除环境 |

## 示例

### Hello World
```
print("你好，世界！");
```

### 变量
```
let name = "张三";
let age = 25;
let score = 98.5;
```

### 函数
```
fn add(a, b) {
    return a + b;
}

let result = add(1, 2);
```

### 条件语句
```
if (age >= 18) {
    print("成年人");
} else {
    print("未成年人");
}
```

### 循环
```
let i = 0;
while (i < 5) {
    print(i);
    i = i + 1;
}
```

### Break 和 Continue
```
let j = 0;
while (j < 10) {
    j = j + 1;
    if (j == 3) {
        continue;
    }
    if (j == 7) {
        break;
    }
    print(j);
}
```

### 数组
```
let arr = [1, 2, 3, 4, 5];
print(len(arr));
push(arr, 6);
let last = pop(arr);
```

### 对象
```
let person = {
    "name": "李四",
    "age": 30
};
print(person["name"]);
```

### 结构体
```
struct Point {
    x,
    y
}

let p = Point(10, 20);
print(p.x);
```

### 类型检查
```
let num = 42;
print(type(num));  // int

let text = "hello";
print(type(text));  // string
```

### 类型转换
```
let strNum = "123";
let num = int(strNum);

let floatNum = float("3.14");
let strVal = str(42);
```

### 随机数
```
let dice = random(1, 7);
let randomFloat = random(10.0);
```

### 注释

```
// 单行注释

/*
多行
注释
*/
```

## 项目结构

```
mp_lang/
├── src/
│   ├── lexer/          # 词法分析
│   ├── parser/         # 语法解析
│   ├── runtime/        # 解释器与求值器
│   ├── lsp/            # 语言服务器协议
│   ├── formatter.rs    # 代码格式化器
│   └── lib.rs          # 核心库
├── [examples](./examples)  # 示例程序
├── tests/              # 测试套件
└── vscode-extension/   # VS Code 插件
```

## 构建

```bash
# 构建项目
cargo build --release

# 运行测试
cargo test
```

## IDE 支持

MP 实现了完整的 LSP（语言服务器协议），支持以下功能：

- **自动补全**：智能代码提示
- **悬停提示**：鼠标悬停显示类型信息
- **跳转到定义**：导航到符号定义处
- **查找引用**：查找符号的所有使用位置
- **诊断信息**：实时错误报告
- **内联提示**：显示变量内联类型
- **工作区符号**：跨文件搜索
- **代码格式化**：自动代码风格调整

建议安装 `vscode-extension/` 目录下的 VS Code 插件以获得最佳开发体验。
