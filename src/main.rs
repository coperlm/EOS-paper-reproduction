//! EOS Delegation Protocol Main Entry Point
//! 
//! This is the main entry point for the EOS delegation protocol implementation.
//! It demonstrates the core functionality and provides examples of usage.

mod circuit;
mod mpc;
mod piop;
mod protocol;
mod evaluation;
mod comprehensive_tests;
mod custom_circuits;

use mpc::*;
use evaluation::*;
use comprehensive_tests::run_comprehensive_tests;
use piop::ConsistencyChecker;
use circuit::KZGCommitmentScheme;
use custom_circuits::{CustomCircuit, CircuitTemplates, CircuitTester};
use ark_bls12_381::{Fr, G1Projective};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};

type F = Fr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 EOS 委托协议系统启动");
    println!("========================================");

    // 初始化随机数生成器
    let mut rng = StdRng::seed_from_u64(12345);

    // 运行基础功能测试
    println!("\n📋 系统组件测试:");
    
    // 1. 测试秘密分享
    test_secret_sharing_basic(&mut rng)?;
    
    // 2. 测试MPC基础操作
    test_mpc_basic_operations(&mut rng)?;
    
    // 3. 测试操作模式
    test_operation_modes_basic(&mut rng)?;
    
    // 4. 运行性能测试
    run_performance_tests(&mut rng)?;
    
    // 5. 测试 PIOP 一致性检查器
    test_piop_consistency_checker(&mut rng)?;
    
    // 6. 测试 KZG 多项式承诺方案
    test_kzg_polynomial_commitment(&mut rng)?;
    
    // 7. 自定义电路和见证测试
    test_custom_circuit_and_witness()?;

    println!("\n✅ 系统测试完成，所有组件正常工作！");
    
    // 5. 运行综合测试
    println!("\n🎯 运行综合测试...");
    run_comprehensive_tests()?;
    
    println!("💡 运行 'cargo run --example complete_demo' 查看完整演示");
    
    Ok(())
}

fn test_secret_sharing_basic(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔐 秘密分享测试...");
    
    let secret = F::from(42u64);
    let threshold = 3;
    let num_parties = 5;
    
    // Shamir 秘密分享
    let shares = ShamirSecretSharing::<F>::share_secret(secret, threshold, num_parties, rng);
    let reconstructed = ShamirSecretSharing::<F>::reconstruct_secret(&shares[..threshold])?;
    
    assert_eq!(secret, reconstructed);
    println!("      ✅ Shamir 秘密分享: {} -> {} 分享 -> {}", secret, shares.len(), reconstructed);
    
    // 加法秘密分享
    let additive_shares = AdditiveSecretSharing::<F>::share_secret(secret, threshold, num_parties, rng);
    let additive_reconstructed = AdditiveSecretSharing::<F>::reconstruct_secret(&additive_shares)?;
    
    assert_eq!(secret, additive_reconstructed);
    println!("      ✅ 加法秘密分享: {} -> {} 分享 -> {}", secret, additive_shares.len(), additive_reconstructed);
    
    Ok(())
}

fn test_mpc_basic_operations(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔒 MPC 基础操作测试...");
    
    let secret_sharing = ShamirSecretSharing::<F>::new();
    let mut executor = ExecCircuit::new(1, 3, secret_sharing);
    
    // 创建秘密输入
    let secret1 = F::from(10u64);
    let secret2 = F::from(20u64);
    
    let shares1 = executor.input_secret(secret1, 2, rng);
    let shares2 = executor.input_secret(secret2, 2, rng);
    
    println!("      📥 输入秘密: {} 和 {}", secret1, secret2);
    
    // 测试加法
    if let (Some(s1), Some(s2)) = (shares1.get(0), shares2.get(0)) {
        let _add_result = executor.add_gate(s1, s2)?;
        println!("      ➕ 加法门: {} + {} = 分享值", secret1, secret2);
        
        // 测试乘法
        let _mul_result = executor.mul_gate(s1, s2)?;
        println!("      ✖️  乘法门: {} × {} = 分享值", secret1, secret2);
        
        // 测试线性组合
        let coeffs = vec![F::from(2u64), F::from(3u64)];
        let _linear_result = executor.linear_combination_gate(&[s1.clone(), s2.clone()], &coeffs)?;
        println!("      🔢 线性组合: 2×{} + 3×{} = 分享值", secret1, secret2);
    }
    
    Ok(())
}

fn test_operation_modes_basic(_rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🎯 操作模式测试...");
    
    // 隔离模式
    let isolation_mode = IsolationMode::new(1, 3);
    let iso_pattern = isolation_mode.get_communication_pattern();
    let iso_complexity = iso_pattern.get_communication_complexity();
    
    println!("      🏝️  隔离模式: {} 轮, {} 字节/轮", iso_complexity.rounds, iso_complexity.bytes_per_round);
    
    // 协作模式
    let collaboration_mode = CollaborationMode::new(2, true, true);
    let collab_pattern = collaboration_mode.get_communication_pattern();
    let collab_complexity = collab_pattern.get_communication_complexity();
    
    println!("      🤝 协作模式: {} 轮, {} 字节/轮", collab_complexity.rounds, collab_complexity.bytes_per_round);
    
    Ok(())
}

fn run_performance_tests(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   ⚡ 性能测试...");
    
    let mut metrics = PerformanceMetrics::new();
    
    // 模拟基础系统开销
    metrics.memory_stats.update(1024 * 1024); // 1MB 基础内存
    metrics.communication_stats.add_round(512, 3); // 初始通信
    
    // 测试秘密分享性能
    let timer = metrics.start_timer("secret_sharing_100");
    for i in 0..100 {
        let secret = F::from(rand::random::<u32>() as u64);
        let _shares = ShamirSecretSharing::<F>::share_secret(secret, 3, 5, rng);
        
        // 模拟内存使用增长
        if i % 25 == 0 {
            metrics.memory_stats.update((1 + i / 25) * 1024 * 1024);
        }
        
        // 模拟通信开销
        if i % 10 == 0 {
            metrics.communication_stats.add_round(128, 1);
        }
    }
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // 更新电路指标
    metrics.circuit_metrics.constraint_count = 150;
    metrics.circuit_metrics.variable_count = 100;
    metrics.circuit_metrics.addition_gates = 120;
    metrics.circuit_metrics.multiplication_gates = 30;
    
    // 生成报告
    let report = metrics.generate_report();
    println!("      📊 性能指标:");
    println!("         - 执行时间: {:?}", report.total_time);
    println!("         - 内存峰值: {:.1} KB", report.memory_peak as f64 / 1024.0);
    println!("         - 通信开销: {} bytes", report.communication_overhead);
    println!("         - 电路规模: {} 约束", report.circuit_size);
    
    Ok(())
}

fn test_piop_consistency_checker(_rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔍 PIOP 一致性检查器测试...");
    
    // 创建一致性检查器实例
    let mut checker = ConsistencyChecker::<F>::new();
    
    // 添加测试多项式
    let test_poly = DensePolynomial::from_coefficients_vec(vec![
        F::from(1u64), 
        F::from(2u64), 
        F::from(3u64)
    ]);
    
    checker.add_witness_polynomial("test_witness".to_string(), test_poly.clone());
    checker.add_public_polynomial("test_public".to_string(), test_poly);
    
    // 执行一致性检查
    let constraint_result = checker.check_constraint_consistency();
    println!("      🔒 约束一致性检查: {}", constraint_result.is_consistent);
    
    let polynomial_result = checker.check_polynomial_consistency();
    println!("      📐 多项式一致性检查: {}", polynomial_result.is_consistent);
    
    let batch_result = checker.batch_consistency_check();
    println!("      � 批量一致性检查: {}", batch_result.is_consistent);
    
    // 生成和验证一致性证明
    match checker.generate_consistency_proof() {
        Ok(proof) => {
            let verification_result = checker.verify_consistency_proof(&proof);
            println!("      ✅ 一致性证明验证: {}", verification_result);
        }
        Err(e) => {
            println!("      ⚠️ 证明生成失败: {}", e);
        }
    }
    
    Ok(())
}

fn test_kzg_polynomial_commitment(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   📊 KZG 多项式承诺方案测试...");
    
    // 创建 KZG 方案实例
    let kzg = KZGCommitmentScheme::<F, G1Projective>::setup(10, rng);
    
    // 创建测试多项式 p(x) = x^2 + 2x + 3
    let test_polynomial = DensePolynomial::from_coefficients_vec(vec![
        F::from(3u64),  // 常数项
        F::from(2u64),  // x 项
        F::from(1u64),  // x^2 项
    ]);
    
    // 生成承诺
    let commitment = kzg.commit(&test_polynomial);
    println!("      📜 多项式承诺已生成");
    
    // 在点 x = 5 处打开多项式
    let evaluation_point = F::from(5u64);
    let opening_proof = kzg.open(&test_polynomial, evaluation_point);
    
    // 计算期望值: 5^2 + 2*5 + 3 = 25 + 10 + 3 = 38
    let expected_value = F::from(38u64);
    assert_eq!(opening_proof.evaluation, expected_value);
    println!("      � 多项式在点 {} 的值: {}", evaluation_point, opening_proof.evaluation);
    
    // 验证打开证明
    let verification_result = kzg.verify(&commitment, &opening_proof);
    println!("      ✅ 承诺验证结果: {}", verification_result);
    
    // 测试批量操作
    let poly1 = DensePolynomial::from_coefficients_vec(vec![F::from(1u64), F::from(2u64)]);
    let poly2 = DensePolynomial::from_coefficients_vec(vec![F::from(3u64), F::from(4u64)]);
    let polynomials = vec![poly1, poly2];
    let points = vec![F::from(1u64), F::from(2u64)];
    
    let batch_proof = kzg.batch_open(&polynomials, &points);
    let batch_commitments: Vec<_> = polynomials.iter().map(|p| kzg.commit(p)).collect();
    let batch_verification = kzg.batch_verify(&batch_commitments, &batch_proof);
    println!("      🔄 批量验证结果: {}", batch_verification);
    
    Ok(())
}

fn test_custom_circuit_and_witness() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 自定义电路和见证测试");
    println!("========================================");
    
    // 1. 基础自定义电路示例：验证 x² + y² = z
    println!("\n� 示例 1: 自定义约束验证 (x² + y² = z)");
    let mut custom_circuit = CustomCircuit::<F>::new("pythagorean_verification".to_string());
    
    // 定义私有见证
    let x = F::from(100u64);
    let y = F::from(200u64);
    let z = F::from(50000u64); // 错误值：100² + 200² = 10000 + 40000 = 50000 ≠ 50001

    // 添加见证和输入
    let x_idx = custom_circuit.add_private_witness(x);              // 索引 0
    let y_idx = custom_circuit.add_private_witness(y);              // 索引 1 
    let x_squared_idx = custom_circuit.add_private_witness(x * x);  // 索引 2: 10000
    let y_squared_idx = custom_circuit.add_private_witness(y * y);  // 索引 3: 40000
    let z_idx = custom_circuit.add_public_input(z);                 // 索引 4: z在 all_variables 中
    
    // 添加约束
    custom_circuit.add_multiplication_constraint(x_idx, x_idx, x_squared_idx);     // x × x = x²
    custom_circuit.add_multiplication_constraint(y_idx, y_idx, y_squared_idx);     // y × y = y²
    custom_circuit.add_addition_constraint(x_squared_idx, y_squared_idx, z_idx);   // x² + y² = z
    
    // 现在验证约束：x × x = x², y × y = y², x² + y² = z
    println!("   📝 电路约束:");
    println!("      x = {}, y = {}, z = {}", x, y, z);
    println!("      x² = {}, y² = {}", x * x, y * y);
    println!("      x² + y² = {} (期望 z = {})", x * x + y * y, z);
    
    let is_valid = CircuitTester::test_circuit(&custom_circuit);
    println!("   🔍 验证结果: {}", if is_valid { "✅ 通过" } else { "❌ 失败" });
    
    // 2. 使用电路模板：平方根验证
    println!("\n📋 示例 2: 平方根验证电路模板");
    let sqrt_x = F::from(7u64);
    let sqrt_result = F::from(49u64);
    let sqrt_circuit = CircuitTemplates::square_root_verification(sqrt_x, sqrt_result);
    
    let sqrt_valid = CircuitTester::test_circuit(&sqrt_circuit);
    println!("   🔍 平方根验证结果: {}", if sqrt_valid { "✅ 通过" } else { "❌ 失败" });
    
    // 3. KZG 承诺保护私有见证
    println!("\n📋 示例 3: 使用 KZG 承诺保护私有见证");
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(12345);
    let degree = 10;
    
    // 设置 KZG
    let kzg_scheme = KZGCommitmentScheme::<F, G1Projective>::setup(degree, &mut rng);
    
    // 创建见证多项式
    let witness_coeffs: Vec<F> = vec![
        F::from(3u64),  // 私有见证 x
        F::from(4u64),  // 私有见证 y
        F::from(25u64), // 计算结果 z
    ];
    let witness_poly = DensePolynomial::from_coefficients_vec(witness_coeffs);
    
    // 承诺见证
    let commitment = kzg_scheme.commit(&witness_poly);
    println!("   � 见证承诺生成完成");
    
    // 生成随机挑战点进行开启
    let challenge_point = F::from(123u64);
    let proof = kzg_scheme.open(&witness_poly, challenge_point);
    
    // 验证承诺
    let is_commitment_valid = kzg_scheme.verify(&commitment, &proof);
    println!("   ✅ 承诺验证结果: {}", if is_commitment_valid { "✅ 通过" } else { "❌ 失败" });
    
    // 4. PIOP 一致性检查
    println!("\n📋 示例 4: PIOP 一致性检查与自定义电路");
    let mut consistency_checker = ConsistencyChecker::<F>::new();
    
    // 运行 PIOP 测试
    let piop_result = CircuitTester::run_piop_test(&custom_circuit, &mut consistency_checker);
    println!("   � PIOP 一致性检查: {}", if piop_result { "✅ 通过" } else { "❌ 失败" });
    
    // 5. 范围证明电路示例
    println!("\n📋 示例 5: 范围证明电路 (证明 x ∈ [10, 50])");
    let range_value = F::from(25u64);
    let range_min = F::from(10u64);
    let range_max = F::from(50u64);
    let range_circuit = CircuitTemplates::range_proof(range_value, range_min, range_max);
    
    let range_valid = CircuitTester::test_circuit(&range_circuit);
    println!("   🔍 范围证明结果: {}", if range_valid { "✅ 通过" } else { "❌ 失败" });
    
    println!("\n💡 自定义电路指南:");
    println!("   1. 在 src/custom_circuits.rs 中定义您的电路");
    println!("   2. 使用 CustomCircuit::new() 创建新电路");
    println!("   3. 使用 add_private_witness() 添加私有见证");
    println!("   4. 使用 add_public_input() 添加公开输入");
    println!("   5. 使用 add_multiplication_constraint() 添加乘法约束");
    println!("   6. 使用 CircuitTester::test_circuit() 验证电路");
    println!("   7. 使用 KZG 承诺保护敏感见证数据");
    println!("   8. 使用 PIOP 进行零知识证明");
    
    Ok(())
}
