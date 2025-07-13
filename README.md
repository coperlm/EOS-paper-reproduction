# EOS 委托协议

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)

**EOS (高效外包方案)** 是SNARK委托协议的高性能实现，支持安全高效的零知识证明计算外包，同时保护隐私并确保可验证性。

## 🚀 特性

### 🔐 **密码学基础**
- **多方计算 (MPC)**: 分布式参与方的安全计算
- **秘密共享方案**: Shamir门限秘密共享和加法秘密共享
- **零知识证明**: 与SNARK证明系统集成
- **多项式承诺**: KZG承诺方案支持

### ⚡ **高性能**
- **优化的电路执行**: 高效的MPC电路评估
- **批处理操作**: 批量秘密共享和多项式操作
- **并行处理**: 多线程计算支持
- **内存高效**: 大规模计算的优化内存使用

### 🛡️ **安全性与隐私**
- **隐私保护**: 通过秘密共享保护输入隐私
- **可验证计算**: 外包计算的密码学验证
- **恶意安全**: 防护对抗性参与方
- **可配置安全参数**: 可调节的安全级别 (64/128/256位)

## 📊 性能基准测试

| 测试类别 | 执行时间 | 内存峰值 | 吞吐量 | 电路规模 |
|---------|---------|---------|-------|----------|
| 基础操作 | 1.4ms | 4MB | - | 150个约束 |
| 高级秘密共享 | 663ms | 1.2MB | 3,017 约束/秒 | 2,000个约束 |
| 复杂MPC电路 | 5.8ms | 11MB | 868,071 约束/秒 | 5,000个约束 |
| 大规模基准 | 866ms | 17MB | 57,710 约束/秒 | 50,000个约束 |

## 🏗️ 架构

```
eos-delegation/
├── src/
│   ├── circuit/           # 电路操作 (FFT, MSM, 多项式操作)
│   │   ├── common.rs      # 通用电路操作
│   │   └── pc_schemes.rs  # 多项式承诺方案
│   ├── mpc/               # 多方计算
│   │   ├── secret_sharing.rs  # Shamir & 加法秘密共享
│   │   ├── executor.rs        # MPC电路执行器
│   │   └── modes.rs           # 隔离与协作模式
│   ├── piop/              # 多项式交互式Oracle证明
│   │   └── consistency_checker.rs
│   ├── protocol/          # 核心委托协议
│   │   └── delegation_protocol.rs
│   └── evaluation/        # 性能评估工具
└── examples/              # 使用示例和演示
```

## 🚀 快速开始

### 前置要求

- **Rust** 1.70+ (带Cargo)
- **Git**

### 安装

```bash
git clone https://github.com/your-username/eos-delegation.git
cd eos-delegation
cargo build --release
```

### 基本使用

```rust
use eos_delegation::*;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(42);
    
    // 初始化秘密共享
    let secret = Fr::from(123u64);
    let shares = ShamirSecretSharing::<Fr>::share_secret(secret, 3, 5, &mut rng);
    
    // 重构秘密
    let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..3])?;
    assert_eq!(secret, reconstructed);
    
    println!("✅ 秘密共享成功!");
    Ok(())
}
```

### 运行示例

```bash
# 基础功能演示
cargo run

# 综合演示
cargo run --example complete_demo

# 运行测试
cargo test
```

## 📋 使用示例

### 1. 秘密共享

```rust
use eos_delegation::mpc::*;
use ark_bls12_381::Fr;

// Shamir秘密共享
let secret = Fr::from(42u64);
let shares = ShamirSecretSharing::<Fr>::share_secret(secret, 3, 5, &mut rng);
let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..3])?;

// 加法秘密共享
let additive_shares = AdditiveSecretSharing::<Fr>::share_secret(secret, 0, 5, &mut rng);
let reconstructed = AdditiveSecretSharing::<Fr>::reconstruct_secret(&additive_shares)?;
```

### 2. MPC电路执行

```rust
use eos_delegation::mpc::*;

let secret_sharing = ShamirSecretSharing::<Fr>::new();
let mut executor = ExecCircuit::new(1, 3, secret_sharing);

// 输入秘密
let shares1 = executor.input_secret(Fr::from(10u64), 2, &mut rng);
let shares2 = executor.input_secret(Fr::from(20u64), 2, &mut rng);

// 执行操作
let add_result = executor.add_gate(&shares1[0], &shares2[0])?;
let mul_result = executor.mul_gate(&shares1[0], &shares2[0])?;
```

### 3. 操作模式

```rust
use eos_delegation::mpc::modes::*;

// 隔离模式 - 最小通信
let isolation_mode = IsolationMode::new(1, 3);
let pattern = isolation_mode.get_communication_pattern();

// 协作模式 - 完全通信
let collaboration_mode = CollaborationMode::new(2, true, true);
let pattern = collaboration_mode.get_communication_pattern();
```

### 4. 性能评估

```rust
use eos_delegation::evaluation::*;

let mut metrics = PerformanceMetrics::new();

let timer = metrics.start_timer("computation");
// ... 执行计算 ...
let (phase, duration) = timer.stop();
metrics.record_timing(phase, duration);

let report = metrics.generate_report();
// 打印详细的性能指标
```

## 🧪 测试

### 运行所有测试
```bash
cargo test
```

### 运行特定测试类别
```bash
# 基础功能测试
cargo test test_secret_sharing

# MPC操作测试  
cargo test test_mpc

# 性能基准测试
cargo test --release test_performance
```

### 运行综合基准测试
```bash
cargo run --release
```

## 📈 性能优化

该实现包含多种优化策略:

1. **批处理**: 批量操作以减少通信开销
2. **并行执行**: 尽可能使用多线程处理  
3. **内存管理**: 高效的内存分配和重用
4. **算法选择**: 基于输入大小选择最优算法
5. **通信优化**: 最小化轮次复杂度

## 🔧 配置

### 安全参数

```rust
// 配置安全级别
let params = ProtocolParams::new(128); // 128位安全性

// 调整门限参数
let threshold = 3;  // 需要的最少份额数
let num_parties = 5; // 参与方总数
```

### 操作模式

```rust
// 隔离模式: 最小化通信
let isolation = IsolationMode::new(
    1,  // isolation_level 隔离级别
    3   // max_communication_rounds 最大通信轮次
);

// 协作模式: 最大化效率
let collaboration = CollaborationMode::new(
    3,    // collaboration_level 协作级别
    true, // use_optimized_protocols 使用优化协议
    true  // enable_parallel_processing 启用并行处理
);
```

## 🤝 贡献

我们欢迎贡献！详情请参见我们的[贡献指南](CONTRIBUTING.md)。

### 开发环境设置

```bash
git clone https://github.com/your-username/eos-delegation.git
cd eos-delegation

# 安装开发依赖
cargo build

# 运行代码检查
cargo clippy

# 格式化代码
cargo fmt

# 运行所有测试
cargo test
```

### 提交更改

1. Fork仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开Pull Request

## 📚 文档

- **API文档**: 运行 `cargo doc --open` 生成并查看文档
- **示例**: 查看 `examples/` 目录获取使用示例
- **架构指南**: `docs/` 中的详细架构文档

## 🔬 研究与论文

此实现基于以下研究:

- **EOS**: 高效的zkSNARK证明者私有委托
- **Marlin**: 具有通用和可更新SRS的预处理zkSNARKs  
- **KZG多项式承诺**
- **Shamir秘密共享方案**

## 📄 许可证

此项目基于MIT许可证 - 详情请参见[LICENSE](LICENSE)文件。

## 🙏 致谢

- [arkworks](https://github.com/arkworks-rs) - 密码学库生态系统
- 致力于zkSNARKs和MPC协议研究的学术界
- 贡献者和维护者

## 📞 支持与联系

- **问题反馈**: [GitHub Issues](https://github.com/your-username/eos-delegation/issues)
- **讨论**: [GitHub Discussions](https://github.com/your-username/eos-delegation/discussions)
- **邮箱**: your-email@domain.com

---

**⭐ 如果您觉得这个项目有用，请考虑给它一个星标！**
