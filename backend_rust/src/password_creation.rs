use std::collections::hash_map::DefaultHasher;
use ark_ec::{ProjectiveCurve};
use ark_ff::{PrimeField};
use ark_bls12_381::{G1Projective as G, Fr as ScalarField};
use ark_std::{UniformRand};
use std::hash::{Hash, Hasher};
use ark_bls12_381::g1::Parameters;
use ark_ec::short_weierstrass_jacobian::GroupProjective;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{Rng};

#[derive(Copy, Clone)]
pub struct HashData {
    pub wallet_address: u64,
    pub first_pass_half: u64,
    pub second_pass_half: u64,
    pub public_hash: Option<u128>, // This is the hash that will be stored on-chain
}

impl HashData {
    pub fn new(wallet_address: &str, password: &str) -> Self {
        let mut hasher1 = DefaultHasher::new();
        wallet_address.hash(&mut hasher1);
        let wallet_address = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        password.hash(&mut hasher2);
        let password = hasher2.finish();

        let (first_pass_half, second_pass_half) = HashData::separate_password(&password);

        Self {
            wallet_address,
            first_pass_half,
            second_pass_half,
            public_hash: None,
        }
    }

    pub fn new_with_hash(wallet_address: &str, password: &str, public_hash: &str) -> Self {
        let mut data = HashData::new(wallet_address, password);
        let public_hash = u128::from_str_radix(public_hash, 10).unwrap();
        data.public_hash = Some(public_hash);
        data
    }

    pub fn calculate_hash(&mut self) -> u128 {
        let wallet_address = BigUint::from_bytes_be(&self.wallet_address.to_be_bytes());
        let first_half = BigUint::from_bytes_be(&self.first_pass_half.to_be_bytes());
        let second_half = BigUint::from_bytes_be(&self.second_pass_half.to_be_bytes());

        let hash = &wallet_address * &first_half;
        let hash = hash - &second_half;

        let hash = hash.to_u128().unwrap();

        self.public_hash = Some(hash);

        self.public_hash.unwrap()
    }

    // This function can be possible to use, but more research is required
    #[deprecated]
    pub fn calculate_hash_ec<R: Rng + ?Sized> (&self, rng: &mut R) -> GroupProjective<Parameters> {
        // let mut rng = ark_std::rand::thread_rng();
        let a = G::rand(rng);

        let wallet_address = ScalarField::from_be_bytes_mod_order(&self.wallet_address.to_be_bytes());
        let first_half = ScalarField::from_be_bytes_mod_order(&self.first_pass_half.to_be_bytes());
        let second_half = ScalarField::from_be_bytes_mod_order(&self.second_pass_half.to_be_bytes());

        let new_var = wallet_address * first_half;

        let point1 = a.mul(new_var.into_repr());
        let point2 = a.mul(second_half.into_repr());
        let point_as_pub_hash = point1 - point2;
        println!("\n --- This is the hash point: {} --- \n", point_as_pub_hash);
        point_as_pub_hash
    }

    pub fn separate_password(password: &u64) -> (u64, u64) {
        // If the password part fails for some reason,
        // then it's more likely due to improper separation of the first and the second parts.
        // Try to create the first and the second parts from the initial value,
        // and only then hash them and convert to be_bytes
        let password_be_bytes = password.to_be_bytes();
        // let (first, second) = password_be_bytes.split_at(password_be_bytes.len() / 2);
        let mut first_half = Vec::new();
        let mut second_half = Vec::new();

        let mut byte_nr = 0;
        for i in password_be_bytes {
            if byte_nr < password_be_bytes.len() {
                first_half.push(i);
            } else {
                second_half.push(i)
            }
            byte_nr += 1;
        }

        let mut hasher1 = DefaultHasher::new();
        first_half.hash(&mut hasher1);
        let first_half = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        second_half.hash(&mut hasher2);
        let second_half = hasher2.finish();

        (first_half, second_half)
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    fn setup() -> HashData {
        let mut data = HashData::new("0xBC3f9D5D958CBaC6d3b3d38F7320B7d1719Ee2eF", "Difficult_Password");
        data.calculate_hash();
        data
    }

    #[test]
    fn test_public_hash_computation() {
        // Tests that the public hash is calculated accurately
        let data = setup();

        let wallet_address = BigUint::from_bytes_be(&data.wallet_address.to_be_bytes());
        let first_half = BigUint::from_bytes_be(&data.first_pass_half.to_be_bytes());
        let second_half = BigUint::from_bytes_be(&data.second_pass_half.to_be_bytes());

        // Check that the calculated hash value satisfies the proper logic

        let hash = data.public_hash.unwrap();
        let hash = BigUint::from_bytes_be(&hash.to_be_bytes());
        let n1 = &wallet_address * &first_half;
        let n2 = &second_half + &hash;

        assert!(n1.eq(&n2));

        // Check that the incorrect logic leads to final numbers not being equal

        let n1 = &wallet_address + &first_half;
        let n2 = &second_half * &hash;

        assert!(!n1.eq(&n2));

        // Check that the incorrect hash value fails when applying the correct logic from the first example

        let rand: u64 = rand::random();
        let hash = BigUint::from(rand);

        let n1 = &wallet_address * &first_half;
        let n2 = &second_half + &hash;

        assert!(!n1.eq(&n2));
    }

    #[test]
    fn test_ec_correctness() {
        // Tests that correctly calculated values: wallet_address, first_half, second_half and public_hash
        // lead to equal Elliptic Curve points computation
        let data = setup();

        let mut rng = rand::thread_rng();
        let a = G::rand(&mut rng);

        let wallet_address = ScalarField::from_be_bytes_mod_order(&data.wallet_address.to_be_bytes());
        let first_half = ScalarField::from_be_bytes_mod_order(&data.first_pass_half.to_be_bytes());
        let second_half = ScalarField::from_be_bytes_mod_order(&data.second_pass_half.to_be_bytes());
        let hash = ScalarField::from_be_bytes_mod_order(&data.public_hash.unwrap().to_be_bytes());

        let n1 = wallet_address * first_half;
        let n2 = second_half + hash;

        let point1 = a.mul(n1.into_repr());
        let point2 = a.mul(n2.into_repr());

        assert!(point1.eq(&point2));

        // Check that the incorrect values will not lead to equal points
        // To check this we generate an incorrect hash (public_hash)

        let wallet_address = ScalarField::from_be_bytes_mod_order(&data.wallet_address.to_be_bytes());
        let first_half = ScalarField::from_be_bytes_mod_order(&data.first_pass_half.to_be_bytes());
        let second_half = ScalarField::from_be_bytes_mod_order(&data.second_pass_half.to_be_bytes());
        let rand: u64 = rand::random();
        let hash = ScalarField::from_be_bytes_mod_order(&BigUint::from(rand).to_bytes_be());

        let n1 = wallet_address * first_half;
        let n2 = second_half + hash;

        let point1 = a.mul(n1.into_repr());
        let point2 = a.mul(n2.into_repr());

        assert!(!point1.eq(&point2));
    }

    #[test]
    fn test_new_with_hash() {
        let mut data = setup();
        let first_hash = data.calculate_hash();
        let hash = &first_hash.to_string();
        let new_data = HashData::new_with_hash(&data.wallet_address.to_string(), "Difficult_Password", &hash);
        let second_hash = new_data.public_hash.unwrap();

        assert!(first_hash.eq(&second_hash))

    }
}