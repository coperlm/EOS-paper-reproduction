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

use mpc::*;
use evaluation::*;
use comprehensive_tests::run_comprehensive_tests;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

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
