# dbjump

快速数据库连接管理工具 - Quick database connection manager

`dbjump` 是一个命令行工具，让您通过简短的别名快速连接各种数据库，无需记住繁琐的连接参数。

## 特性

- 🚀 **快速连接**：通过别名一键连接数据库
- 🔧 **配置集中**：TOML 格式配置文件，简洁易读
- 🔒 **安全设计**：自动设置配置文件权限（600），密码不泄露到进程列表
- 🎯 **原生体验**：调用原生 CLI 工具，保持完整交互功能
- 🎨 **Fzf 集成**：支持交互式选择和实时预览
- ⚡ **Shell 补全**：通过 oh-my-zsh 插件提供智能补全
- 📦 **单一可执行文件**：Rust 编写，无需额外依赖

## 支持的数据库

- ✅ ClickHouse
- ✅ PostgreSQL

更多数据库支持即将到来...

## 安装

### 前置要求

- Rust 工具链（用于编译）
- 要连接的数据库对应的 CLI 工具：
  - ClickHouse: `clickhouse-client`
  - PostgreSQL: `psql`

### 编译和安装

```bash
# 克隆仓库
git clone <repository-url>
cd dbjump

# 运行安装脚本
./scripts/install.sh
```

安装脚本会：
1. 编译 release 版本的二进制文件
2. 安装到 `~/.local/bin/dbjump`
3. 生成 shell 补全脚本
4. 安装 oh-my-zsh 插件（如果检测到 oh-my-zsh）

### 启用插件

#### 使用 Oh-My-Zsh

编辑 `~/.zshrc`，在 plugins 数组中添加 `dbjump`：

```bash
plugins=(git docker ... dbjump)
```

然后重新加载配置：

```bash
source ~/.zshrc
```

#### 手动加载（不使用 Oh-My-Zsh）

在 `~/.zshrc` 中添加：

```bash
source ~/.config/dbjump/plugin/dbjump.plugin.zsh
```

## 使用方法

### 1. 初始化配置文件

```bash
dbjump init
```

这会在 `~/.config/dbjump/config.toml` 创建配置文件模板。

### 2. 编辑配置文件

```bash
vim ~/.config/dbjump/config.toml
```

添加数据库连接配置：

```toml
[[database]]
alias = "prod-clickhouse"
engine = "clickhouse"
host = "192.168.1.100"
port = 9000
user = "admin"
password = "secret123"
database = "default"  # 可选
options = ["--multiline"]  # 可选

[[database]]
alias = "dev-postgres"
engine = "postgresql"
host = "localhost"
port = 5432
user = "postgres"
password = "devpass"
database = "myapp"  # 可选
options = []  # 可选
```

### 3. 验证配置

```bash
dbjump validate
```

### 4. 连接数据库

#### 直接连接（通过别名）

```bash
dbjump prod-clickhouse
```

#### 交互式选择（使用 fzf）

如果安装了 fzf，直接运行 `dbjump` 不带参数：

```bash
dbjump
```

这会打开一个交互式界面，让您：
- 模糊搜索所有别名
- 在预览窗口查看连接信息（密码已隐藏）
- 按 Enter 连接选中的数据库
- 按 Ctrl+/ 切换预览窗口

#### 传递额外参数

```bash
dbjump prod-clickhouse --query "SELECT version()"
```

### 5. 其他命令

```bash
# 列出所有配置的数据库
dbjump list

# 以 JSON 格式列出
dbjump list --format json

# 查看某个数据库的连接信息（密码隐藏）
dbjump info prod-clickhouse

# 生成 shell 补全脚本
dbjump completions zsh
```

## 配置

### 配置文件路径

默认路径：`~/.config/dbjump/config.toml`

您可以通过环境变量自定义路径：

```bash
export DBJUMP_CONFIG=/path/to/your/config.toml
```

### 安全性

- 配置目录自动设置 700 权限（仅所有者可访问）
- 配置文件自动设置 600 权限（仅所有者可读写）
- ClickHouse 密码通过 `--password` 参数传递
- PostgreSQL 密码通过 `PGPASSWORD` 环境变量传递，不出现在进程列表中

## 工作原理

`dbjump` 是一个配置管理工具，它不直接实现数据库连接，而是：

1. 读取配置文件中的连接参数
2. 构建对应数据库 CLI 工具的命令
3. 在 Unix 系统上使用 `exec()` 替换当前进程，完整保留交互式体验
4. 在非 Unix 系统上使用 `spawn()` 执行命令

这样的设计保证了：
- 完整的原生 CLI 功能和交互体验
- 所有数据库特性都可用（历史记录、快捷键等）
- 无需为每个数据库实现连接逻辑

## 开发

### 运行测试

```bash
cargo test
```

### 本地开发

```bash
# 编译
cargo build

# 运行
cargo run -- init
cargo run -- list

# 格式化
cargo fmt

# 检查
cargo clippy
```

## 贡献

欢迎提交 Issue 和 Pull Request！
