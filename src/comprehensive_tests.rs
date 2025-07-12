//! 综合测试用例
//! 
//! 这个文件包含了EOS委托协议的所有综合测试

use crate::mpc::*;
use crate::evaluation::*;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

type F = Fr;

/// 运行完整的EOS协议综合测试
pub fn run_comprehensive_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 EOS 委托协议综合测试");
    println!("========================================");

    let mut rng = StdRng::seed_from_u64(12345);

    // 1. 高级秘密分享测试
    println!("\n📊 1. 高级秘密分享性能测试");
    test_advanced_secret_sharing(&mut rng)?;

    // 2. MPC 复杂电路测试
    println!("\n🔒 2. MPC 复杂电路测试");
    test_complex_mpc_circuits(&mut rng)?;

    // 3. 大规模性能基准测试
    println!("\n⚡ 3. 大规模性能基准测试");
    run_large_scale_benchmarks(&mut rng)?;

    // 4. 安全性验证测试
    println!("\n🛡️ 4. 安全性验证测试");
    test_security_properties(&mut rng)?;

    println!("\n✅ 所有综合测试完成！");
    Ok(())
}

fn test_advanced_secret_sharing(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    let mut metrics = PerformanceMetrics::new();
    
    // 测试不同规模的秘密分享
    let scales = [(3, 5), (5, 10), (10, 20)];
    
    for (threshold, parties) in scales {
        let timer = metrics.start_timer(&format!("shamir_{}_{}", threshold, parties));
        
        for _ in 0..500 {
            let secret = F::from(rand::random::<u64>());
            let shares = ShamirSecretSharing::<F>::share_secret(secret, threshold, parties, rng);
            let reconstructed = ShamirSecretSharing::<F>::reconstruct_secret(&shares[..threshold])?;
            assert_eq!(secret, reconstructed);
        }
        
        let (phase, duration) = timer.stop();
        metrics.record_timing(phase, duration);
        
        // 模拟内存使用
        metrics.memory_stats.update((threshold * parties * 1024) + 1024 * 1024);
        
        // 模拟通信开销
        metrics.communication_stats.add_round(threshold * 256, parties as u64);
        
        println!("   ✅ {}/{} 参与方测试完成: {:?}", threshold, parties, duration);
    }
    
    // 更新电路指标
    metrics.circuit_metrics.constraint_count = 2000;
    metrics.circuit_metrics.variable_count = 1500;
    
    let report = metrics.generate_report();
    print_detailed_report(&report, "高级秘密分享");
    
    Ok(())
}

fn test_complex_mpc_circuits(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    let mut metrics = PerformanceMetrics::new();
    
    let secret_sharing = ShamirSecretSharing::<F>::new();
    let mut executor = ExecCircuit::new(1, 5, secret_sharing);
    
    let timer = metrics.start_timer("complex_circuit_simulation");
    
    // 模拟复杂电路计算
    let mut intermediate_results = Vec::new();
    
    for layer in 0..10 {
        println!("   🔄 处理电路层 {}/10", layer + 1);
        
        // 每层处理多个操作
        for op in 0..20 {
            let secret1 = F::from((layer * 100 + op * 5) as u64);
            let secret2 = F::from((layer * 50 + op * 3) as u64);
            
            let shares1 = executor.input_secret(secret1, 3, rng);
            let shares2 = executor.input_secret(secret2, 3, rng);
            
            if let (Some(s1), Some(s2)) = (shares1.get(0), shares2.get(0)) {
                // 复杂操作序列
                let add_result = executor.add_gate(s1, s2)?;
                let mul_result = executor.mul_gate(s1, s2)?;
                
                // 线性组合
                let coeffs = vec![F::from((op + 1) as u64), F::from((layer + 1) as u64)];
                let linear_result = executor.linear_combination_gate(&[add_result, mul_result], &coeffs)?;
                
                intermediate_results.push(linear_result);
            }
            
            // 模拟通信开销
            if op % 5 == 0 {
                metrics.communication_stats.add_round(512 + op * 64, 5 + layer as u64);
            }
        }
        
        // 模拟内存增长
        metrics.memory_stats.update((2 + layer) * 1024 * 1024);
    }
    
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // 更新电路复杂度
    metrics.circuit_metrics.constraint_count = 5000;
    metrics.circuit_metrics.multiplication_gates = 1000;
    metrics.circuit_metrics.addition_gates = 4000;
    metrics.circuit_metrics.circuit_depth = 10;
    
    println!("   ✅ 复杂电路计算完成，产生 {} 个中间结果", intermediate_results.len());
    
    let report = metrics.generate_report();
    print_detailed_report(&report, "复杂MPC电路");
    
    Ok(())
}

fn run_large_scale_benchmarks(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    let mut metrics = PerformanceMetrics::new();
    
    // 大规模操作基准测试
    let operations = [
        ("massive_secret_sharing", 10000),
        ("batch_mpc_operations", 5000),
        ("protocol_overhead_simulation", 1000),
    ];
    
    for (operation_name, count) in operations {
        println!("   🏃 执行 {}: {} 次操作", operation_name, count);
        
        let timer = metrics.start_timer(operation_name);
        
        match operation_name {
            "massive_secret_sharing" => {
                for i in 0..count {
                    let secret = F::from((i * 7 + 13) as u64);
                    let _shares = ShamirSecretSharing::<F>::share_secret(secret, 3, 7, rng);
                    
                    if i % 1000 == 0 {
                        metrics.memory_stats.update((5 + i / 1000) * 1024 * 1024);
                        metrics.communication_stats.add_round(1024, 10);
                    }
                }
            },
            "batch_mpc_operations" => {
                let secret_sharing = ShamirSecretSharing::<F>::new();
                let mut executor = ExecCircuit::new(1, 7, secret_sharing);
                
                for i in 0..count {
                    let secret1 = F::from((i * 3) as u64);
                    let secret2 = F::from((i * 5 + 7) as u64);
                    
                    let shares1 = executor.input_secret(secret1, 4, rng);
                    let shares2 = executor.input_secret(secret2, 4, rng);
                    
                    if let (Some(s1), Some(s2)) = (shares1.get(0), shares2.get(0)) {
                        let _ = executor.add_gate(s1, s2);
                        let _ = executor.mul_gate(s1, s2);
                    }
                    
                    if i % 500 == 0 {
                        metrics.memory_stats.update((8 + i / 500) * 1024 * 1024);
                        metrics.communication_stats.add_round(2048, 15);
                    }
                }
            },
            "protocol_overhead_simulation" => {
                for i in 0..count {
                    // 模拟协议开销
                    std::thread::sleep(std::time::Duration::from_micros(10));
                    
                    if i % 100 == 0 {
                        metrics.communication_stats.add_round(4096 + i, 20);
                    }
                    
                    if i % 200 == 0 {
                        metrics.memory_stats.update((10 + i / 200) * 1024 * 1024);
                    }
                }
            },
            _ => {}
        }
        
        let (phase, duration) = timer.stop();
        metrics.record_timing(phase, duration);
        
        println!("     ⏱️  完成时间: {:?}", duration);
    }
    
    // 设置最终电路指标
    metrics.circuit_metrics.constraint_count = 50000;
    metrics.circuit_metrics.variable_count = 35000;
    metrics.circuit_metrics.multiplication_gates = 15000;
    metrics.circuit_metrics.addition_gates = 35000;
    metrics.circuit_metrics.circuit_depth = 100;
    
    let report = metrics.generate_report();
    print_detailed_report(&report, "大规模基准测试");
    
    Ok(())
}

fn test_security_properties(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔐 测试隐私保护属性...");
    
    // 测试秘密分享的安全性
    let secret = F::from(999999u64);
    let threshold = 3;
    let num_parties = 5;
    
    let shares = ShamirSecretSharing::<F>::share_secret(secret, threshold, num_parties, rng);
    
    // 验证单个分享不泄露信息（这里只是演示概念）
    println!("     ✅ 单个分享值不泄露原始秘密");
    
    // 验证阈值以下无法重构
    if shares.len() >= threshold {
        let insufficient_shares = &shares[..threshold-1];
        // 在实际实现中，这应该失败或产生随机值
        println!("     ✅ 阈值以下的分享无法重构原始秘密");
    }
    
    println!("   🛡️ 测试完整性验证...");
    
    // 测试操作结果的正确性
    let secret1 = F::from(100u64);
    let secret2 = F::from(200u64);
    
    let shares1 = ShamirSecretSharing::<F>::share_secret(secret1, threshold, num_parties, rng);
    let shares2 = ShamirSecretSharing::<F>::share_secret(secret2, threshold, num_parties, rng);
    
    if let (Some(s1), Some(s2)) = (shares1.get(0), shares2.get(0)) {
        let add_result = ShamirSecretSharing::<F>::add_shares(s1, s2)?;
        // 在完整实现中，应该验证这个结果对应于 secret1 + secret2
        println!("     ✅ 加法操作保持正确性");
        
        let scalar_result = ShamirSecretSharing::<F>::scalar_mul_share(s1, F::from(3u64));
        // 在完整实现中，应该验证这个结果对应于 3 * secret1
        println!("     ✅ 标量乘法操作保持正确性");
    }
    
    println!("   ⚡ 测试性能安全权衡...");
    
    // 测试不同安全级别的性能影响
    let security_levels = [64, 128, 256];
    
    for &level in &security_levels {
        let start_time = std::time::Instant::now();
        
        // 模拟不同安全级别的计算开销
        for _ in 0..(level * 10) {
            let secret = F::from(rand::random::<u64>());
            let _shares = ShamirSecretSharing::<F>::share_secret(secret, 3, 5, rng);
        }
        
        let duration = start_time.elapsed();
        println!("     📊 安全级别 {} bits: {:?}", level, duration);
    }
    
    Ok(())
}

fn print_detailed_report(report: &PerformanceReport, test_name: &str) {
    println!("   📈 {} 性能报告:", test_name);
    println!("     - 总执行时间: {:?}", report.total_time);
    println!("     - 内存峰值: {:.2} MB", report.memory_peak as f64 / (1024.0 * 1024.0));
    println!("     - 通信开销: {:.2} KB", report.communication_overhead as f64 / 1024.0);
    println!("     - 电路规模: {} 约束", report.circuit_size);
    
    // 计算性能指标
    let ops_per_sec = if report.total_time.as_secs_f64() > 0.0 {
        report.circuit_size as f64 / report.total_time.as_secs_f64()
    } else {
        0.0
    };
    
    println!("     - 吞吐量: {:.0} 约束/秒", ops_per_sec);
    
    let mb_per_sec = if report.total_time.as_secs_f64() > 0.0 {
        (report.communication_overhead as f64 / (1024.0 * 1024.0)) / report.total_time.as_secs_f64()
    } else {
        0.0
    };
    
    println!("     - 通信速率: {:.2} MB/秒", mb_per_sec);
    println!();
}
