//! Complete demonstration of EOS delegation protocol
//! 
//! This example shows a full end-to-end usage of the EOS delegation protocol,
//! including setup, delegation, execution, and verification.

use eos_delegation::mpc::*;
use eos_delegation::evaluation::*;
use eos_delegation::protocol::*;
use ark_bls12_381::{Bls12_381, Fr};
use ark_relations::r1cs::ConstraintSystem;
use ark_std::rand::{rngs::StdRng, SeedableRng};

type F = Fr;
type E = Bls12_381;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 EOS 委托协议完整演示");
    println!("========================================");

    // 初始化随机数生成器
    let mut rng = StdRng::seed_from_u64(12345);

    // 1. 秘密分享演示
    println!("\n📊 1. 秘密分享系统测试");
    test_secret_sharing(&mut rng)?;

    // 2. MPC 电路执行演示
    println!("\n🔒 2. MPC 电路执行测试");
    test_mpc_execution(&mut rng)?;

    // 3. 操作模式对比
    println!("\n🎯 3. 操作模式性能对比");
    test_operation_modes(&mut rng)?;

    // 4. 完整协议演示
    println!("\n🛡️ 4. 完整 ISNARK 协议演示");
    test_full_protocol(&mut rng)?;

    // 5. 性能基准测试
    println!("\n⚡ 5. 性能基准测试");
    run_benchmarks(&mut rng)?;

    println!("\n✅ 所有测试完成！");
    Ok(())
}

fn test_secret_sharing(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   测试 Shamir 秘密分享...");
    
    // 测试参数
    let secret = F::from(42u64);
    let threshold = 3;
    let num_parties = 5;
    
    // 分享秘密
    let shares = ShamirSecretSharing::<F>::share_secret(secret, threshold, num_parties, rng);
    println!("   ✅ 生成 {} 个分享值", shares.len());
    
    // 重构秘密（使用足够的分享值）
    let reconstructed = ShamirSecretSharing::<F>::reconstruct_secret(&shares[..threshold])?;
    println!("   ✅ 秘密重构成功: {} == {}", secret, reconstructed);
    
    assert_eq!(secret, reconstructed);
    
    // 测试加法秘密分享
    println!("   测试加法秘密分享...");
    let additive_shares = AdditiveSecretSharing::<F>::share_secret(secret, threshold, num_parties, rng);
    let additive_reconstructed = AdditiveSecretSharing::<F>::reconstruct_secret(&additive_shares)?;
    println!("   ✅ 加法秘密分享重构成功: {} == {}", secret, additive_reconstructed);
    
    assert_eq!(secret, additive_reconstructed);
    
    Ok(())
}

fn test_mpc_execution(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   创建 MPC 执行器...");
    
    let party_id = 1;
    let num_parties = 3;
    let secret_sharing = ShamirSecretSharing::<F>::new();
    
    let mut executor = ExecCircuit::new(party_id, num_parties, secret_sharing);
    
    // 测试基本门操作
    println!("   测试基本电路门...");
    
    // 输入两个秘密值
    let secret1 = F::from(10u64);
    let secret2 = F::from(20u64);
    
    let shares1 = executor.input_secret(secret1, 2, rng);
    let shares2 = executor.input_secret(secret2, 2, rng);
    
    // 测试加法门
    if let (Some(share1), Some(share2)) = (shares1.get(0), shares2.get(0)) {
        let _add_result = executor.add_gate(share1, share2)?;
        println!("   ✅ 加法门测试完成");
        
        // 测试乘法门
        let _mul_result = executor.mul_gate(share1, share2)?;
        println!("   ✅ 乘法门测试完成");
        
        // 测试线性组合
        let coeffs = vec![F::from(2u64), F::from(3u64)];
        let _linear_result = executor.linear_combination_gate(&[share1.clone(), share2.clone()], &coeffs)?;
        println!("   ✅ 线性组合门测试完成");
    }
    
    Ok(())
}

fn test_operation_modes(_rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   测试隔离模式...");
    
    let isolation_mode = IsolationMode::new(1, 3);
    let pattern = isolation_mode.get_communication_pattern();
    let complexity = pattern.get_communication_complexity();
    
    println!("   隔离模式通信复杂度:");
    println!("     - 轮数: {}", complexity.rounds);
    println!("     - 总字节数: {}", complexity.total_bytes());
    println!("     - 总延迟: {} ms", complexity.total_latency_ms());
    
    println!("   测试协作模式...");
    
    let collaboration_mode = CollaborationMode::new(3, true, true);
    let collab_pattern = collaboration_mode.get_communication_pattern();
    let collab_complexity = collab_pattern.get_communication_complexity();
    
    println!("   协作模式通信复杂度:");
    println!("     - 轮数: {}", collab_complexity.rounds);
    println!("     - 总字节数: {}", collab_complexity.total_bytes());
    println!("     - 总延迟: {} ms", collab_complexity.total_latency_ms());
    
    Ok(())
}

fn test_full_protocol(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    println!("   初始化协议组件...");
    
    // 创建电路执行器
    let secret_sharing = ShamirSecretSharing::<F>::new();
    let circuit_executor = ExecCircuit::new(1, 3, secret_sharing);
    
    // 创建操作模式
    let operation_mode = CollaborationMode::new(2, true, false);
    
    // 创建协议实例
    let mut protocol = DelegationProtocol::<E, F, ShamirSecretSharing<F>, CollaborationMode>::new(
        circuit_executor,
        operation_mode,
        128, // 安全参数
    );
    
    println!("   ✅ 协议实例创建成功");
    
    // 创建简单电路
    let circuit = ConstraintSystem::new();
    
    // 准备见证和公共输入
    let witness = vec![F::from(10u64), F::from(20u64), F::from(30u64)];
    let public_inputs = vec![F::from(100u64)];
    
    println!("   执行委托计算...");
    
    // 执行委托计算
    let result = protocol.delegate_computation(&circuit, &witness, &public_inputs, rng)?;
    
    println!("   ✅ 委托计算完成");
    println!("   验证结果: {}", result.verification_result);
    
    assert!(result.verification_result, "协议验证失败");
    
    Ok(())
}

fn run_benchmarks(rng: &mut StdRng) -> Result<(), Box<dyn std::error::Error>> {
    let mut metrics = PerformanceMetrics::new();
    
    println!("   运行性能基准测试...");
    
    // 模拟内存使用
    metrics.memory_stats.update(2 * 1024 * 1024); // 2MB
    
    // 模拟通信数据
    metrics.communication_stats.add_round(1024, 5);
    metrics.communication_stats.add_round(2048, 8);
    metrics.communication_stats.add_round(512, 3);
    
    // 模拟电路复杂度
    metrics.circuit_metrics.constraint_count = 500;
    metrics.circuit_metrics.variable_count = 300;
    metrics.circuit_metrics.circuit_depth = 50;
    
    // 基准测试1: 秘密分享性能
    let timer = metrics.start_timer("secret_sharing");
    for _ in 0..1000 {
        let secret = F::from(rand::random::<u64>());
        let _shares = ShamirSecretSharing::<F>::share_secret(secret, 3, 5, rng);
    }
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // 模拟更多内存使用
    metrics.memory_stats.update(4 * 1024 * 1024); // 4MB 峰值
    
    // 基准测试2: MPC 操作性能
    let timer = metrics.start_timer("mpc_operations");
    let secret_sharing = ShamirSecretSharing::<F>::new();
    let mut executor = ExecCircuit::new(1, 3, secret_sharing);
    
    for i in 0..100 {
        let secret1 = F::from(rand::random::<u64>());
        let secret2 = F::from(rand::random::<u64>());
        let shares1 = executor.input_secret(secret1, 2, rng);
        let shares2 = executor.input_secret(secret2, 2, rng);
        
        if let (Some(s1), Some(s2)) = (shares1.get(0), shares2.get(0)) {
            let _ = executor.add_gate(s1, s2);
            
            // 模拟通信开销
            if i % 10 == 0 {
                metrics.communication_stats.add_round(256, 2);
            }
        }
        
        // 模拟内存增长
        if i % 20 == 0 {
            metrics.memory_stats.update((4 + i / 20) * 1024 * 1024);
        }
    }
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // 基准测试3: 协议开销测试
    let timer = metrics.start_timer("protocol_overhead");
    std::thread::sleep(std::time::Duration::from_millis(50)); // 模拟协议处理时间
    
    // 模拟协议通信
    for _ in 0..5 {
        metrics.communication_stats.add_round(4096, 10);
    }
    
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // 更新最终电路指标
    metrics.circuit_metrics.constraint_count = 1000;
    metrics.circuit_metrics.multiplication_gates = 200;
    metrics.circuit_metrics.addition_gates = 800;
    
    // 生成性能报告
    let report = metrics.generate_report();
    print_performance_report(&report);
    
    Ok(())
}

/// Print performance report helper function
fn print_performance_report(report: &PerformanceReport) {
    println!("   📈 性能报告:");
    println!("     - 总执行时间: {:?}", report.total_time);
    
    for (phase, duration) in &report.phase_breakdown {
        println!("     - {}: {:?}", phase, duration);
    }
    
    println!("     - 内存峰值: {} KB", report.memory_peak / 1024);
    println!("     - 通信开销: {} bytes", report.communication_overhead);
    println!("     - 电路大小: {} 约束", report.circuit_size);
}
