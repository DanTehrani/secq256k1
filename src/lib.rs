pub mod affine;
pub mod field;
pub mod scalar;
pub mod wide64;

pub use affine::AffinePoint;
use affine::AffinePointCore;
pub use primeorder::elliptic_curve;
pub use primeorder::elliptic_curve::bigint::U256;

use field::field_secp::FieldElement;
use primeorder::elliptic_curve::{AffineArithmetic, Curve, ProjectiveArithmetic, ScalarArithmetic};
use primeorder::{PrimeCurve, PrimeCurveParams};
pub use scalar::Scalar;

pub type EncodedPoint = primeorder::elliptic_curve::sec1::EncodedPoint<Secq256K1>;
pub type FieldBytes = primeorder::elliptic_curve::FieldBytes<Secq256K1>;
pub type ProjectivePoint = primeorder::ProjectivePoint<Secq256K1>;

pub const ORDER: U256 =
    // U256::from_be_hex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
    U256::from_be_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Secq256K1;

impl Curve for Secq256K1 {
    type UInt = U256;

    const ORDER: U256 =
        //    U256::from_be_hex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
        U256::from_be_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");
}

impl PrimeCurveParams for Secq256K1 {
    type FieldElement = FieldElement;

    const ZERO: FieldElement = FieldElement::ZERO;
    const ONE: FieldElement = FieldElement::ONE;

    const EQUATION_A: FieldElement = FieldElement::ZERO;

    const EQUATION_B: FieldElement = FieldElement::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000007",
    );

    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_be_hex(
            //"76c39f5585cb160eb6b06c87a2ce32e23134e45a097781a6a24288e37702eda6",
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        ),
        FieldElement::from_be_hex(
            //"3ffc646c7b2918b5dc2d265a8e82a7f7d18983d26e8dc055a4120ddad952677f",
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        ),
    );
}

impl PrimeCurve for Secq256K1 {}

impl AffineArithmetic for Secq256K1 {
    type AffinePoint = AffinePointCore;
}

impl ProjectiveArithmetic for Secq256K1 {
    type ProjectivePoint = ProjectivePoint;
}

impl ScalarArithmetic for Secq256K1 {
    type Scalar = Scalar;
}
