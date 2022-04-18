
## Get Started

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```


## Bug

Q: vscode一直卡在fetching metadata阶段。

A: 运行`cargo metadata`，发现
```
Blocking waiting for file lock on package cache
```
换源
```
vim ~/.cargo/config
```
写入以下内容
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'tuna' # 从下面选一个镜像源名，如：tuna、sjtu、ustc，或者 rustcc

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```
运行
```
rm -rf ~/.cargo/.package-cache
```
删除 cargo 的缓存，而后再运行 `cargo metadata` 后发现没有 blocking 的提示，重启 vscdoe，加载 rust-analyzer 成功。
