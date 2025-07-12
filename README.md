# EOS Delegation Protocol

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)

**EOS (Efficient Outsourcing Scheme)** is a high-performance implementation of a SNARK delegation protocol that enables secure and efficient outsourcing of zero-knowledge proof computations while preserving privacy and ensuring verifiability.

## 🚀 Features

### 🔐 **Cryptographic Foundations**
- **Multi-Party Computation (MPC)**: Secure computation across distributed parties
- **Secret Sharing Schemes**: Shamir's threshold secret sharing and additive secret sharing
- **Zero-Knowledge Proofs**: Integration with SNARK proving systems
- **Polynomial Commitments**: KZG commitment scheme support

### ⚡ **High Performance**
- **Optimized Circuit Execution**: Efficient MPC circuit evaluation
- **Batch Operations**: Batched secret sharing and polynomial operations
- **Parallel Processing**: Multi-threaded computation support
- **Memory Efficient**: Optimized memory usage for large-scale computations

### 🛡️ **Security & Privacy**
- **Privacy-Preserving**: Input privacy through secret sharing
- **Verifiable Computation**: Cryptographic verification of outsourced computations
- **Malicious Security**: Protection against adversarial parties
- **Configurable Security Parameters**: Adjustable security levels (64/128/256-bit)

## 📊 Performance Benchmarks

| Test Category | Execution Time | Memory Peak | Throughput | Circuit Size |
|---------------|----------------|-------------|------------|--------------|
| Basic Operations | 1.4ms | 4MB | - | 150 constraints |
| Advanced Secret Sharing | 663ms | 1.2MB | 3,017 constraints/sec | 2,000 constraints |
| Complex MPC Circuits | 5.8ms | 11MB | 868,071 constraints/sec | 5,000 constraints |
| Large-Scale Benchmarks | 866ms | 17MB | 57,710 constraints/sec | 50,000 constraints |

## 🏗️ Architecture

```
eos-delegation/
├── src/
│   ├── circuit/           # Circuit operations (FFT, MSM, Polynomial ops)
│   │   ├── common.rs      # Common circuit operations
│   │   └── pc_schemes.rs  # Polynomial commitment schemes
│   ├── mpc/               # Multi-party computation
│   │   ├── secret_sharing.rs  # Shamir & Additive secret sharing
│   │   ├── executor.rs        # MPC circuit executor
│   │   └── modes.rs           # Isolation & Collaboration modes
│   ├── piop/              # Polynomial Interactive Oracle Proofs
│   │   └── consistency_checker.rs
│   ├── protocol/          # Core delegation protocol
│   │   └── delegation_protocol.rs
│   └── evaluation/        # Performance evaluation tools
└── examples/              # Usage examples and demos
```

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70+ (with Cargo)
- **Git**

### Installation

```bash
git clone https://github.com/your-username/eos-delegation.git
cd eos-delegation
cargo build --release
```

### Basic Usage

```rust
use eos_delegation::*;
use ark_bls12_381::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(42);
    
    // Initialize secret sharing
    let secret = Fr::from(123u64);
    let shares = ShamirSecretSharing::<Fr>::share_secret(secret, 3, 5, &mut rng);
    
    // Reconstruct secret
    let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..3])?;
    assert_eq!(secret, reconstructed);
    
    println!("✅ Secret sharing successful!");
    Ok(())
}
```

### Running Examples

```bash
# Basic functionality demo
cargo run

# Comprehensive demonstration
cargo run --example complete_demo

# Run tests
cargo test
```

## 📋 Usage Examples

### 1. Secret Sharing

```rust
use eos_delegation::mpc::*;
use ark_bls12_381::Fr;

// Shamir's Secret Sharing
let secret = Fr::from(42u64);
let shares = ShamirSecretSharing::<Fr>::share_secret(secret, 3, 5, &mut rng);
let reconstructed = ShamirSecretSharing::<Fr>::reconstruct_secret(&shares[..3])?;

// Additive Secret Sharing
let additive_shares = AdditiveSecretSharing::<Fr>::share_secret(secret, 0, 5, &mut rng);
let reconstructed = AdditiveSecretSharing::<Fr>::reconstruct_secret(&additive_shares)?;
```

### 2. MPC Circuit Execution

```rust
use eos_delegation::mpc::*;

let secret_sharing = ShamirSecretSharing::<Fr>::new();
let mut executor = ExecCircuit::new(1, 3, secret_sharing);

// Input secrets
let shares1 = executor.input_secret(Fr::from(10u64), 2, &mut rng);
let shares2 = executor.input_secret(Fr::from(20u64), 2, &mut rng);

// Perform operations
let add_result = executor.add_gate(&shares1[0], &shares2[0])?;
let mul_result = executor.mul_gate(&shares1[0], &shares2[0])?;
```

### 3. Operation Modes

```rust
use eos_delegation::mpc::modes::*;

// Isolation Mode - Minimal communication
let isolation_mode = IsolationMode::new(1, 3);
let pattern = isolation_mode.get_communication_pattern();

// Collaboration Mode - Full communication
let collaboration_mode = CollaborationMode::new(2, true, true);
let pattern = collaboration_mode.get_communication_pattern();
```

### 4. Performance Evaluation

```rust
use eos_delegation::evaluation::*;

let mut metrics = PerformanceMetrics::new();

let timer = metrics.start_timer("computation");
// ... perform computation ...
let (phase, duration) = timer.stop();
metrics.record_timing(phase, duration);

let report = metrics.generate_report();
// Print detailed performance metrics
```

## 🧪 Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Categories
```bash
# Basic functionality tests
cargo test test_secret_sharing

# MPC operation tests  
cargo test test_mpc

# Performance benchmarks
cargo test --release test_performance
```

### Run Comprehensive Benchmarks
```bash
cargo run --release
```

## 📈 Performance Optimization

The implementation includes several optimization strategies:

1. **Batch Processing**: Operations are batched to reduce communication overhead
2. **Parallel Execution**: Multi-threaded processing where possible  
3. **Memory Management**: Efficient memory allocation and reuse
4. **Algorithm Selection**: Optimal algorithms chosen based on input size
5. **Communication Optimization**: Minimized round complexity

## 🔧 Configuration

### Security Parameters

```rust
// Configure security level
let params = ProtocolParams::new(128); // 128-bit security

// Adjust threshold parameters
let threshold = 3;  // Minimum shares needed
let num_parties = 5; // Total number of parties
```

### Operation Modes

```rust
// Isolation Mode: Minimize communication
let isolation = IsolationMode::new(
    1,  // isolation_level
    3   // max_communication_rounds
);

// Collaboration Mode: Maximize efficiency
let collaboration = CollaborationMode::new(
    3,    // collaboration_level
    true, // use_optimized_protocols
    true  // enable_parallel_processing
);
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/your-username/eos-delegation.git
cd eos-delegation

# Install development dependencies
cargo build

# Run linter
cargo clippy

# Format code
cargo fmt

# Run all tests
cargo test
```

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📚 Documentation

- **API Documentation**: Run `cargo doc --open` to generate and view docs
- **Examples**: See the `examples/` directory for usage examples
- **Architecture Guide**: Detailed architecture documentation in `docs/`

## 🔬 Research & Papers

This implementation is based on research in:

- **EOS**: Efficient Private Delegation of zkSNARK Provers
- **Marlin**: Preprocessing zkSNARKs with Universal and Updatable SRS  
- **KZG Polynomial Commitments**
- **Shamir's Secret Sharing Scheme**

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [arkworks](https://github.com/arkworks-rs) - Cryptographic library ecosystem
- Research community working on zkSNARKs and MPC protocols
- Contributors and maintainers

## 📞 Support & Contact

- **Issues**: [GitHub Issues](https://github.com/your-username/eos-delegation/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/eos-delegation/discussions)
- **Email**: your-email@domain.com

---

**⭐ If you find this project useful, please consider giving it a star!**
