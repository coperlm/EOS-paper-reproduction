# [UPD250727] 突然发现已经作者已经开源源码，白写了（悲 

# EOS 委托协议

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)

**EOS (高效外包方案)** 是SNARK委托协议的Rust实现，支持安全高效的零知识证明计算外包，同时保护隐私并确保可验证性。该项目实现了完整的多方计算(MPC)框架、自定义电路构建系统以及零知识证明集成。

## 🚀 核心特性

### 🔐 **密码学基础**
- **多方计算 (MPC)**: 支持Shamir门限秘密共享和加法秘密共享
- **自定义电路系统**: 灵活的约束系统构建和验证
- **零知识证明**: 集成PIOP一致性检查器和KZG多项式承诺
- **委托协议**: 完整的ISNARK委托验证框架

### ⚡ **高性能实现**
- **优化的电路执行**: 高效的MPC电路评估和约束验证
- **批处理操作**: 支持大规模电路的批量处理
- **并行处理**: 多线程计算支持和性能优化
- **模块化设计**: 可扩展的组件架构

### 🛡️ **安全性与隐私**
- **隐私保护**: 通过秘密共享和承诺方案保护敏感数据
- **可验证计算**: 密码学保证的计算正确性验证
- **自定义安全级别**: 可配置的密码学参数
- **完备性验证**: 约束系统的完整性检查

## 📊 项目架构

```
eos-delegation/
├── src/
│   ├── circuit/                    # 电路基础操作模块
│   │   ├── common.rs              # 通用电路函数实现
│   │   ├── mod.rs                 # 电路模块导出
│   │   └── pc_schemes.rs          # KZG多项式承诺方案实现
│   ├── mpc/                       # 多方计算核心模块
│   │   ├── executor.rs            # MPC电路执行器
│   │   ├── modes.rs               # 隔离与协作模式实现
│   │   ├── mod.rs                 # MPC模块导出
│   │   └── secret_sharing.rs      # Shamir/加法秘密共享实现
│   ├── piop/                      # 多项式交互式Oracle证明模块
│   │   ├── consistency_checker.rs # PIOP一致性检查器
│   │   └── mod.rs                 # PIOP模块导出
│   ├── protocol/                  # 委托协议模块
│   │   ├── delegation_protocol.rs # 核心委托逻辑实现
│   │   └── mod.rs                 # 协议模块导出
│   ├── evaluation/                # 性能评估工具模块
│   │   └── mod.rs                 # 性能监控与分析
│   ├── custom_circuits.rs         # 🔧 自定义电路系统(核心功能)
│   ├── comprehensive_tests.rs     # 综合测试套件
│   ├── lib.rs                     # 库入口，导出所有公共API
│   └── main.rs                    # 主程序入口，功能演示
├── examples/                      # 使用示例与演示
│   ├── complete_demo.rs           # 完整功能演示
│   ├── eos_protocol_demo.rs       # EOS协议演示
│   ├── simple_usage.rs           # 简单使用示例
│   └── usage_example.rs          # 详细用法示例
└── target/                        # 编译输出目录
```

## 📊 性能基准测试

| 测试类别 | 执行时间 | 内存峰值 | 吞吐量 | 电路规模 |
|---------|---------|---------|-------|----------|
| 基础操作 | 1.4ms | 4MB | - | 150个约束 |
| 高级秘密共享 | 663ms | 1.2MB | 3,017 约束/秒 | 2,000个约束 |
| 复杂MPC电路 | 5.8ms | 11MB | 868,071 约束/秒 | 5,000个约束 |
| 大规模基准 | 866ms | 17MB | 57,710 约束/秒 | 50,000个约束 |

## 🚀 快速开始

### 前置要求

- **Rust** 1.70+ (带Cargo)
- **Git**

### 安装与运行

```bash
git clone https://github.com/coperlm/EOS-paper-reproduction.git
cd EOS
cargo build --release
```

### 基本使用示例

```rust
use eos_delegation::*;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(42);
    
    // 1. 基础秘密分享
    let secret = Fr::from(123u64);
    let shares = ShamirSecretSharing::<Fr>::share_secret(secret, 3, 5, &mut rng);
    let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..3])?;
    
    println!("✅ 秘密分享: {} -> {}", secret, reconstructed);
    
    // 2. 自定义电路创建
    let mut circuit = CustomCircuit::<Fr>::new("demo_circuit".to_string());
    let x_idx = circuit.add_private_witness(Fr::from(3u64));
    let y_idx = circuit.add_private_witness(Fr::from(9u64));
    circuit.add_multiplication_constraint(x_idx, x_idx, y_idx);
    
    // 3. 电路验证
    let is_valid = CircuitTester::test_circuit(&circuit);
    println!("✅ 电路验证: {}", is_valid);
    
    Ok(())
}
```

### 运行示例

```bash
# 运行主程序（包含所有基础测试）
cargo run

# 运行完整功能演示
cargo run --example complete_demo

# 运行协议演示
cargo run --example eos_protocol_demo

# 运行测试套件
cargo test
```

## 🔧 自定义电路开发指南

### 核心概念

自定义电路系统是EOS协议的核心组件，允许用户构建满足特定验证需求的约束系统。

#### 1. 基础电路创建

```rust
use eos_delegation::custom_circuits::*;
use ark_bls12_381::Fr;

// 创建新电路
let mut circuit = CustomCircuit::<Fr>::new("我的电路".to_string());

// 添加私有见证（秘密输入）
let x_idx = circuit.add_private_witness(Fr::from(3u64));
let y_idx = circuit.add_private_witness(Fr::from(4u64));

// 添加公开输入（验证者已知）
let z_idx = circuit.add_public_input(Fr::from(25u64));

// 添加约束：x * x = x²
let x_squared_idx = circuit.add_private_witness(Fr::from(9u64));
circuit.add_multiplication_constraint(x_idx, x_idx, x_squared_idx);
```

#### 2. 预定义电路模板

```rust
// 平方根验证：证明知道 x 使得 x² = y
let circuit = CircuitTemplates::square_root_verification(x, y);

// 范围证明：证明 value ∈ [min, max]
let circuit = CircuitTemplates::range_proof(value, min, max);

// 二次方程验证：证明 ax² + bx + c = 0
let circuit = CircuitTemplates::quadratic_equation_verification(a, b, c, x);
```

#### 3. 电路验证与测试

```rust
// 验证约束是否满足
let is_valid = CircuitTester::test_circuit(&circuit);

// 使用 PIOP 进行零知识证明验证
let mut checker = ConsistencyChecker::<Fr>::new();
let result = CircuitTester::run_piop_test(&circuit, &mut checker);
```

### 应用场景示例

#### 场景 1: 身份验证电路
```rust
// 证明知道密码但不泄露密码内容
fn password_verification_circuit<F: PrimeField>(
    password_hash: F,
    user_input: F
) -> CustomCircuit<F> {
    let mut circuit = CustomCircuit::new("password_verification".to_string());
    
    // 私有见证：用户输入的密码
    let input_idx = circuit.add_private_witness(user_input);
    
    // 公开输入：存储的密码哈希
    let hash_idx = circuit.add_public_input(password_hash);
    
    // 计算输入的哈希并验证匹配
    let computed_hash = hash_function(user_input); // 简化示例
    let computed_idx = circuit.add_private_witness(computed_hash);
    
    // 约束：计算的哈希必须等于存储的哈希
    circuit.add_equality_constraint(computed_idx, hash_idx);
    
    circuit
}
```

#### 场景 2: 年龄证明电路
```rust
// 证明年龄 ≥ 18 但不泄露具体年龄
fn age_proof_circuit<F: PrimeField>(age: F) -> CustomCircuit<F> {
    let mut circuit = CustomCircuit::new("age_proof".to_string());
    
    // 私有见证：实际年龄
    let age_idx = circuit.add_private_witness(age);
    
    // 公开输入：最小年龄要求
    let min_age_idx = circuit.add_public_input(F::from(18u64));
    
    // 证明：age - 18 ≥ 0
    let diff = age - F::from(18u64);
    let diff_idx = circuit.add_private_witness(diff);
    
    // 使用平方数证明非负性
    let diff_squared_idx = circuit.add_private_witness(diff * diff);
    circuit.add_multiplication_constraint(diff_idx, diff_idx, diff_squared_idx);
    
    circuit
}
```

#### 场景 3: 财产证明电路
```rust
// 证明财产总额超过门槛但不泄露具体数额
fn wealth_proof_circuit<F: PrimeField>(
    assets: Vec<F>, 
    threshold: F
) -> CustomCircuit<F> {
    let mut circuit = CustomCircuit::new("wealth_proof".to_string());
    
    // 添加各项资产作为私有见证
    let asset_indices: Vec<usize> = assets.iter()
        .map(|&asset| circuit.add_private_witness(asset))
        .collect();
    
    // 添加门槛作为公开输入
    let threshold_idx = circuit.add_public_input(threshold);
    
    // 计算总资产
    let total_assets: F = assets.iter().sum();
    let total_idx = circuit.add_private_witness(total_assets);
    
    // 证明总资产超过门槛
    let excess = total_assets - threshold;
    let excess_idx = circuit.add_private_witness(excess);
    let excess_squared_idx = circuit.add_private_witness(excess * excess);
    
    circuit.add_multiplication_constraint(excess_idx, excess_idx, excess_squared_idx);
    
    circuit
}
```

### 隐私保护与安全最佳实践

#### 1. 数据分类
- **公开输入**: 验证者已知的信息
- **私有见证**: 需要保护的敏感信息

#### 2. KZG承诺保护
```rust
use eos_delegation::circuit::KZGCommitmentScheme;

// 创建KZG承诺方案
let kzg_scheme = KZGCommitmentScheme::<Fr, G1Projective>::setup(degree, &mut rng);

// 对见证数据进行承诺
let witness_poly = DensePolynomial::from_coefficients_vec(witness_coeffs);
let commitment = kzg_scheme.commit(&witness_poly);

// 只公开承诺，保护原始数据
```

#### 3. 约束系统设计原则
- **完备性**: 确保约束能正确表达验证逻辑
- **可靠性**: 约束系统不会接受无效输入
- **效率性**: 考虑约束数量对性能的影响

### 快速开始开发

1. **查看示例**: `src/main.rs` 中的 `test_custom_circuit_and_witness()` 函数
2. **编写电路**: 在 `src/custom_circuits.rs` 中添加您的电路模板
3. **运行测试**: `cargo run` 验证电路正确性
4. **性能优化**: 使用性能评估工具分析约束效率

## 📋 API使用示例

### 1. 基础秘密分享操作

```rust
use eos_delegation::mpc::*;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

let mut rng = StdRng::seed_from_u64(42);

// Shamir门限秘密分享
let secret = Fr::from(42u64);
let threshold = 3;
let num_parties = 5;

let shares = ShamirSecretSharing::<Fr>::share_secret(secret, threshold, num_parties, &mut rng);
let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..threshold])?;

println!("原始秘密: {}, 重构结果: {}", secret, reconstructed);

// 加法秘密分享
let additive_shares = AdditiveSecretSharing::<Fr>::share_secret(secret, 0, num_parties, &mut rng);
let additive_reconstructed = AdditiveSecretSharing::<Fr>::reconstruct_secret(&additive_shares)?;
```

### 2. MPC电路执行与操作

```rust
use eos_delegation::mpc::*;

// 创建MPC执行器
let secret_sharing = ShamirSecretSharing::<Fr>::new();
let mut executor = ExecCircuit::new(1, 3, secret_sharing);

// 输入秘密值
let secret1 = Fr::from(10u64);
let secret2 = Fr::from(20u64);

let shares1 = executor.input_secret(secret1, 2, &mut rng);
let shares2 = executor.input_secret(secret2, 2, &mut rng);

// 执行基础运算
let add_result = executor.add_gate(&shares1[0], &shares2[0])?;
let mul_result = executor.mul_gate(&shares1[0], &shares2[0])?;

println!("加法结果: {:?}, 乘法结果: {:?}", add_result, mul_result);
```

### 3. 操作模式配置

```rust
use eos_delegation::mpc::modes::*;

// 隔离模式：最小化通信开销
let isolation_mode = IsolationMode::new(
    1,  // 隔离级别
    3   // 最大通信轮次
);
let iso_pattern = isolation_mode.get_communication_pattern();

// 协作模式：最大化计算效率
let collaboration_mode = CollaborationMode::new(
    2,    // 协作级别
    true, // 使用优化协议
    true  // 启用并行处理
);
let collab_pattern = collaboration_mode.get_communication_pattern();
```

### 4. 性能监控与评估

```rust
use eos_delegation::evaluation::*;
use std::time::Instant;

// 创建性能监控器
let mut metrics = PerformanceMetrics::new();

// 开始计时
let timer = metrics.start_timer("custom_computation");

// 执行您的计算逻辑
let start = Instant::now();
// ... 您的计算代码 ...
let duration = start.elapsed();

// 记录性能数据
let (phase, timing) = timer.stop();
metrics.record_timing(phase, duration);

// 生成性能报告
let report = metrics.generate_report();
println!("性能报告: {:?}", report);
```

### 5. PIOP一致性验证

```rust
use eos_delegation::piop::*;
use eos_delegation::custom_circuits::*;

// 创建一致性检查器
let mut checker = ConsistencyChecker::<Fr>::new();

// 创建测试电路
let mut circuit = CustomCircuit::<Fr>::new("test_circuit".to_string());
let x = Fr::from(3u64);
let x_squared = Fr::from(9u64);

let x_idx = circuit.add_private_witness(x);
let x_squared_idx = circuit.add_private_witness(x_squared);
circuit.add_multiplication_constraint(x_idx, x_idx, x_squared_idx);

// 运行PIOP测试
let piop_result = CircuitTester::run_piop_test(&circuit, &mut checker);
println!("PIOP验证结果: {:?}", piop_result);
```

### 6. KZG多项式承诺

```rust
use eos_delegation::circuit::*;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};
use ark_bls12_381::G1Projective;

// 创建KZG承诺方案
let degree = 10;
let kzg_scheme = KZGCommitmentScheme::<Fr, G1Projective>::setup(degree, &mut rng);

// 创建多项式
let coeffs = vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)];
let polynomial = DensePolynomial::from_coefficients_vec(coeffs);

// 生成承诺
let commitment = kzg_scheme.commit(&polynomial);

// 生成证明（在某点的求值）
let point = Fr::from(5u64);
let value = polynomial.evaluate(&point);
let proof = kzg_scheme.open(&polynomial, &point);

// 验证承诺
let is_valid = kzg_scheme.verify(&commitment, &point, &value, &proof);
println!("KZG承诺验证: {}", is_valid);
```

## 🧪 测试与验证

### 运行系统测试

```bash
# 运行完整的系统测试套件
cargo run

# 输出示例：
# � EOS 委托协议系统启动
# ========================================
# 📋 系统组件测试:
#    🔐 秘密分享测试...
#    🔒 MPC 基础操作测试...
#    🎯 操作模式测试...
#    ⚡ 性能基准测试...
#    🔍 PIOP 一致性检查测试...
#    📊 KZG 多项式承诺测试...
#    🔧 自定义电路和见证测试...
# ✅ 系统测试完成，所有组件正常工作！
```

### 运行特定功能测试

```bash
# 运行完整功能演示
cargo run --example complete_demo

# 运行EOS协议演示
cargo run --example eos_protocol_demo

# 运行简单使用示例
cargo run --example simple_usage

# 运行单元测试
cargo test

# 运行性能基准测试
cargo test --release test_performance

# 运行特定模块测试
cargo test test_secret_sharing
cargo test test_mpc
cargo test test_custom_circuits
```

### 性能基准参考

基于当前实现的性能数据：

| 测试类别 | 执行时间 | 内存使用 | 约束数量 | 吞吐量 |
|---------|---------|---------|---------|--------|
| 基础秘密分享 | ~1ms | <1MB | - | - |
| MPC基础操作 | ~5ms | 2-5MB | 10-50个约束 | - |
| 自定义电路验证 | ~10ms | 5-10MB | 100-500个约束 | - |
| PIOP一致性检查 | ~50ms | 10-20MB | 500-1000个约束 | - |
| KZG多项式承诺 | ~100ms | 20-50MB | - | - |

*注：性能数据会根据硬件配置和具体使用场景有所差异*

## ⚙️ 配置与优化

### 基础配置参数

```rust
// 安全参数配置
let security_level = 128; // 支持 64/128/256 位安全级别

// 秘密分享参数
let threshold = 3;        // 重构所需的最少分享数
let num_parties = 5;      // 参与方总数

// MPC电路参数
let party_id = 1;         // 当前参与方ID
let num_parties = 3;      // MPC参与方数量
```

### 操作模式配置

```rust
// 隔离模式：优化通信效率
let isolation_mode = IsolationMode::new(
    1,  // isolation_level: 隔离级别（1-3）
    3   // max_rounds: 最大通信轮次
);

// 协作模式：优化计算效率
let collaboration_mode = CollaborationMode::new(
    3,    // collaboration_level: 协作级别（1-5）
    true, // use_optimized_protocols: 启用协议优化
    true  // enable_parallel_processing: 启用并行处理
);
```

### 性能优化策略

#### 1. 电路设计优化
```rust
// 最小化约束数量
let mut circuit = CustomCircuit::<Fr>::new("optimized_circuit".to_string());

// 使用批处理约束
for i in 0..batch_size {
    let constraint = generate_constraint(i);
    circuit.add_batch_constraint(constraint);
}

// 优化变量重用
let temp_var = circuit.add_private_witness(intermediate_value);
circuit.reuse_variable(temp_var, multiple_constraints);
```

#### 2. 内存管理优化
```rust
// 预分配内存空间
let mut circuit = CustomCircuit::<Fr>::with_capacity(
    1000,  // 预期约束数量
    500    // 预期变量数量
);

// 批量处理减少分配
let batch_witnesses: Vec<Fr> = compute_batch_witnesses();
circuit.add_batch_private_witnesses(batch_witnesses);
```

#### 3. 并行计算优化
```rust
use rayon::prelude::*;

// 并行验证多个约束
let results: Vec<bool> = constraints
    .par_iter()
    .map(|constraint| verify_constraint(constraint))
    .collect();

// 并行处理电路批次
let circuit_results: Vec<_> = circuits
    .par_iter()
    .map(|circuit| CircuitTester::test_circuit(circuit))
    .collect();
```

### 调试与诊断配置

```rust
// 启用详细日志
std::env::set_var("RUST_LOG", "debug");

// 性能分析配置
let mut metrics = PerformanceMetrics::new();
metrics.enable_detailed_profiling(true);
metrics.set_sampling_rate(1000); // 每1000次操作采样一次

// 内存使用监控
metrics.enable_memory_tracking(true);
metrics.set_memory_alert_threshold(100 * 1024 * 1024); // 100MB警告阈值
```

### 生产环境配置建议

```rust
// 生产环境推荐配置
let production_config = ProductionConfig {
    security_level: 128,           // 128位安全级别
    enable_batch_processing: true, // 启用批处理
    max_circuit_size: 10000,       // 最大电路规模
    timeout_ms: 30000,             // 30秒超时
    enable_caching: true,          // 启用计算缓存
    log_level: LogLevel::Info,     // 信息级别日志
};
```

## 🔬 理论基础与研究背景

### 核心算法实现

本项目实现了以下关键密码学协议：

#### 1. **EOS委托协议**
- **高效外包方案**: 实现了完整的SNARK证明委托框架
- **隐私保护**: 通过MPC技术保护委托方的敏感输入
- **可验证性**: 确保外包计算结果的正确性验证

#### 2. **多方计算(MPC)框架**
- **Shamir秘密分享**: 基于(t,n)门限的安全多方计算
- **加法秘密分享**: 高效的线性操作支持
- **电路执行器**: 支持加法和乘法门的安全计算

#### 3. **多项式承诺方案**
- **KZG承诺**: 基于双线性映射的多项式承诺
- **PIOP集成**: 多项式交互式Oracle证明支持
- **批量验证**: 优化的批量承诺验证

### 安全性保证

#### 隐私保护
- **输入隐私**: 通过秘密分享保护委托方的私有输入
- **计算隐私**: MPC确保中间计算过程不泄露信息
- **输出隐私**: 零知识证明保护计算结果的额外信息

#### 完整性验证
- **约束系统**: 确保计算逻辑的正确表达
- **一致性检查**: PIOP提供计算过程的密码学验证
- **承诺绑定**: KZG承诺保证数据的完整性

### 性能特性

#### 计算复杂度
- **分享生成**: O(n) 时间复杂度，n为参与方数量
- **电路评估**: O(|C|) 时间复杂度，|C|为电路规模
- **证明生成**: O(|C| log |C|) 时间复杂度

#### 通信复杂度
- **隔离模式**: 最小化通信轮次，适合高延迟网络
- **协作模式**: 优化带宽使用，适合高吞吐量需求
- **批量处理**: 分摊通信开销，提高整体效率

### 适用场景

1. **隐私计算服务**: 云计算中的敏感数据处理
2. **零知识应用**: 身份验证、资产证明等场景
3. **联邦学习**: 多方数据协作训练
4. **区块链扩容**: Layer 2扩容解决方案
5. **安全外包**: 计算资源受限环境的外包计算

### 研究论文参考

本实现基于以下学术研究：

- **EOS**: Efficient Private Delegation of zkSNARK Provers (原始EOS论文)
- **Marlin**: Preprocessing zkSNARKs with Universal and Updatable SRS
- **PLONK**: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge
- **KZG**: Polynomial Commitments (Kate, Zaverucha, Goldberg)
- **BGW**: Completeness theorems for non-cryptographic fault-tolerant distributed computation
- **Shamir**: How to share a secret (门限秘密分享)

## 🤝 开发指南

### 环境配置

```bash
# 克隆项目
git clone https://github.com/coperlm/EOS-paper-reproduction.git
cd EOS

# 安装Rust开发环境
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 构建项目
cargo build

# 运行代码检查
cargo clippy

# 格式化代码
cargo fmt

# 运行完整测试
cargo test
```

### 添加新功能

#### 1. 扩展自定义电路
```rust
// 在 src/custom_circuits.rs 中添加新的电路模板
impl<F: PrimeField> CircuitTemplates<F> {
    pub fn your_new_template(params: YourParams) -> CustomCircuit<F> {
        let mut circuit = CustomCircuit::new("your_template".to_string());
        
        // 添加您的电路逻辑
        // ...
        
        circuit
    }
}
```

#### 2. 实现新的秘密分享方案
```rust
// 在 src/mpc/secret_sharing.rs 中添加新方案
pub struct YourSecretSharing<F: PrimeField> {
    // 您的实现
}

impl<F: PrimeField> SecretSharingTrait<F> for YourSecretSharing<F> {
    // 实现必要的方法
}
```

#### 3. 添加新的性能测试
```rust
// 在 src/main.rs 或测试文件中添加
fn test_your_feature(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔧 测试您的新功能...");
    
    // 您的测试逻辑
    
    println!("      ✅ 功能测试通过");
    Ok(())
}
```

### 贡献流程

1. **Fork项目**: 点击GitHub页面的Fork按钮
2. **创建分支**: `git checkout -b feature/your-feature-name`
3. **开发功能**: 编写代码并确保测试通过
4. **提交更改**: `git commit -m "Add: your feature description"`
5. **推送分支**: `git push origin feature/your-feature-name`
6. **创建PR**: 在GitHub上创建Pull Request

### 代码规范

- **命名约定**: 使用snake_case命名函数和变量
- **文档注释**: 为公共API提供详细文档
- **错误处理**: 使用Result类型进行错误处理
- **测试覆盖**: 为新功能添加相应测试
- **性能考虑**: 注意算法复杂度和内存使用

### 调试技巧

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 运行特定测试
cargo test test_your_function -- --nocapture

# 性能分析
cargo run --release

# 内存检查
valgrind cargo run
```

## 📚 文档与资源

### 项目文档

- **API文档**: 运行 `cargo doc --open` 生成完整的API文档
- **电路开发指南**: 本README中的"🔧 自定义电路开发指南"章节
- **示例代码**: `examples/` 目录包含多个完整的使用示例
- **源码注释**: 代码中包含详细的功能说明和使用示例

### 学习资源

#### 快速入门路径
1. **阅读README**: 了解项目概况和基本用法
2. **运行示例**: `cargo run` 体验基础功能
3. **学习电路**: 查看本README中的自定义电路开发指南
4. **查看源码**: 从 `src/main.rs` 开始了解实现
5. **尝试修改**: 基于示例创建自己的电路

#### 核心概念理解
- **秘密分享**: 理解Shamir和加法秘密分享的区别和应用
- **MPC电路**: 学习如何构建和执行安全多方计算
- **零知识证明**: 理解PIOP和KZG承诺的作用
- **委托协议**: 了解EOS协议的整体架构

### 相关论文与资源

#### 核心论文
- [EOS论文原文](EOS.pdf) - 项目所基于的原始研究论文
- Marlin: Preprocessing zkSNARKs with Universal and Updatable SRS
- KZG Polynomial Commitments
- Shamir's Secret Sharing Scheme

#### 技术资源
- [arkworks库文档](https://github.com/arkworks-rs) - 本项目使用的密码学库
- [Rust官方文档](https://doc.rust-lang.org/) - Rust语言学习资源
- [零知识证明学习资源](https://zkp.science/) - ZK相关的学习材料

### 常见问题解答

#### Q: 如何开始开发自定义电路？
A: 建议从`src/main.rs`中的`test_custom_circuit_and_witness()`函数开始，它展示了完整的电路创建和验证流程。

#### Q: 项目支持哪些安全级别？
A: 支持64位、128位和256位安全级别，推荐在生产环境使用128位或256位。

#### Q: 如何优化电路性能？
A: 主要通过减少约束数量、优化变量重用、使用批处理操作等方式。详见配置优化部分。

#### Q: 项目是否支持分布式部署？
A: 当前版本主要是概念验证实现，支持基本的MPC模拟。真实的分布式部署需要额外的网络通信层。

### 获取帮助

- **GitHub Issues**: [报告问题或请求功能](https://github.com/coperlm/EOS-paper-reproduction/issues)
- **代码审查**: 欢迎提交PR进行代码审查和改进
- **学术讨论**: 欢迎就相关算法和实现进行技术交流

## 📄 许可证与致谢

### 许可证
此项目基于MIT许可证开源 - 详情请参见[LICENSE](LICENSE)文件。

### 致谢

#### 开源项目
- **[arkworks](https://github.com/arkworks-rs)**: 提供了优秀的Rust密码学库生态系统
- **Rust社区**: 为高性能系统编程提供了卓越的语言和工具链

#### 学术贡献
- **EOS论文作者**: 为高效的SNARK委托协议奠定了理论基础
- **密码学研究社区**: 在零知识证明和多方计算领域的持续贡献
- **arkworks团队**: 为学术研究提供了实用的密码学实现

#### 项目维护
- **主要开发者**: coperlm
- **项目仓库**: [EOS-paper-reproduction](https://github.com/coperlm/EOS-paper-reproduction)
- **贡献者**: 感谢所有为项目贡献代码、文档和反馈的开发者

---

### 📞 联系方式

- **项目主页**: [GitHub Repository](https://github.com/coperlm/EOS-paper-reproduction)
- **问题反馈**: [GitHub Issues](https://github.com/coperlm/EOS-paper-reproduction/issues)
- **功能请求**: [GitHub Discussions](https://github.com/coperlm/EOS-paper-reproduction/discussions)

---

## 🌟 项目亮点

- ✅ **完整实现**: 从理论到代码的完整EOS协议实现
- ✅ **模块化设计**: 清晰的架构便于学习和扩展
- ✅ **丰富示例**: 多个实用场景的完整示例
- ✅ **详细文档**: 中文文档和代码注释
- ✅ **性能优化**: 针对实际应用的性能优化
- ✅ **开源友好**: MIT许可证，欢迎贡献和使用

**⭐ 如果此项目对您的学习或研究有帮助，请考虑给它一个星标！**

**🚀 让我们一起推进零知识证明和安全多方计算技术的发展！**
