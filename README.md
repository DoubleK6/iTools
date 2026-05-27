# iTools

基于 Tauri 2 + Rust + Svelte 构建的 Windows 硬件工具箱。

## 功能

- **工具启动器**：点击卡片启动第三方硬件工具（EXE）
- **SN 码管理**：读取/修改主板 SN 码（需管理员权限）
- **硬件信息**：展示 CPU、内存、硬盘、显卡等信息（可选扩展）

## 技术栈

- **前端**：Svelte + TypeScript
- **后端**：Rust
- **桌面框架**：Tauri 2.x
- **包管理器**：pnpm

## 开发

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建
pnpm tauri build

# 手动写的构建脚本可先构建前端再把前端嵌入exe
npm.cmd run build:manual-release
```

## 源码目录结构

```
iTools/
├── src/                        # 前端源码（Svelte + TypeScript）
│   ├── main.ts                 # 前端入口
│   ├── App.svelte              # 主界面组件
│   ├── components/
│   │   ├── ToolCard.svelte     # 工具卡片组件
│   │   └── SnEditor.svelte     # SN 码编辑组件
│   ├── styles/
│   │   └── global.css          # 全局样式
│   └── vite-env.d.ts           # TypeScript 类型声明
│
├── src-tauri/                  # 后端源码（Rust）
│   ├── src/
│   │   ├── main.rs             # Tauri 入口，注册命令
│   │   ├── logger.rs           # 日志模块
│   │   ├── hardware/
│   │   │   ├── mod.rs
│   │   │   └── sn.rs           # SN 码读取/写入
│   │   └── launcher/
│   │       └── mod.rs          # 工具启动器
│   ├── config.json             # 工具配置（源码）
│   ├── tauri.conf.json         # Tauri 配置
│   ├── Cargo.toml              # Rust 依赖
│   └── build.rs                # 构建脚本
│
├── index.html                  # HTML 入口
├── vite.config.ts              # Vite 配置
├── svelte.config.js            # Svelte 配置
├── tsconfig.json               # TypeScript 配置
└── package.json                # Node.js 依赖
```

## 模块说明

| 模块 | 文件 | 功能 |
|------|------|------|
| 主入口 | `main.rs` | Tauri 应用初始化，注册所有命令 |
| 日志 | `logger.rs` | 运行日志记录 |
| SN 管理 | `hardware/sn.rs` | 读取/写入主板 SN 码 |
| 工具启动 | `launcher/mod.rs` | 读取配置、启动第三方工具 |
| 主界面 | `App.svelte` | 选项卡切换（工具箱/SN管理） |
| 工具卡片 | `ToolCard.svelte` | 工具展示卡片，支持图标 |
| SN 编辑 | `SnEditor.svelte` | SN 码读取和编辑界面 |

## 运行时目录结构

构建完成后，exe 相对目录结构如下：

```
itools.exe
├── config/
│   └── config.json         # 工具配置文件
├── icons/
│   ├── cpuz.png            # 工具图标
│   ├── gpuz.png
│   └── ...
├── sidecars/
│   └── AMIDEWIN.EXE        # SN 写入工具
├── tools/
│   ├── CPUZ/
│   │   └── cpuz_x64.exe
│   ├── GPUZ/
│   │   └── GPU-Z.exe
│   └── ...
└── logs/
    └── itools.log          # 运行日志
```

## 配置说明

### config.json

```json
[
  {
    "id": "cpuz",
    "name": "CPU-Z",
    "description": "CPU 检测工具",
    "icon": "icons/cpuz.png",
    "path": "tools/CPUZ/cpuz_x64.exe"
  }
]
```

- `id`：工具唯一标识
- `name`：显示名称
- `description`：工具描述
- `icon`：图标路径（相对于 exe 目录，可选）
- `path`：工具 exe 路径（相对于 exe 目录）

## 构建产物

```
target/release/
├── itools.exe
├── config/
├── icons/
├── sidecars/
└── tools/
```
