# dbjump

快速数据库连接管理工具 - Quick database connection manager

`dbjump` 是一个命令行工具，让您通过简短的别名快速连接各种数据库，无需记住繁琐的连接参数。

## 特性

- 🚀 **快速连接**：通过别名一键连接数据库
- 🔧 **配置集中**：TOML 格式配置文件，简洁易读
- 🔒 **安全设计**：自动设置配置文件权限（600），密码不泄露到进程列表
- 🎯 **原生体验**：调用原生 CLI 工具，保持完整交互功能
- 🎨 **Fzf 集成**：支持交互式选择和实时预览
- ⚡ **Shell 集成**：自动配置 `j` 快捷命令（可自定义）和智能补全
- 📦 **单一可执行文件**：Rust 编写，无需额外依赖

## 支持的数据库

- ✅ ClickHouse (使用 `clickhouse client` 命令)
- ✅ PostgreSQL (使用 `psql` 命令)
- ✅ MySQL (使用 `mysql` 命令)
- ✅ MongoDB (使用 `mongosh` 命令)

## 安装

### 前置要求

- Rust 工具链（用于编译）
- 要连接的数据库对应的 CLI 工具：
  - ClickHouse: `clickhouse` 命令行工具
  - PostgreSQL: `psql` 命令行工具
  - MySQL: `mysql` 命令行工具
  - MongoDB: `mongosh` 命令行工具

### 编译和安装

#### 1. 编译二进制文件

```bash
cd dbjump
cargo build --release
```

#### 2. 安装二进制文件

```bash
# 复制到 PATH 中的目录
cp target/release/dbjump ~/.local/bin/
# 或者
sudo cp target/release/dbjump /usr/local/bin/
```

确保安装目录在 PATH 中：

```bash
# 如果使用 ~/.local/bin，确保它在 PATH 中
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### 3. 安装 Oh-My-Zsh 插件（可选但推荐）

```bash
# 复制插件到 oh-my-zsh
mkdir -p ~/.oh-my-zsh/custom/plugins/dbjump
cp -r oh-my-zsh/dbjump/* ~/.oh-my-zsh/custom/plugins/dbjump/
```

然后编辑 `~/.zshrc`，在 plugins 数组中添加 `dbjump`：

```bash
plugins=(git docker ... dbjump)
```

重新加载配置：

```bash
source ~/.zshrc
```

#### 4. 手动集成（不使用 Oh-My-Zsh）

如果您不使用 Oh-My-Zsh，可以在 `~/.zshrc` 中添加以下内容来启用 Shell 集成（包含 `j` 快捷命令和补全）：

```bash
# 在 ~/.zshrc 中添加
eval "$(dbjump shell zsh)"

# 如果想自定义快捷命令名称（默认为 j）
# eval "$(dbjump shell --cmd myjump zsh)"
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

添加数据库连接配置。**注意：所有连接参数（host, port, user, password）都是可选的**，如果不指定，将使用数据库 CLI 工具的默认值。

```toml
# ClickHouse - 完整配置
[[database]]
alias = "prod-clickhouse"
engine = "clickhouse"
host = "192.168.1.100"
port = 9000
user = "admin"
password = "secret123"
database = "default"  # 可选
options = ["--multiline"]  # 可选

# ClickHouse - 使用默认值（localhost:9000, user=default）
[[database]]
alias = "local-clickhouse"
engine = "clickhouse"
# 不指定任何参数，使用 clickhouse client 的默认值

# PostgreSQL - 完整配置
[[database]]
alias = "dev-postgres"
engine = "postgresql"
host = "localhost"
port = 5432
user = "postgres"
password = "devpass"
database = "myapp"  # 可选
options = []  # 可选

# PostgreSQL - 使用默认值（localhost:5432, user=$USER）
[[database]]
alias = "local-postgres"
engine = "postgresql"
database = "mydb"  # 只指定数据库名

# MySQL - 完整配置
[[database]]
alias = "dev-mysql"
engine = "mysql"
host = "localhost"
port = 3306
user = "root"
password = "secret123"
database = "myapp"

# MySQL - 使用默认值（localhost:3306）
[[database]]
alias = "local-mysql"
engine = "mysql"
database = "mydb"

# MongoDB - 完整配置
[[database]]
alias = "dev-mongo"
engine = "mongodb"
host = "localhost"
port = 27017
user = "admin"
password = "secret123"
database = "myapp"

# MongoDB - 使用默认值（localhost:27017）
[[database]]
alias = "local-mongo"
engine = "mongodb"
```

### 3. 验证配置

```bash
dbjump validate
```

### 4. 连接数据库

#### 使用快捷命令 `j` (推荐)

Shell 集成提供了 `j` 命令（默认），它是连接功能的简写。

```bash
# 交互式选择 (需要 fzf)
j

# 直接连接
j prod-clickhouse

# 传递额外参数
j prod-clickhouse --query "SELECT 1"
```

#### 使用完整命令

```bash
# 直接连接
dbjump connect prod-clickhouse
```

#### 交互式选择（使用 fzf）

如果安装了 fzf，直接运行 `dbjump` 或 `j` 不带参数：

```bash
dbjump connect
# 或
j
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

# 生成 shell 集成脚本
dbjump shell zsh
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
- ClickHouse 密码通过 `CLICKHOUSE_PASSWORD` 环境变量传递，不出现在进程列表中
- PostgreSQL 密码通过 `PGPASSWORD` 环境变量传递，不出现在进程列表中
- MySQL 密码通过 `MYSQL_PWD` 环境变量传递，不出现在进程列表中
- MongoDB 密码通过连接字符串传递（用户名和密码会自动进行 URL 编码）

## 工作原理

`dbjump` 是一个配置管理工具，它不直接实现数据库连接，而是：

1. 读取配置文件中的连接参数
2. 构建对应数据库 CLI 工具的命令：
   - ClickHouse: `clickhouse client [参数]`
   - PostgreSQL: `psql [参数]`
   - MySQL: `mysql [参数]`
   - MongoDB: `mongosh [连接字符串] [参数]`
3. 在 Unix 系统上使用 `exec()` 替换当前进程，完整保留交互式体验
4. 在非 Unix 系统上使用 `spawn()` 执行命令

这样的设计保证了：
- 完整的原生 CLI 功能和交互体验
- 所有数据库特性都可用（历史记录、快捷键等）
- 无需为每个数据库实现连接逻辑

### 参数优先级

所有连接参数都是可选的。当参数未在配置文件中指定时，数据库 CLI 工具将使用其默认值：

- **ClickHouse**: 默认 `localhost:9000`, user=`default`
- **PostgreSQL**: 默认 `localhost:5432`, user=当前系统用户
- **MySQL**: 默认 `localhost:3306`, user=当前系统用户
- **MongoDB**: 默认 `localhost:27017`

这样可以最小化配置文件的复杂度，只需指定与默认值不同的参数。
