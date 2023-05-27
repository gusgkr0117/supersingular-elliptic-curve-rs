use supersingular_elliptic_curve::matrix::Matrix;
use num::bigint::{BigUint, RandomBits};
use rand::Rng;

#[test]
fn default_test() {
    let mat1 : Matrix<BigUint> = Matrix::new(3, 4);
    let mat2 : Matrix<BigUint> = Matrix::new(3, 4);

    let mat3:Matrix<BigUint> = mat1.clone() + mat2.clone();
    let big_zero = BigUint::new(vec![0x0]);

    assert_eq!(mat3.element[0], big_zero);

    let mut rng = rand::thread_rng();
    let a: BigUint = rng.sample(RandomBits::new(256));
    let b: BigUint = rng.sample(RandomBits::new(256));
    let c:BigUint = a.clone() % b.clone();

    println!("{:?}", c);
    println!("{:?}", a);
    println!("{:?}", b);
}