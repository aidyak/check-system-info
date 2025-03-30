# CheckSystemInfo

(Japanese translation is below)

CheckSystemInfo is a simple system monitoring tool written in Rust.
It is designed to periodically display CPU usage and memory usage, making it easy to understand your system’s status at a glance.

## Features

- **Display CPU Usage**: Shows the CPU usage of each core.
- **Display Memory Usage**: Shows total memory, used memory, and free memory.
- **Customizable Update Interval**: You can specify the update interval (in seconds) via command-line arguments (default is 5 seconds).
- **Simple CLI Interface**: Utilizes the [clap](https://github.com/clap-rs/clap) crate for an intuitive command-line interface.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version is recommended)
- Cargo

### Build Instructions

Clone the repository and build a release version using the following commands:

```bash
git clone https://github.com/aidyak/check-system-info.git
cd check-system-info
cargo build --release
```

### Usage
After building, the executable will be located at ./target/release/check-system-info.
For example, to display both CPU and memory information at 5-second intervals, run:

```bash
./target/release/check-system-info --cpu --memory --interval 5
```

### Options
- --cpu
Display CPU usage.

- --memory
Display memory usage.

- --interval <seconds>

Set the update interval in seconds (default is 5 seconds).

### Output Example
Example execution:

```bash
$ ./target/release/check-system-info --cpu --memory --interval 5

CPU Usage:
  CPU 0: 12.34%
  CPU 1: 15.67%
  CPU 2: 10.23%
  CPU 3: 20.45%
Memory Usage:
  Total Memory: 8192 MB, Used: 4096 MB, Free: 4096 MB
--------------------
```
The above information is updated every 5 seconds.

---

CheckSystemInfoはRust製のシンプルなシステムモニタリングツールです。
このツールは、定期的にCPU使用率やメモリ使用量を表示し、システムの状態を簡単に把握できるように設計されています。

## 特徴

- **CPU使用率の表示**: 各コアのCPU使用率を表示します。
- **メモリ使用量の表示**: 総メモリ、使用中メモリ、空きメモリを表示します。
- **更新間隔の設定**: コマンドライン引数で更新間隔（秒）を指定可能（デフォルトは5秒）。
- **シンプルなCLIインターフェース**: [clap](https://github.com/clap-rs/clap) クレートを利用した直感的な操作。

## インストール

### 前提条件

- [Rust](https://www.rust-lang.org/)（最新の安定版推奨）
- Cargo

### ビルド方法

以下のコマンドでリポジトリをクローンし、リリースビルドを行ってください。

```bash
git clone https://github.com/aidyak/check-system-info.git
cd check-system-info
cargo build --release
```

### 使い方
ビルドが完了すると、`./target/release/check-system-info` が実行ファイルとなります。
例えば、CPUとメモリの情報を5秒間隔で表示する場合は、以下のように実行します。

```bash
./target/release/check-system-info --cpu --memory --interval 5
```

### オプション
- --cpu
CPU使用率を表示します。

- --memory
メモリ使用量を表示します。

- --interval <秒数>
更新間隔を秒単位で指定します（デフォルトは5秒）。

### 出力例
実行例:

```bash
$ ./target/release/simplesysmonitor --cpu --memory --interval 5

CPU 使用率:
  CPU 0: 12.34%
  CPU 1: 15.67%
  CPU 2: 10.23%
  CPU 3: 20.45%
メモリ 使用量:
  総メモリ: 8192 MB, 使用中: 4096 MB, 空き: 4096 MB
--------------------
```
5秒ごとに上記の情報が更新されます。


