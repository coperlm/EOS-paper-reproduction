/// 演示如何使用 EOS 委托协议的基本功能

use eos_delegation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 EOS Delegation Protocol Example");
    println!("=====================================");
    
    // Initialize random number generator
    let mut rng = ark_std::test_rng();
    
    // Example 1: Basic polynomial operations
    println!("\n📐 Example 1: Polynomial Operations");
    demonstrate_polynomial_operations()?;
    
    // Example 2: Secret sharing
    println!("\n🔐 Example 2: Secret Sharing");
    demonstrate_secret_sharing(&mut rng)?;
    
    // Example 3: MPC modes
    println!("\n🤝 Example 3: MPC Operation Modes");
    demonstrate_mpc_modes();
    
    // Example 4: Performance evaluation
    println!("\n📊 Example 4: Performance Evaluation");
    demonstrate_performance_evaluation();
    
    println!("\n✅ All examples completed successfully!");
    Ok(())
}

fn demonstrate_polynomial_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating test polynomials...");
    
    // Create some test polynomials
    let poly1 = ark_poly::univariate::DensePolynomial::from_coefficients_vec(vec![
        TestField::from(1u64), TestField::from(2u64), TestField::from(3u64)
    ]);
    let poly2 = ark_poly::univariate::DensePolynomial::from_coefficients_vec(vec![
        TestField::from(4u64), TestField::from(5u64), TestField::from(6u64)
    ]);
    
    // Perform polynomial addition
    let result = PolyAdd::add(poly1, poly2);
    println!("  ✓ Polynomial addition completed");
    println!("  Result degree: {}", result.degree());
    
    Ok(())
}

fn demonstrate_secret_sharing(rng: &mut impl ark_std::rand::Rng) -> Result<(), Box<dyn std::error::Error>> {
    println!("  Creating secret sharing scheme...");
    
    // Create secret to share
    let secret = TestField::from(42u64);
    let threshold = 3;
    let num_parties = 5;
    
    // Create Shamir secret shares
    let shares = ShamirSecretSharing::share_secret(secret, threshold, num_parties, rng);
    println!("  ✓ Created {} shares with threshold {}", shares.len(), threshold);
    
    // Reconstruct secret from shares
    let reconstructed = ShamirSecretSharing::reconstruct_secret(&shares[0..threshold])?;
    println!("  ✓ Secret reconstruction: {}", reconstructed == secret);
    
    Ok(())
}

fn demonstrate_mpc_modes() {
    println!("  Creating operation modes...");
    
    // Create isolation mode
    let isolation_mode = IsolationMode::new(1, 3);
    println!("  ✓ Isolation Mode - Level: {}, Max Rounds: {}", 
             isolation_mode.isolation_level, 
             isolation_mode.max_communication_rounds);
    
    // Create collaboration mode  
    let collaboration_mode = CollaborationMode::new(3, true, true);
    println!("  ✓ Collaboration Mode - Level: {}, Optimized: {}, Parallel: {}", 
             collaboration_mode.collaboration_level,
             collaboration_mode.use_optimized_protocols,
             collaboration_mode.enable_parallel_processing);
    
    // Show communication patterns
    let isolation_pattern = isolation_mode.get_communication_pattern();
    let collaboration_pattern = collaboration_mode.get_communication_pattern();
    
    match isolation_pattern {
        CommunicationPattern::Minimal { max_rounds, batch_size } => {
            println!("  Isolation Pattern: {} rounds, batch size {}", max_rounds, batch_size);
        }
        _ => {}
    }
    
    match collaboration_pattern {
        CommunicationPattern::Full { parallelism_degree, use_optimized_protocols } => {
            println!("  Collaboration Pattern: {} parallel, optimized: {}", 
                     parallelism_degree, use_optimized_protocols);
        }
        _ => {}
    }
}

fn demonstrate_performance_evaluation() {
    println!("  Setting up performance metrics...");
    
    let mut metrics = PerformanceMetrics::new();
    
    // Simulate some operations
    let timer = metrics.start_timer("setup");
    std::thread::sleep(std::time::Duration::from_millis(10));
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    let timer = metrics.start_timer("execution");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let (phase, duration) = timer.stop();
    metrics.record_timing(phase, duration);
    
    // Generate performance report
    let report = metrics.generate_report();
    println!("  ✓ Total execution time: {:.2?}", report.total_time);
    println!("  ✓ Memory peak: {} bytes", report.memory_peak);
    println!("  ✓ Communication overhead: {} bytes", report.communication_overhead);
    
    // Create a benchmark suite
    let mut benchmark_suite = BenchmarkSuite::new();
    
    benchmark_suite.add_test_case(BenchmarkCase {
        name: "Small Circuit".to_string(),
        description: "Test with 100 constraints".to_string(),
        circuit_size: 100,
        num_parties: 3,
        expected_duration_ms: 10,
    });
    
    benchmark_suite.add_test_case(BenchmarkCase {
        name: "Medium Circuit".to_string(), 
        description: "Test with 1000 constraints".to_string(),
        circuit_size: 1000,
        num_parties: 5,
        expected_duration_ms: 100,
    });
    
    println!("  ✓ Created benchmark suite with {} test cases", benchmark_suite.test_cases.len());
}
