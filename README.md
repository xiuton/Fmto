# Fmto - 配置文件格式转换工具

Fmto 是一个用 Rust 编写的命令行工具，用于在不同格式的配置文件之间进行转换。支持以下格式：

- INI
- XML
- HOCON
- ENV
- JSON
- YAML
- TOML

## 安装

```bash
cargo install --path .
```

## 使用方法

### 基本用法

1. 使用输入文件名和输出文件名：
```bash
fmto -i input.json -o output.yaml
```

2. 指定输入和输出格式：
```bash
fmto -i input.json -o output.yaml -f json -t yaml
```

3. 使用输出目录（自动使用输入文件名）：
```bash
fmto -i input.json -d output_dir -t yaml
```
or
```bash
cargo run -- -i config.json -o output_dir/config.yaml
```

### 完整示例

1. JSON 转 YAML：
```bash
# 基本转换
fmto -i config.json -o config.yaml

# 指定格式
fmto -i config.json -o config.yaml -f json -t yaml

# 使用输出目录
fmto -i config.json -d output_dir -t yaml
```

2. YAML 转 TOML：
```bash
# 基本转换
fmto -i config.yaml -o config.toml

# 指定格式
fmto -i config.yaml -o config.toml -f yaml -t toml

# 使用输出目录
fmto -i config.yaml -d output_dir -t toml
```

3. INI 转 JSON：
```bash
# 基本转换
fmto -i config.ini -o config.json

# 指定格式
fmto -i config.ini -o config.json -f ini -t json

# 使用输出目录
fmto -i config.ini -d output_dir -t json
```

4. HOCON 转 YAML：
```bash
# 基本转换
fmto -i config.conf -o config.yaml

# 指定格式
fmto -i config.conf -o config.yaml -f hocon -t yaml

# 使用输出目录
fmto -i config.conf -d output_dir -t yaml
```

5. ENV 转 TOML：
```bash
# 基本转换
fmto -i config.env -o config.toml

# 指定格式
fmto -i config.env -o config.toml -f env -t toml

# 使用输出目录
fmto -i config.env -d output_dir -t toml
```

6. XML 转 JSON：
```bash
# 基本转换
fmto -i config.xml -o config.json

# 指定格式
fmto -i config.xml -o config.json -f xml -t json

# 使用输出目录
fmto -i config.xml -d output_dir -t json
```

### 命令行参数

- `-i, --input <INPUT>`: 输入文件路径（必需）
- `-o, --output <OUTPUT>`: 输出文件路径（可选）
- `-d, --output-dir <OUTPUT_DIR>`: 输出目录（可选）
- `-f, --input-format <INPUT_FORMAT>`: 输入文件格式（可选，将根据文件扩展名自动检测）
- `-t, --output-format <OUTPUT_FORMAT>`: 输出文件格式（可选，将根据文件扩展名自动检测）

### 支持的格式

| 格式 | 文件扩展名 | 说明 |
|------|------------|------|
| INI  | .ini       | INI 配置文件格式 |
| XML  | .xml       | XML 文档格式 |
| HOCON| .conf      | HOCON 配置文件格式 |
| ENV  | .env       | 环境变量文件格式 |
| JSON | .json      | JSON 数据格式 |
| YAML | .yaml/.yml | YAML 配置文件格式 |
| TOML | .toml      | TOML 配置文件格式 |

## 注意事项

1. 如果不指定输出文件路径（-o）或输出目录（-d），程序将使用输入文件名加上输出格式的扩展名作为输出文件名
2. 如果指定了输出目录，程序会自动创建不存在的目录
3. 如果不指定输入或输出格式，程序会根据文件扩展名自动检测格式
4. 所有转换都会保持数据的结构和类型信息

## 错误处理

程序会在以下情况下报错：

1. 输入文件不存在
2. 无法确定输入或输出格式
3. 输入文件格式无效
4. 输出目录创建失败
5. 文件写入失败

## 开发

```bash
# 克隆仓库
git clone https://github.com/yourusername/fmto.git
cd fmto

# 构建项目
cargo build

# 运行测试
cargo test

# 安装
cargo install --path .
```