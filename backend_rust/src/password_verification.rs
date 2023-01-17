use ark_crypto_primitives::crh::{
    CRH,
    CRHGadget,
    pedersen};
use ark_bls12_381::{Fr as ScalarField};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective as JubJub};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError};
use ark_ff::{BigInteger, PrimeField};
use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use ark_relations::r1cs::{
    ConstraintLayer, TracingMode::OnlyConstraints,
};
use tracing_subscriber::layer::SubscriberExt;
use super::HashData;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Window;

impl pedersen::Window for Window {
    const WINDOW_SIZE: usize = 128;
    const NUM_WINDOWS: usize = 8;
}

pub type ConstraintF = ark_bls12_381::Fr;
pub type MyCRH = pedersen::CRH<JubJub, Window>;
pub type MyCRHGadget = pedersen::constraints::CRHGadget<JubJub, EdwardsVar, Window>;
pub type DataParams = <MyCRH as CRH>::Parameters;
pub type Commitment = <MyCRH as CRH>::Output;
pub type CommitmentVar = <MyCRHGadget as CRHGadget<MyCRH, ConstraintF>>::OutputVar;

pub struct HashDataVar {
    data: HashData,
    params: DataParams,
    public_hash_commitment: Option<Commitment>,
}

impl ConstraintSynthesizer<ConstraintF> for HashDataVar {
    #[tracing::instrument(target = "r1cs", skip(self, cs))]
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {

        let wallet_address_var_scalar = ScalarField::from_be_bytes_mod_order(&self.data.wallet_address.to_be_bytes());
        let first_half_var_scalar = ScalarField::from_be_bytes_mod_order(&self.data.first_pass_half.to_be_bytes());
        let second_half_var_scalar = ScalarField::from_be_bytes_mod_order(&self.data.second_pass_half.to_be_bytes());

        let final_scalar = wallet_address_var_scalar * first_half_var_scalar - second_half_var_scalar;

        let final_commitment = HashDataVar::scalar_to_commitment(final_scalar, &self.params);

        let pub_hash_commitment_var = CommitmentVar::new_input(
            ark_relations::ns!(cs, "The commitment of the of the public hash"),
            || { Ok(self.public_hash_commitment.unwrap()) },
        )?;

        let final_commitment_var = CommitmentVar::new_witness(
            ark_relations::ns!(cs, "The commitment of wallet address, first half of the password and the second half of the password"),
            || { Ok(final_commitment) },
        )?;

        pub_hash_commitment_var.enforce_equal(&final_commitment_var)?;

        Ok(())
    }
}

impl HashDataVar {
    pub fn new(data: HashData) -> Self {
        let mut rng = rand::thread_rng();
        let params = MyCRH::setup(&mut rng).unwrap();

        let public_hash_commitment = HashDataVar::be_bytes_to_commitment(&data.public_hash.unwrap().to_be_bytes(), &params);

        let public_hash_commitment = Some(public_hash_commitment);

        Self {
            data,
            params,
            public_hash_commitment,
        }
    }

    pub fn be_bytes_to_commitment(be_bytes: &[u8], params: &DataParams) -> Commitment {
        // Firstly, turning the be_bytes into ScalarField, so that we could operate
        // with this algebraic value in the arkworks ecosystem
        let scalar = ScalarField::from_be_bytes_mod_order(be_bytes);

        // Then turn the scalar value to BigInteger, so that we could convert it to be_bytes.
        // These extra steps are done, because the main computations are done with the use of ScalarField.
        // Check generate_constraints() function to see mentioned "computations".
        HashDataVar::scalar_to_commitment(scalar, &params)
    }

    pub fn scalar_to_commitment(scalar: ScalarField, params: &DataParams) -> Commitment {
        let big_int = scalar.into_repr();

        // Finally, calculating the commitment
        let public_hash_commitment = MyCRH::evaluate(params, &big_int.to_bytes_be()).unwrap();

        public_hash_commitment
    }

    fn default() -> HashData {
        let wallet_address = "0xBC3f9D5D958CBaC6d3b3d38F7320B7d1719Ee2eF";
        let password = "Difficult_Password";

        let mut data = HashData::new(wallet_address, password);
        data.calculate_hash();
        data
    }

    // pub fn prove_with_zkp<CG: ConstraintSynthesizer<ConstraintF>>(data: CG) -> bool {
    pub fn prove_with_zkp(data: HashDataVar) -> Result<bool, SynthesisError> {
        if let None = data.public_hash_commitment {
            return Err(SynthesisError::AssignmentMissing);
        }

        // This is an improper approach.
        // It was done to capture the Err in advance,
        // as otherwise this Err would be thrown in Groth16::prove()
        // and would be impossible to capture, arkworks do not return the Err in their code.
        // The issue is in arkworks' prover.rs:66
        let data_copy = HashDataVar::new(data.data.clone());

        if let false = HashDataVar::test_cs(data_copy) {
            return Err(SynthesisError::Unsatisfiable);
        };
        // The end of an improper implementation

        // Use a circuit just to generate the circuit
        // This circuit is used to tell the SNARK the setup of the circuit that we are going to verify.
        // Thus SNARK generates proving key (pk) and  verifying key (vk) and "connects" them (meaning
        // that only this vk can verify this pk and only this pk can be verifier by this vk)
        let circuit_defining_cs = HashDataVar::new(HashDataVar::default());

        let mut rng = rand::thread_rng();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit_defining_cs, &mut rng)?;

        // Use the same circuit but with different inputs to verify against
        // This test checks that the SNARK passes on the provided input

        let public_input= [
            data.public_hash_commitment.unwrap().x,
            data.public_hash_commitment.unwrap().y
        ];

        let proof = Groth16::prove(&pk, data, &mut rng)?;
        let valid_proof = Groth16::verify(&vk, &public_input, &proof)?;

        Ok(valid_proof)
    }

    // Should be used only in tests.
    // See prove_with_zkp() comments for more details.
    fn test_cs(data: HashDataVar) -> bool {
        let mut layer = ConstraintLayer::default();
        layer.mode = OnlyConstraints;
        let subscriber = tracing_subscriber::Registry::default().with(layer);
        let _guard = tracing::subscriber::set_default(subscriber);
        let cs = ConstraintSystem::new_ref();
        data.generate_constraints(cs.clone()).unwrap();
        let result = cs.is_satisfied().unwrap();
        if !result {
            println!("{:?}", cs.which_is_unsatisfied());
        }
        result
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use ark_std::UniformRand;

    fn seeded_setup(wallet_address: &str, password: &str) -> HashData {
        let mut data = HashData::new(wallet_address, password);
        data.calculate_hash();
        data
    }

    fn setup() -> HashData {
        let wallet_address = "0xBC3f9D5D958CBaC6d3b3d38F7320B7d1719Ee2eF";
        let password = "Difficult_Password";
        seeded_setup(wallet_address, password)
    }

    fn test_cs(data: HashDataVar) -> bool {
        let mut layer = ConstraintLayer::default();
        layer.mode = OnlyConstraints;
        let subscriber = tracing_subscriber::Registry::default().with(layer);
        let _guard = tracing::subscriber::set_default(subscriber);
        let cs = ConstraintSystem::new_ref();
        data.generate_constraints(cs.clone()).unwrap();
        let result = cs.is_satisfied().unwrap();
        if !result {
            println!("{:?}", cs.which_is_unsatisfied());
        }
        result
    }

    #[test]
    fn cs_validity_test() {
        let data = setup();

        let data = HashDataVar::new(data);

        assert!(test_cs(data));

        let data = setup();

        let mut data = HashDataVar::new(data);

        let mut rng = rand::thread_rng();
        let rand = u128::rand(&mut rng);

        let commitment = HashDataVar::be_bytes_to_commitment(&rand.to_be_bytes(), &data.params);
        data.public_hash_commitment = Some(commitment);

        assert!(!test_cs(data));
    }

    #[test]
    fn snark_verification() {

        // This test checks that the SNARK passes on the provided input
        // It proves the completeness of the Zero-Knowledge Protocol
        let data_to_prove = HashDataVar::new(seeded_setup("0xC41aD56cb5b2D58292f30caF35aba86a86935675", "$Ver1_strong!password$"));

        assert!(HashDataVar::prove_with_zkp(data_to_prove).unwrap());

        // Change the first_pass_half and second_pass_half to fail the test
        // This proves the soundness of the Zero-Knowledge Protocol
        let mut data_to_prove = HashDataVar::new(seeded_setup("0x3648979251569849bC49605593DE48E2a44cf72d", "notstrongpasswordatall"));

        let wrong_password: u64 = rand::random();
        let (first_pass_half, second_pass_half) = HashData::separate_password(&wrong_password);
        data_to_prove.data.first_pass_half = first_pass_half;
        data_to_prove.data.second_pass_half = second_pass_half;

        // assert!(!test_cs(data_to_prove));

        // The assertion below should work, but arkworks do not return the Err in their code,
        // so instead of capturing it, the program will throw an error, when trying to
        // run the test.
        // That's why the assertion above is used, although the one below is more correct.

        assert!(HashDataVar::prove_with_zkp(data_to_prove).is_err());
    }

    #[test]
    fn snark_verification_with_known_hash() {
        let data = HashData::new_with_hash("0xcfb75c0e3c6b4218fd02973934dee896fa484429", "password", "144447681626574366700357989607795100402");

        let data_var = HashDataVar::new(data);
        assert!(HashDataVar::prove_with_zkp(data_var).unwrap());
    }

// #[test]
// fn test_native_equality() {
//     let rng = &mut test_rng();
//     let cs = ConstraintSystem::<Fr>::new_ref();
//     let data = HashDataVar::new(666, 666);
//
//     let (input, input_var) = generate_u8_input(cs.clone(), 128, rng);
//
//     let parameters = <TestCRH as CRH>::setup(rng).unwrap();
//     let primitive_result = <TestCRH as CRH>::evaluate(&parameters, &input).unwrap();
//
//     let parameters_var = pedersen::constraints::CRHParametersVar::new_constant(
//         ark_relations::ns!(cs, "CRH Parameters"),
//         &parameters,
//     )
//         .unwrap();
//
//     let result_var =
//         <TestCRHGadget as CRHGadget<_, _>>::evaluate(&parameters_var, &input_var).unwrap();
//
//     let primitive_result = primitive_result;
//     assert_eq!(primitive_result, result_var.value().unwrap());
//     assert!(cs.is_satisfied().unwrap());
// }
}
