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
# 克隆仓库
git clone https://github.com/gantoho/fmto.git
cd fmto

# 构建项目
cargo build --release

# 安装fmto命令
cargo install --path .
```

## 使用方法

### 图形界面

1. 直接运行程序启动图形界面：
```bash
fmto
```
或
```bash
cargo run
```

2. 使用 `-g` 或 `--gui` 参数启动图形界面：
```bash
fmto -g
```
或
```bash
cargo run -- -g
```

图形界面功能：
- 选择输入文件
- 选择输出目录
- 选择输出格式（可多选）
- 显示转换状态和错误信息

### 命令行界面

1. 使用输入文件名和输出文件名：
```bash
fmto -i input.json -o output.yaml
```

在fmto指令无法使用的时候，还可以直接使用以下指令
```bash
cargo run -- -i input.json -o output.yaml
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
cargo run -- -i input.json -o output_dir/config.yaml
```

4. 同时转换为多个格式：
```bash
# 指定多个输出文件
fmto -i config.conf -o config.json config.toml config.yaml

# 指定多个输出格式
fmto -i config.conf -o config.json config.toml config.yaml -t json toml yaml

# 使用输出目录并转换为多个格式
fmto -i config.conf -d output_dir -t json toml yaml

# 使用输出目录并转换为所有支持的格式
fmto -i config.conf -d output_dir
```

### 完整示例

1. JSON 转多个格式：
```bash
# 转换为 YAML 和 TOML
fmto -i config.json -o config.yaml config.toml

# 指定格式转换
fmto -i config.json -o config.yaml config.toml -t yaml toml

# 使用输出目录
fmto -i config.json -d output_dir -t yaml toml
```

2. HOCON 转多个格式：
```bash
# 转换为 JSON、YAML 和 TOML
fmto -i config.conf -o config.json config.yaml config.toml

# 指定格式转换
fmto -i config.conf -o config.json config.yaml config.toml -t json yaml toml

# 使用输出目录
fmto -i config.conf -d output_dir -t json yaml toml
```

3. 转换为所有支持的格式：
```bash
# 使用输出目录
fmto -i config.conf -d output_dir
```

### 命令行参数

- `-i, --input <INPUT>`: 输入文件路径（可选，不指定则启动图形界面）
- `-o, --output <OUTPUT>`: 输出文件路径（可选，可以指定多个）
- `-d, --output-dir <OUTPUT_DIR>`: 输出目录（可选）
- `-f, --input-format <INPUT_FORMAT>`: 输入文件格式（可选，将根据文件扩展名自动检测）
- `-t, --output-format <OUTPUT_FORMAT>`: 输出文件格式（可选，可以指定多个，与输出文件一一对应）
- `-g, --gui`: 启动图形界面（可选）

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
5. 当指定多个输出文件时，如果不指定输出格式，程序会根据文件扩展名自动检测格式
6. 当使用输出目录时，如果不指定输出格式，程序会转换为所有支持的格式
7. 如果不指定输入文件或使用 `-g` 参数，程序将启动图形界面

## 错误处理

程序会在以下情况下报错：

1. 输入文件不存在
2. 无法确定输入或输出格式
3. 输入文件格式无效
4. 输出目录创建失败
5. 文件写入失败
6. 输出格式与输出文件数量不匹配

## 开发

```bash
# 克隆仓库
git clone https://github.com/gantoho/fmto.git
cd fmto

# 构建项目
cargo build

# 运行测试
cargo test

# 安装
cargo install --path .
```