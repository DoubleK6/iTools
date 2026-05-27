# iTools 开发文档

## 1. 项目概述

**iTools** 是一款基于 **Tauri  + Rust** 构建的 Windows 硬件工具箱聚合软件。
核心功能：

- 作为“启动器”，点击说明框启动第三方硬件工具（EXE）。
- 部分功能由 iTools 自身实现：**读取/修改主板 SN 码**。
- 可选扩展：展示 CPU、内存、硬盘、显卡等硬件信息。

技术选型：

- **前端**：Svelte + TypeScript
- **后端**：Rust 1.70+
- **桌面框架**：Tauri 2.x
- **包管理器**：pnpm
- **目标平台**：Windows 10 / Windows 11（也可兼容 Windows 7 需额外注意 WebView2 依赖）

## 开发顺序

1. 搭建项目骨架（Svelte + Tauri）
2. 实现工具启动器（最简单）
3. 实现 SN 读取
4. 实现 SN 写入（需要 AMIDEWIN.exe）
5. 可选：硬件信息展示

------

## 2. 项目环境准备

### 2.1 安装依赖

- 当前环境已安装

### 2.2 创建 Tauri 项目

bash

```
npm create tauri-app@latest
# 输入项目名: iTools
# 选择前端框架 (如 React + TypeScript)
# 选择包管理器 (npm/pnpm/yarn)
cd iTools
npm install
```



### 2.3 添加 Tauri 插件

bash

```
npm run tauri add shell          # 启动外部程序
npm run tauri add dialog         # 打开文件/保存对话框（可选）
npm run tauri add fs             # 读写配置文件（可选）
npm run tauri add process        # 获取当前进程信息（用于提权检测）
```



------

## 3. 项目结构建议

text

```
iTools/
├── src-tauri/               # Rust 后端
│   ├── src/
│   │   ├── main.rs          # 入口，注册 commands
│   │   ├── hardware/        # 硬件信息模块
│   │   │   ├── mod.rs
│   │   │   ├── sn.rs        # SN 读取与写入
│   │   │   ├── sysinfo.rs   # CPU/内存/磁盘等
│   │   │   └── wmi.rs       # WMI 查询封装
│   │   ├── launcher/        # 第三方程序启动器
│   │   │   └── mod.rs
│   │   └── utils/           # 辅助函数（权限检查、路径解析等）
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── sidecars/            # 存放 sidecar 程序（如 AMIDEWIN.exe）
├── src/                     # 前端代码
│   ├── App.tsx
│   ├── components/
│   │   ├── ToolCard.tsx     # 工具卡片组件
│   │   ├── SnEditor.tsx     # SN 编辑组件
│   │   └── HardwarePanel.tsx
│   ├── hooks/useTauriCommand.ts
│   └── styles/
├── tools/                   # 存放需要集成的第三方工具（按需）
│   ├── cpuz/
│   ├── gpuz/
│   └── ...
└── package.json
```



------

## 4. 后端核心模块实现

### 4.1 读取 SN 码（Rust）

等同调用powershell命令(Get-CimInstance Win32_BIOS).SerialNumber获取到的内容

### 4.2 修改 SN 码

需要管理员权限 + 调用 `AMIDEWIN.exe` 写入。
我们将 AMIDEWIN.exe 放入 `src-tauri/sidecars/` 目录，并在 `tauri.conf.json` 中声明为 sidecar。

## 5. 前端界面设计

### 5.1 工具卡片组件

### 5.2 SN 编辑器

## 6. 第三方程序启动机制

### 6.1 后端命令

### 6.2 配置第三方工具清单

工具列表从 `config.json` 加载（相对于 exe 目录）：

```json
[
  {
    "id": "cpuz",
    "name": "CPU-Z",
    "description": "CPU 检测工具",
    "icon": "icons/cpuz.png",
    "path": "tools/cpuz/cpuz.exe"
  }
]
```

## 7. Tauri 配置

### 7.1 `tauri.conf.json` 关键配置

### 7.2 Sidecar 配置

将 `AMIDEWIN.EXE` 放入 `sidecars/AMIDEWIN.EXE`（相对于 exe 目录）。

### 7.3 运行时目录结构

```
target/release/
├── itools.exe              # 主程序
├── config/                 # 配置目录
│   └── config.json         # 工具配置
├── sidecars/               # SN 修改工具目录
│   └── AMIDEWIN.EXE        # SN 写入工具
└── tools/                  # 第三方工具目录
    ├── cpuz/
    │   └── cpuz.exe
    ├── gpuz/
    │   └── gpuz.exe
    └── ...
```