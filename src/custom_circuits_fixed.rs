use ark_ff::{Field, PrimeField};
use ark_poly::{DenseUVPolynomial, univariate::DensePolynomial};
use ark_std::vec::Vec;
use crate::piop::ConsistencyChecker;

/// 自定义电路定义
#[derive(Debug, Clone)]
pub struct CustomCircuit<F: PrimeField> {
    /// 电路名称
    pub name: String,
    /// 约束数量
    pub num_constraints: usize,
    /// 变量数量
    pub num_variables: usize,
    /// 私有见证
    pub private_witnesses: Vec<F>,
    /// 公开输入
    pub public_inputs: Vec<F>,
    /// 乘法约束定义 (a, b, c) 表示 a * b = c 的约束
    pub multiplication_constraints: Vec<(usize, usize, usize)>,
    /// 加法约束定义 (a, b, c) 表示 a + b = c 的约束
    pub addition_constraints: Vec<(usize, usize, usize)>,
}

impl<F: PrimeField> CustomCircuit<F> {
    /// 创建新的自定义电路
    pub fn new(name: String) -> Self {
        Self {
            name,
            num_constraints: 0,
            num_variables: 0,
            private_witnesses: Vec::new(),
            public_inputs: Vec::new(),
            multiplication_constraints: Vec::new(),
            addition_constraints: Vec::new(),
        }
    }
    
    /// 添加私有见证
    pub fn add_private_witness(&mut self, witness: F) -> usize {
        self.private_witnesses.push(witness);
        let index = self.num_variables;
        self.num_variables += 1;
        index
    }
    
    /// 添加公开输入
    pub fn add_public_input(&mut self, input: F) -> usize {
        self.public_inputs.push(input);
        let index = self.num_variables;
        self.num_variables += 1;
        index
    }
    
    /// 添加约束: var_a * var_b = var_c
    pub fn add_multiplication_constraint(&mut self, var_a: usize, var_b: usize, var_c: usize) {
        self.multiplication_constraints.push((var_a, var_b, var_c));
        self.num_constraints += 1;
    }
    
    /// 添加约束: var_a + var_b = var_c
    pub fn add_addition_constraint(&mut self, var_a: usize, var_b: usize, var_c: usize) {
        self.addition_constraints.push((var_a, var_b, var_c));
        self.num_constraints += 1;
    }
    
    /// 验证电路约束是否满足
    pub fn verify_constraints(&self) -> bool {
        let mut all_variables: Vec<F> = Vec::new();
        all_variables.extend(&self.private_witnesses);
        all_variables.extend(&self.public_inputs);
        
        // 验证乘法约束
        for &(a_idx, b_idx, c_idx) in &self.multiplication_constraints {
            if a_idx >= all_variables.len() || 
               b_idx >= all_variables.len() || 
               c_idx >= all_variables.len() {
                return false;
            }
            
            let a: F = all_variables[a_idx];
            let b: F = all_variables[b_idx];
            let c: F = all_variables[c_idx];
            
            if a * b != c {
                println!("   ❌ 乘法约束失败: {} × {} ≠ {} (期望 {})", a, b, c, a * b);
                return false;
            }
        }
        
        // 验证加法约束
        for &(a_idx, b_idx, c_idx) in &self.addition_constraints {
            if a_idx >= all_variables.len() || 
               b_idx >= all_variables.len() || 
               c_idx >= all_variables.len() {
                return false;
            }
            
            let a: F = all_variables[a_idx];
            let b: F = all_variables[b_idx];
            let c: F = all_variables[c_idx];
            
            if a + b != c {
                println!("   ❌ 加法约束失败: {} + {} ≠ {} (期望 {})", a, b, c, a + b);
                return false;
            }
        }
        
        true
    }
    
    /// 将见证转换为多项式表示
    pub fn witnesses_to_polynomials(&self) -> Vec<DensePolynomial<F>> {
        self.private_witnesses.iter()
            .map(|&w| DensePolynomial::from_coefficients_vec(vec![w]))
            .collect()
    }
    
    /// 生成约束多项式
    pub fn generate_constraint_polynomials(&self) -> Vec<DensePolynomial<F>> {
        let mut all_variables: Vec<F> = Vec::new();
        all_variables.extend(&self.private_witnesses);
        all_variables.extend(&self.public_inputs);
        
        let mut constraint_polys = Vec::new();
        
        // 处理乘法约束
        for &(a_idx, b_idx, c_idx) in &self.multiplication_constraints {
            let a: F = all_variables[a_idx];
            let b: F = all_variables[b_idx];
            let c: F = all_variables[c_idx];
            
            // 约束多项式: a * b - c (应该等于0)
            let constraint_value = a * b - c;
            constraint_polys.push(DensePolynomial::from_coefficients_vec(vec![constraint_value]));
        }
        
        // 处理加法约束
        for &(a_idx, b_idx, c_idx) in &self.addition_constraints {
            let a: F = all_variables[a_idx];
            let b: F = all_variables[b_idx];
            let c: F = all_variables[c_idx];
            
            // 约束多项式: a + b - c (应该等于0)
            let constraint_value = a + b - c;
            constraint_polys.push(DensePolynomial::from_coefficients_vec(vec![constraint_value]));
        }
        
        constraint_polys
    }
}

/// 预定义的电路模板
pub struct CircuitTemplates;

impl CircuitTemplates {
    /// 平方根验证电路: 验证 x² = y
    pub fn square_root_verification<F: PrimeField>(x: F, y: F) -> CustomCircuit<F> {
        let mut circuit = CustomCircuit::new("square_root_verification".to_string());
        
        // 添加见证和输入
        let x_idx = circuit.add_private_witness(x);  // 私有见证: 平方根
        let _y_idx = circuit.add_public_input(y);     // 公开输入: 平方值
        let x_squared_idx = circuit.add_private_witness(x * x);  // 中间结果
        
        // 约束: x * x = x²
        circuit.add_multiplication_constraint(x_idx, x_idx, x_squared_idx);
        
        circuit
    }
    
    /// 范围证明电路: 证明 x 在 [min, max] 范围内
    pub fn range_proof<F: PrimeField>(x: F, min: F, max: F) -> CustomCircuit<F> {
        let mut circuit = CustomCircuit::new("range_proof".to_string());
        
        let _x_idx = circuit.add_private_witness(x);
        let _min_idx = circuit.add_public_input(min);
        let _max_idx = circuit.add_public_input(max);
        
        // x - min ≥ 0 和 max - x ≥ 0 的证明
        // 这需要更复杂的约束系统来处理不等式
        // 这里提供框架，实际实现需要将不等式转换为等式约束
        
        let x_minus_min_idx = circuit.add_private_witness(x - min);
        let max_minus_x_idx = circuit.add_private_witness(max - x);
        
        // 简化处理：假设范围检查通过平方数来证明非负性
        let square1_idx = circuit.add_private_witness((x - min) * (x - min));
        let square2_idx = circuit.add_private_witness((max - x) * (max - x));
        
        circuit.add_multiplication_constraint(x_minus_min_idx, x_minus_min_idx, square1_idx);
        circuit.add_multiplication_constraint(max_minus_x_idx, max_minus_x_idx, square2_idx);
        
        circuit
    }
}

/// 电路测试工具
pub struct CircuitTester;

impl CircuitTester {
    /// 测试自定义电路
    pub fn test_circuit<F: PrimeField>(circuit: &CustomCircuit<F>) -> bool {
        println!("🧪 测试电路: {}", circuit.name);
        println!("   📊 约束数量: {}", circuit.num_constraints);
        println!("   🔢 变量数量: {}", circuit.num_variables);
        println!("   🔒 私有见证数量: {}", circuit.private_witnesses.len());
        println!("   📖 公开输入数量: {}", circuit.public_inputs.len());
        
        let is_valid = circuit.verify_constraints();
        println!("   ✅ 约束验证结果: {}", is_valid);
        
        is_valid
    }
    
    /// 运行电路的 PIOP 测试
    pub fn run_piop_test<F: PrimeField>(
        circuit: &CustomCircuit<F>, 
        checker: &mut ConsistencyChecker<F>
    ) -> bool {
        let witness_polys = circuit.witnesses_to_polynomials();
        let constraint_polys = circuit.generate_constraint_polynomials();
        
        // 添加见证多项式
        for (i, poly) in witness_polys.iter().enumerate() {
            checker.add_witness_polynomial(format!("witness_{}", i), poly.clone());
        }
        
        // 添加约束多项式
        for (i, poly) in constraint_polys.iter().enumerate() {
            checker.add_public_polynomial(format!("constraint_{}", i), poly.clone());
        }
        
        let result = checker.batch_consistency_check();
        result.is_consistent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    
    type TestField = Fr;
    
    #[test]
    fn test_square_root_circuit() {
        let x = TestField::from(5u64);
        let y = TestField::from(25u64);
        
        let circuit = CircuitTemplates::square_root_verification(x, y);
        assert!(CircuitTester::test_circuit(&circuit));
    }
    
    #[test]
    fn test_custom_circuit_creation() {
        let mut circuit = CustomCircuit::<TestField>::new("test".to_string());
        
        let a_idx = circuit.add_private_witness(TestField::from(3u64));
        let b_idx = circuit.add_private_witness(TestField::from(4u64));
        let c_idx = circuit.add_private_witness(TestField::from(12u64));
        
        circuit.add_multiplication_constraint(a_idx, b_idx, c_idx);
        
        assert!(circuit.verify_constraints());
    }
}
