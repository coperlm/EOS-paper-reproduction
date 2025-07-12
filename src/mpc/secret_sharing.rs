//! Secret sharing implementation for EOS delegation protocol
//! 
//! This module implements secret sharing schemes used in the MPC components
//! of the EOS delegation protocol to ensure privacy and security.

use ark_ff::{Field, PrimeField};
use ark_std::rand::Rng;

/// A secret sharing scheme trait
pub trait SecretSharing<F: Field>: Clone {
    type Share: Clone;
    type SecretKey;
    
    /// Share a secret among n parties with threshold t
    fn share_secret(
        secret: F,
        threshold: usize,
        num_parties: usize,
        rng: &mut impl Rng,
    ) -> Vec<Self::Share>;
    
    /// Reconstruct secret from shares
    fn reconstruct_secret(shares: &[Self::Share]) -> Result<F, SecretSharingError>;
    
    /// Verify if a share is valid
    fn verify_share(share: &Self::Share, secret_key: &Self::SecretKey) -> bool;
    
    /// Add two shares (local operation for most schemes)
    fn add_shares(left: &Self::Share, right: &Self::Share) -> Result<Self::Share, SecretSharingError>;
    
    /// Multiply two shares (may require communication)
    fn mul_shares(left: &Self::Share, right: &Self::Share) -> Result<Self::Share, SecretSharingError>;
    
    /// Multiply a share by a scalar (local operation)
    fn scalar_mul_share(share: &Self::Share, scalar: F) -> Self::Share;
}

/// Shamir's secret sharing implementation
#[derive(Clone)]
pub struct ShamirSecretSharing<F: PrimeField> {
    _phantom: std::marker::PhantomData<F>,
}

impl<F: PrimeField> ShamirSecretSharing<F> {
    pub fn new() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

#[derive(Debug, Clone)]
pub struct ShamirShare<F: Field> {
    pub index: usize,
    pub value: F,
}

impl<F: PrimeField> SecretSharing<F> for ShamirSecretSharing<F> {
    type Share = ShamirShare<F>;
    type SecretKey = ();
    
    fn share_secret(
        secret: F,
        threshold: usize,
        num_parties: usize,
        rng: &mut impl Rng,
    ) -> Vec<Self::Share> {
        assert!(threshold <= num_parties);
        
        // Generate random polynomial coefficients
        let mut coeffs = vec![secret]; // a_0 = secret
        for _ in 1..threshold {
            coeffs.push(F::rand(rng));
        }
        
        // Evaluate polynomial at points 1, 2, ..., num_parties
        (1..=num_parties)
            .map(|i| {
                let x = F::from(i as u64);
                let mut y = F::zero();
                let mut x_power = F::one();
                
                for coeff in &coeffs {
                    y += *coeff * x_power;
                    x_power *= x;
                }
                
                ShamirShare { index: i, value: y }
            })
            .collect()
    }
    
    fn reconstruct_secret(shares: &[Self::Share]) -> Result<F, SecretSharingError> {
        if shares.is_empty() {
            return Err(SecretSharingError::InsufficientShares);
        }
        
        // Lagrange interpolation at x = 0
        let mut result = F::zero();
        
        for (i, share_i) in shares.iter().enumerate() {
            let mut numerator = F::one();
            let mut denominator = F::one();
            
            for (j, share_j) in shares.iter().enumerate() {
                if i != j {
                    let xi = F::from(share_i.index as u64);
                    let xj = F::from(share_j.index as u64);
                    
                    numerator *= -xj; // (0 - xj)
                    denominator *= xi - xj;
                }
            }
            
            if denominator.is_zero() {
                return Err(SecretSharingError::InvalidShares);
            }
            
            result += share_i.value * numerator * denominator.inverse().unwrap();
        }
        
        Ok(result)
    }
    
    fn verify_share(_share: &Self::Share, _secret_key: &Self::SecretKey) -> bool {
        // Shamir's scheme doesn't require verification with secret key
        true
    }
    
    fn add_shares(left: &Self::Share, right: &Self::Share) -> Result<Self::Share, SecretSharingError> {
        // Addition is local for Shamir's scheme
        Ok(ShamirShare {
            index: left.index, // Both shares should have the same index
            value: left.value + right.value,
        })
    }
    
    fn mul_shares(left: &Self::Share, right: &Self::Share) -> Result<Self::Share, SecretSharingError> {
        // Multiplication requires degree reduction in Shamir's scheme
        // This is a simplified version - in practice needs more complex protocol
        Ok(ShamirShare {
            index: left.index,
            value: left.value * right.value,
        })
    }
    
    fn scalar_mul_share(share: &Self::Share, scalar: F) -> Self::Share {
        ShamirShare {
            index: share.index,
            value: share.value * scalar,
        }
    }
}

/// Additive secret sharing for linear operations
#[derive(Clone)]
pub struct AdditiveSecretSharing<F: Field> {
    _phantom: std::marker::PhantomData<F>,
}

impl<F: Field> AdditiveSecretSharing<F> {
    pub fn new() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

#[derive(Debug, Clone)]
pub struct AdditiveShare<F: Field> {
    pub party_id: usize,
    pub value: F,
}

impl<F: Field> SecretSharing<F> for AdditiveSecretSharing<F> {
    type Share = AdditiveShare<F>;
    type SecretKey = ();
    
    fn share_secret(
        secret: F,
        _threshold: usize,
        num_parties: usize,
        rng: &mut impl Rng,
    ) -> Vec<Self::Share> {
        let mut shares = Vec::with_capacity(num_parties);
        let mut sum = F::zero();
        
        // Generate random shares for all but the last party
        for i in 0..num_parties - 1 {
            let share_value = F::rand(rng);
            sum += share_value;
            shares.push(AdditiveShare {
                party_id: i,
                value: share_value,
            });
        }
        
        // Last share ensures the sum equals the secret
        shares.push(AdditiveShare {
            party_id: num_parties - 1,
            value: secret - sum,
        });
        
        shares
    }
    
    fn reconstruct_secret(shares: &[Self::Share]) -> Result<F, SecretSharingError> {
        if shares.is_empty() {
            return Err(SecretSharingError::InsufficientShares);
        }
        
        Ok(shares.iter().map(|s| s.value).sum())
    }
    
    fn verify_share(_share: &Self::Share, _secret_key: &Self::SecretKey) -> bool {
        true
    }
    
    fn add_shares(left: &Self::Share, right: &Self::Share) -> Result<Self::Share, SecretSharingError> {
        // Addition is local for additive sharing
        Ok(AdditiveShare {
            party_id: left.party_id,
            value: left.value + right.value,
        })
    }
    
    fn mul_shares(_left: &Self::Share, _right: &Self::Share) -> Result<Self::Share, SecretSharingError> {
        // Multiplication is not directly supported in additive sharing
        // Would require conversion to another scheme or special protocols
        Err(SecretSharingError::ReconstructionFailed)
    }
    
    fn scalar_mul_share(share: &Self::Share, scalar: F) -> Self::Share {
        AdditiveShare {
            party_id: share.party_id,
            value: share.value * scalar,
        }
    }
}

/// Secret sharing error types
#[derive(Debug, Clone)]
pub enum SecretSharingError {
    InsufficientShares,
    InvalidShares,
    ReconstructionFailed,
}

impl std::fmt::Display for SecretSharingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SecretSharingError::InsufficientShares => write!(f, "Insufficient shares for reconstruction"),
            SecretSharingError::InvalidShares => write!(f, "Invalid shares provided"),
            SecretSharingError::ReconstructionFailed => write!(f, "Secret reconstruction failed"),
        }
    }
}

impl std::error::Error for SecretSharingError {}
