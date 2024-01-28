#[non_exhaustive]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug, Hash)]
pub enum InstanceType {
    A12xlarge,

    A14xlarge,

    A1Large,

    A1Medium,

    A1Metal,

    A1Xlarge,

    C1Medium,

    C1Xlarge,

    C32xlarge,

    C34xlarge,

    C38xlarge,

    C3Large,

    C3Xlarge,

    C42xlarge,

    C44xlarge,

    C48xlarge,

    C4Large,

    C4Xlarge,

    C512xlarge,

    C518xlarge,

    C524xlarge,

    C52xlarge,

    C54xlarge,

    C59xlarge,

    C5Large,

    C5Metal,

    C5Xlarge,

    C5a12xlarge,

    C5a16xlarge,

    C5a24xlarge,

    C5a2xlarge,

    C5a4xlarge,

    C5a8xlarge,

    C5aLarge,

    C5aXlarge,

    C5ad12xlarge,

    C5ad16xlarge,

    C5ad24xlarge,

    C5ad2xlarge,

    C5ad4xlarge,

    C5ad8xlarge,

    C5adLarge,

    C5adXlarge,

    C5d12xlarge,

    C5d18xlarge,

    C5d24xlarge,

    C5d2xlarge,

    C5d4xlarge,

    C5d9xlarge,

    C5dLarge,

    C5dMetal,

    C5dXlarge,

    C5n18xlarge,

    C5n2xlarge,

    C5n4xlarge,

    C5n9xlarge,

    C5nLarge,

    C5nMetal,

    C5nXlarge,

    C6a12xlarge,

    C6a16xlarge,

    C6a24xlarge,

    C6a2xlarge,

    C6a32xlarge,

    C6a48xlarge,

    C6a4xlarge,

    C6a8xlarge,

    C6aLarge,

    C6aMetal,

    C6aXlarge,

    C6g12xlarge,

    C6g16xlarge,

    C6g2xlarge,

    C6g4xlarge,

    C6g8xlarge,

    C6gLarge,

    C6gMedium,

    C6gMetal,

    C6gXlarge,

    C6gd12xlarge,

    C6gd16xlarge,

    C6gd2xlarge,

    C6gd4xlarge,

    C6gd8xlarge,

    C6gdLarge,

    C6gdMedium,

    C6gdMetal,

    C6gdXlarge,

    C6gn12xlarge,

    C6gn16xlarge,

    C6gn2xlarge,

    C6gn4xlarge,

    C6gn8xlarge,

    C6gnLarge,

    C6gnMedium,

    C6gnXlarge,

    C6i12xlarge,

    C6i16xlarge,

    C6i24xlarge,

    C6i2xlarge,

    C6i32xlarge,

    C6i4xlarge,

    C6i8xlarge,

    C6iLarge,

    C6iMetal,

    C6iXlarge,

    C6id12xlarge,

    C6id16xlarge,

    C6id24xlarge,

    C6id2xlarge,

    C6id32xlarge,

    C6id4xlarge,

    C6id8xlarge,

    C6idLarge,

    C6idMetal,

    C6idXlarge,

    C6in12xlarge,

    C6in16xlarge,

    C6in24xlarge,

    C6in2xlarge,

    C6in32xlarge,

    C6in4xlarge,

    C6in8xlarge,

    C6inLarge,

    C6inMetal,

    C6inXlarge,

    C7a12xlarge,

    C7a16xlarge,

    C7a24xlarge,

    C7a2xlarge,

    C7a32xlarge,

    C7a48xlarge,

    C7a4xlarge,

    C7a8xlarge,

    C7aLarge,

    C7aMedium,

    C7aMetal48xl,

    C7aXlarge,

    C7g12xlarge,

    C7g16xlarge,

    C7g2xlarge,

    C7g4xlarge,

    C7g8xlarge,

    C7gLarge,

    C7gMedium,

    C7gMetal,

    C7gXlarge,

    C7gd12xlarge,

    C7gd16xlarge,

    C7gd2xlarge,

    C7gd4xlarge,

    C7gd8xlarge,

    C7gdLarge,

    C7gdMedium,

    C7gdXlarge,

    C7gn12xlarge,

    C7gn16xlarge,

    C7gn2xlarge,

    C7gn4xlarge,

    C7gn8xlarge,

    C7gnLarge,

    C7gnMedium,

    C7gnXlarge,

    C7i12xlarge,

    C7i16xlarge,

    C7i24xlarge,

    C7i2xlarge,

    C7i48xlarge,

    C7i4xlarge,

    C7i8xlarge,

    C7iLarge,

    C7iXlarge,

    Cc14xlarge,

    Cc28xlarge,

    Cg14xlarge,

    Cr18xlarge,

    D22xlarge,

    D24xlarge,

    D28xlarge,

    D2Xlarge,

    D32xlarge,

    D34xlarge,

    D38xlarge,

    D3Xlarge,

    D3en12xlarge,

    D3en2xlarge,

    D3en4xlarge,

    D3en6xlarge,

    D3en8xlarge,

    D3enXlarge,

    Dl124xlarge,

    Dl2q24xlarge,

    F116xlarge,

    F12xlarge,

    F14xlarge,

    G22xlarge,

    G28xlarge,

    G316xlarge,

    G34xlarge,

    G38xlarge,

    G3sXlarge,

    G4ad16xlarge,

    G4ad2xlarge,

    G4ad4xlarge,

    G4ad8xlarge,

    G4adXlarge,

    G4dn12xlarge,

    G4dn16xlarge,

    G4dn2xlarge,

    G4dn4xlarge,

    G4dn8xlarge,

    G4dnMetal,

    G4dnXlarge,

    G512xlarge,

    G516xlarge,

    G524xlarge,

    G52xlarge,

    G548xlarge,

    G54xlarge,

    G58xlarge,

    G5Xlarge,

    G5g16xlarge,

    G5g2xlarge,

    G5g4xlarge,

    G5g8xlarge,

    G5gMetal,

    G5gXlarge,

    H116xlarge,

    H12xlarge,

    H14xlarge,

    H18xlarge,

    Hi14xlarge,

    Hpc6a48xlarge,

    Hpc6id32xlarge,

    Hpc7a12xlarge,

    Hpc7a24xlarge,

    Hpc7a48xlarge,

    Hpc7a96xlarge,

    Hpc7g16xlarge,

    Hpc7g4xlarge,

    Hpc7g8xlarge,

    Hs18xlarge,

    I22xlarge,

    I24xlarge,

    I28xlarge,

    I2Xlarge,

    I316xlarge,

    I32xlarge,

    I34xlarge,

    I38xlarge,

    I3Large,

    I3Metal,

    I3Xlarge,

    I3en12xlarge,

    I3en24xlarge,

    I3en2xlarge,

    I3en3xlarge,

    I3en6xlarge,

    I3enLarge,

    I3enMetal,

    I3enXlarge,

    I4g16xlarge,

    I4g2xlarge,

    I4g4xlarge,

    I4g8xlarge,

    I4gLarge,

    I4gXlarge,

    I4i16xlarge,

    I4i2xlarge,

    I4i32xlarge,

    I4i4xlarge,

    I4i8xlarge,

    I4iLarge,

    I4iMetal,

    I4iXlarge,

    Im4gn16xlarge,

    Im4gn2xlarge,

    Im4gn4xlarge,

    Im4gn8xlarge,

    Im4gnLarge,

    Im4gnXlarge,

    Inf124xlarge,

    Inf12xlarge,

    Inf16xlarge,

    Inf1Xlarge,

    Inf224xlarge,

    Inf248xlarge,

    Inf28xlarge,

    Inf2Xlarge,

    Is4gen2xlarge,

    Is4gen4xlarge,

    Is4gen8xlarge,

    Is4genLarge,

    Is4genMedium,

    Is4genXlarge,

    M1Large,

    M1Medium,

    M1Small,

    M1Xlarge,

    M22xlarge,

    M24xlarge,

    M2Xlarge,

    M32xlarge,

    M3Large,

    M3Medium,

    M3Xlarge,

    M410xlarge,

    M416xlarge,

    M42xlarge,

    M44xlarge,

    M4Large,

    M4Xlarge,

    M512xlarge,

    M516xlarge,

    M524xlarge,

    M52xlarge,

    M54xlarge,

    M58xlarge,

    M5Large,

    M5Metal,

    M5Xlarge,

    M5a12xlarge,

    M5a16xlarge,

    M5a24xlarge,

    M5a2xlarge,

    M5a4xlarge,

    M5a8xlarge,

    M5aLarge,

    M5aXlarge,

    M5ad12xlarge,

    M5ad16xlarge,

    M5ad24xlarge,

    M5ad2xlarge,

    M5ad4xlarge,

    M5ad8xlarge,

    M5adLarge,

    M5adXlarge,

    M5d12xlarge,

    M5d16xlarge,

    M5d24xlarge,

    M5d2xlarge,

    M5d4xlarge,

    M5d8xlarge,

    M5dLarge,

    M5dMetal,

    M5dXlarge,

    M5dn12xlarge,

    M5dn16xlarge,

    M5dn24xlarge,

    M5dn2xlarge,

    M5dn4xlarge,

    M5dn8xlarge,

    M5dnLarge,

    M5dnMetal,

    M5dnXlarge,

    M5n12xlarge,

    M5n16xlarge,

    M5n24xlarge,

    M5n2xlarge,

    M5n4xlarge,

    M5n8xlarge,

    M5nLarge,

    M5nMetal,

    M5nXlarge,

    M5zn12xlarge,

    M5zn2xlarge,

    M5zn3xlarge,

    M5zn6xlarge,

    M5znLarge,

    M5znMetal,

    M5znXlarge,

    M6a12xlarge,

    M6a16xlarge,

    M6a24xlarge,

    M6a2xlarge,

    M6a32xlarge,

    M6a48xlarge,

    M6a4xlarge,

    M6a8xlarge,

    M6aLarge,

    M6aMetal,

    M6aXlarge,

    M6g12xlarge,

    M6g16xlarge,

    M6g2xlarge,

    M6g4xlarge,

    M6g8xlarge,

    M6gLarge,

    M6gMedium,

    M6gMetal,

    M6gXlarge,

    M6gd12xlarge,

    M6gd16xlarge,

    M6gd2xlarge,

    M6gd4xlarge,

    M6gd8xlarge,

    M6gdLarge,

    M6gdMedium,

    M6gdMetal,

    M6gdXlarge,

    M6i12xlarge,

    M6i16xlarge,

    M6i24xlarge,

    M6i2xlarge,

    M6i32xlarge,

    M6i4xlarge,

    M6i8xlarge,

    M6iLarge,

    M6iMetal,

    M6iXlarge,

    M6id12xlarge,

    M6id16xlarge,

    M6id24xlarge,

    M6id2xlarge,

    M6id32xlarge,

    M6id4xlarge,

    M6id8xlarge,

    M6idLarge,

    M6idMetal,

    M6idXlarge,

    M6idn12xlarge,

    M6idn16xlarge,

    M6idn24xlarge,

    M6idn2xlarge,

    M6idn32xlarge,

    M6idn4xlarge,

    M6idn8xlarge,

    M6idnLarge,

    M6idnMetal,

    M6idnXlarge,

    M6in12xlarge,

    M6in16xlarge,

    M6in24xlarge,

    M6in2xlarge,

    M6in32xlarge,

    M6in4xlarge,

    M6in8xlarge,

    M6inLarge,

    M6inMetal,

    M6inXlarge,

    M7a12xlarge,

    M7a16xlarge,

    M7a24xlarge,

    M7a2xlarge,

    M7a32xlarge,

    M7a48xlarge,

    M7a4xlarge,

    M7a8xlarge,

    M7aLarge,

    M7aMedium,

    M7aMetal48xl,

    M7aXlarge,

    M7g12xlarge,

    M7g16xlarge,

    M7g2xlarge,

    M7g4xlarge,

    M7g8xlarge,

    M7gLarge,

    M7gMedium,

    M7gMetal,

    M7gXlarge,

    M7gd12xlarge,

    M7gd16xlarge,

    M7gd2xlarge,

    M7gd4xlarge,

    M7gd8xlarge,

    M7gdLarge,

    M7gdMedium,

    M7gdXlarge,

    M7iFlex2xlarge,

    M7iFlex4xlarge,

    M7iFlex8xlarge,

    M7iFlexLarge,

    M7iFlexXlarge,

    M7i12xlarge,

    M7i16xlarge,

    M7i24xlarge,

    M7i2xlarge,

    M7i48xlarge,

    M7i4xlarge,

    M7i8xlarge,

    M7iLarge,

    M7iXlarge,

    Mac1Metal,

    Mac2M2proMetal,

    Mac2Metal,

    P216xlarge,

    P28xlarge,

    P2Xlarge,

    P316xlarge,

    P32xlarge,

    P38xlarge,

    P3dn24xlarge,

    P4d24xlarge,

    P4de24xlarge,

    P548xlarge,

    R32xlarge,

    R34xlarge,

    R38xlarge,

    R3Large,

    R3Xlarge,

    R416xlarge,

    R42xlarge,

    R44xlarge,

    R48xlarge,

    R4Large,

    R4Xlarge,

    R512xlarge,

    R516xlarge,

    R524xlarge,

    R52xlarge,

    R54xlarge,

    R58xlarge,

    R5Large,

    R5Metal,

    R5Xlarge,

    R5a12xlarge,

    R5a16xlarge,

    R5a24xlarge,

    R5a2xlarge,

    R5a4xlarge,

    R5a8xlarge,

    R5aLarge,

    R5aXlarge,

    R5ad12xlarge,

    R5ad16xlarge,

    R5ad24xlarge,

    R5ad2xlarge,

    R5ad4xlarge,

    R5ad8xlarge,

    R5adLarge,

    R5adXlarge,

    R5b12xlarge,

    R5b16xlarge,

    R5b24xlarge,

    R5b2xlarge,

    R5b4xlarge,

    R5b8xlarge,

    R5bLarge,

    R5bMetal,

    R5bXlarge,

    R5d12xlarge,

    R5d16xlarge,

    R5d24xlarge,

    R5d2xlarge,

    R5d4xlarge,

    R5d8xlarge,

    R5dLarge,

    R5dMetal,

    R5dXlarge,

    R5dn12xlarge,

    R5dn16xlarge,

    R5dn24xlarge,

    R5dn2xlarge,

    R5dn4xlarge,

    R5dn8xlarge,

    R5dnLarge,

    R5dnMetal,

    R5dnXlarge,

    R5n12xlarge,

    R5n16xlarge,

    R5n24xlarge,

    R5n2xlarge,

    R5n4xlarge,

    R5n8xlarge,

    R5nLarge,

    R5nMetal,

    R5nXlarge,

    R6a12xlarge,

    R6a16xlarge,

    R6a24xlarge,

    R6a2xlarge,

    R6a32xlarge,

    R6a48xlarge,

    R6a4xlarge,

    R6a8xlarge,

    R6aLarge,

    R6aMetal,

    R6aXlarge,

    R6g12xlarge,

    R6g16xlarge,

    R6g2xlarge,

    R6g4xlarge,

    R6g8xlarge,

    R6gLarge,

    R6gMedium,

    R6gMetal,

    R6gXlarge,

    R6gd12xlarge,

    R6gd16xlarge,

    R6gd2xlarge,

    R6gd4xlarge,

    R6gd8xlarge,

    R6gdLarge,

    R6gdMedium,

    R6gdMetal,

    R6gdXlarge,

    R6i12xlarge,

    R6i16xlarge,

    R6i24xlarge,

    R6i2xlarge,

    R6i32xlarge,

    R6i4xlarge,

    R6i8xlarge,

    R6iLarge,

    R6iMetal,

    R6iXlarge,

    R6id12xlarge,

    R6id16xlarge,

    R6id24xlarge,

    R6id2xlarge,

    R6id32xlarge,

    R6id4xlarge,

    R6id8xlarge,

    R6idLarge,

    R6idMetal,

    R6idXlarge,

    R6idn12xlarge,

    R6idn16xlarge,

    R6idn24xlarge,

    R6idn2xlarge,

    R6idn32xlarge,

    R6idn4xlarge,

    R6idn8xlarge,

    R6idnLarge,

    R6idnMetal,

    R6idnXlarge,

    R6in12xlarge,

    R6in16xlarge,

    R6in24xlarge,

    R6in2xlarge,

    R6in32xlarge,

    R6in4xlarge,

    R6in8xlarge,

    R6inLarge,

    R6inMetal,

    R6inXlarge,

    R7a12xlarge,

    R7a16xlarge,

    R7a24xlarge,

    R7a2xlarge,

    R7a32xlarge,

    R7a48xlarge,

    R7a4xlarge,

    R7a8xlarge,

    R7aLarge,

    R7aMedium,

    R7aMetal48xl,

    R7aXlarge,

    R7g12xlarge,

    R7g16xlarge,

    R7g2xlarge,

    R7g4xlarge,

    R7g8xlarge,

    R7gLarge,

    R7gMedium,

    R7gMetal,

    R7gXlarge,

    R7gd12xlarge,

    R7gd16xlarge,

    R7gd2xlarge,

    R7gd4xlarge,

    R7gd8xlarge,

    R7gdLarge,

    R7gdMedium,

    R7gdXlarge,

    R7i12xlarge,

    R7i16xlarge,

    R7i24xlarge,

    R7i2xlarge,

    R7i48xlarge,

    R7i4xlarge,

    R7i8xlarge,

    R7iLarge,

    R7iXlarge,

    R7iz12xlarge,

    R7iz16xlarge,

    R7iz2xlarge,

    R7iz32xlarge,

    R7iz4xlarge,

    R7iz8xlarge,

    R7izLarge,

    R7izXlarge,

    T1Micro,

    T22xlarge,

    T2Large,

    T2Medium,

    T2Micro,

    T2Nano,

    T2Small,

    T2Xlarge,

    T32xlarge,

    T3Large,

    T3Medium,

    T3Micro,

    T3Nano,

    T3Small,

    T3Xlarge,

    T3a2xlarge,

    T3aLarge,

    T3aMedium,

    T3aMicro,

    T3aNano,

    T3aSmall,

    T3aXlarge,

    T4g2xlarge,

    T4gLarge,

    T4gMedium,

    T4gMicro,

    T4gNano,

    T4gSmall,

    T4gXlarge,

    Trn12xlarge,

    Trn132xlarge,

    Trn1n32xlarge,

    U12tb1112xlarge,

    U12tb1Metal,

    U18tb1112xlarge,

    U18tb1Metal,

    U24tb1112xlarge,

    U24tb1Metal,

    U3tb156xlarge,

    U6tb1112xlarge,

    U6tb156xlarge,

    U6tb1Metal,

    U9tb1112xlarge,

    U9tb1Metal,

    Vt124xlarge,

    Vt13xlarge,

    Vt16xlarge,

    X116xlarge,

    X132xlarge,

    X1e16xlarge,

    X1e2xlarge,

    X1e32xlarge,

    X1e4xlarge,

    X1e8xlarge,

    X1eXlarge,

    X2gd12xlarge,

    X2gd16xlarge,

    X2gd2xlarge,

    X2gd4xlarge,

    X2gd8xlarge,

    X2gdLarge,

    X2gdMedium,

    X2gdMetal,

    X2gdXlarge,

    X2idn16xlarge,

    X2idn24xlarge,

    X2idn32xlarge,

    X2idnMetal,

    X2iedn16xlarge,

    X2iedn24xlarge,

    X2iedn2xlarge,

    X2iedn32xlarge,

    X2iedn4xlarge,

    X2iedn8xlarge,

    X2iednMetal,

    X2iednXlarge,

    X2iezn12xlarge,

    X2iezn2xlarge,

    X2iezn4xlarge,

    X2iezn6xlarge,

    X2iezn8xlarge,

    X2ieznMetal,

    Z1d12xlarge,

    Z1d2xlarge,

    Z1d3xlarge,

    Z1d6xlarge,

    Z1dLarge,

    Z1dMetal,

    Z1dXlarge,
    Unknown,
}
impl ::std::convert::From<&str> for InstanceType {
    fn from(s: &str) -> Self {
        match s {
            "a1.2xlarge" => InstanceType::A12xlarge,
            "a1.4xlarge" => InstanceType::A14xlarge,
            "a1.large" => InstanceType::A1Large,
            "a1.medium" => InstanceType::A1Medium,
            "a1.metal" => InstanceType::A1Metal,
            "a1.xlarge" => InstanceType::A1Xlarge,
            "c1.medium" => InstanceType::C1Medium,
            "c1.xlarge" => InstanceType::C1Xlarge,
            "c3.2xlarge" => InstanceType::C32xlarge,
            "c3.4xlarge" => InstanceType::C34xlarge,
            "c3.8xlarge" => InstanceType::C38xlarge,
            "c3.large" => InstanceType::C3Large,
            "c3.xlarge" => InstanceType::C3Xlarge,
            "c4.2xlarge" => InstanceType::C42xlarge,
            "c4.4xlarge" => InstanceType::C44xlarge,
            "c4.8xlarge" => InstanceType::C48xlarge,
            "c4.large" => InstanceType::C4Large,
            "c4.xlarge" => InstanceType::C4Xlarge,
            "c5.12xlarge" => InstanceType::C512xlarge,
            "c5.18xlarge" => InstanceType::C518xlarge,
            "c5.24xlarge" => InstanceType::C524xlarge,
            "c5.2xlarge" => InstanceType::C52xlarge,
            "c5.4xlarge" => InstanceType::C54xlarge,
            "c5.9xlarge" => InstanceType::C59xlarge,
            "c5.large" => InstanceType::C5Large,
            "c5.metal" => InstanceType::C5Metal,
            "c5.xlarge" => InstanceType::C5Xlarge,
            "c5a.12xlarge" => InstanceType::C5a12xlarge,
            "c5a.16xlarge" => InstanceType::C5a16xlarge,
            "c5a.24xlarge" => InstanceType::C5a24xlarge,
            "c5a.2xlarge" => InstanceType::C5a2xlarge,
            "c5a.4xlarge" => InstanceType::C5a4xlarge,
            "c5a.8xlarge" => InstanceType::C5a8xlarge,
            "c5a.large" => InstanceType::C5aLarge,
            "c5a.xlarge" => InstanceType::C5aXlarge,
            "c5ad.12xlarge" => InstanceType::C5ad12xlarge,
            "c5ad.16xlarge" => InstanceType::C5ad16xlarge,
            "c5ad.24xlarge" => InstanceType::C5ad24xlarge,
            "c5ad.2xlarge" => InstanceType::C5ad2xlarge,
            "c5ad.4xlarge" => InstanceType::C5ad4xlarge,
            "c5ad.8xlarge" => InstanceType::C5ad8xlarge,
            "c5ad.large" => InstanceType::C5adLarge,
            "c5ad.xlarge" => InstanceType::C5adXlarge,
            "c5d.12xlarge" => InstanceType::C5d12xlarge,
            "c5d.18xlarge" => InstanceType::C5d18xlarge,
            "c5d.24xlarge" => InstanceType::C5d24xlarge,
            "c5d.2xlarge" => InstanceType::C5d2xlarge,
            "c5d.4xlarge" => InstanceType::C5d4xlarge,
            "c5d.9xlarge" => InstanceType::C5d9xlarge,
            "c5d.large" => InstanceType::C5dLarge,
            "c5d.metal" => InstanceType::C5dMetal,
            "c5d.xlarge" => InstanceType::C5dXlarge,
            "c5n.18xlarge" => InstanceType::C5n18xlarge,
            "c5n.2xlarge" => InstanceType::C5n2xlarge,
            "c5n.4xlarge" => InstanceType::C5n4xlarge,
            "c5n.9xlarge" => InstanceType::C5n9xlarge,
            "c5n.large" => InstanceType::C5nLarge,
            "c5n.metal" => InstanceType::C5nMetal,
            "c5n.xlarge" => InstanceType::C5nXlarge,
            "c6a.12xlarge" => InstanceType::C6a12xlarge,
            "c6a.16xlarge" => InstanceType::C6a16xlarge,
            "c6a.24xlarge" => InstanceType::C6a24xlarge,
            "c6a.2xlarge" => InstanceType::C6a2xlarge,
            "c6a.32xlarge" => InstanceType::C6a32xlarge,
            "c6a.48xlarge" => InstanceType::C6a48xlarge,
            "c6a.4xlarge" => InstanceType::C6a4xlarge,
            "c6a.8xlarge" => InstanceType::C6a8xlarge,
            "c6a.large" => InstanceType::C6aLarge,
            "c6a.metal" => InstanceType::C6aMetal,
            "c6a.xlarge" => InstanceType::C6aXlarge,
            "c6g.12xlarge" => InstanceType::C6g12xlarge,
            "c6g.16xlarge" => InstanceType::C6g16xlarge,
            "c6g.2xlarge" => InstanceType::C6g2xlarge,
            "c6g.4xlarge" => InstanceType::C6g4xlarge,
            "c6g.8xlarge" => InstanceType::C6g8xlarge,
            "c6g.large" => InstanceType::C6gLarge,
            "c6g.medium" => InstanceType::C6gMedium,
            "c6g.metal" => InstanceType::C6gMetal,
            "c6g.xlarge" => InstanceType::C6gXlarge,
            "c6gd.12xlarge" => InstanceType::C6gd12xlarge,
            "c6gd.16xlarge" => InstanceType::C6gd16xlarge,
            "c6gd.2xlarge" => InstanceType::C6gd2xlarge,
            "c6gd.4xlarge" => InstanceType::C6gd4xlarge,
            "c6gd.8xlarge" => InstanceType::C6gd8xlarge,
            "c6gd.large" => InstanceType::C6gdLarge,
            "c6gd.medium" => InstanceType::C6gdMedium,
            "c6gd.metal" => InstanceType::C6gdMetal,
            "c6gd.xlarge" => InstanceType::C6gdXlarge,
            "c6gn.12xlarge" => InstanceType::C6gn12xlarge,
            "c6gn.16xlarge" => InstanceType::C6gn16xlarge,
            "c6gn.2xlarge" => InstanceType::C6gn2xlarge,
            "c6gn.4xlarge" => InstanceType::C6gn4xlarge,
            "c6gn.8xlarge" => InstanceType::C6gn8xlarge,
            "c6gn.large" => InstanceType::C6gnLarge,
            "c6gn.medium" => InstanceType::C6gnMedium,
            "c6gn.xlarge" => InstanceType::C6gnXlarge,
            "c6i.12xlarge" => InstanceType::C6i12xlarge,
            "c6i.16xlarge" => InstanceType::C6i16xlarge,
            "c6i.24xlarge" => InstanceType::C6i24xlarge,
            "c6i.2xlarge" => InstanceType::C6i2xlarge,
            "c6i.32xlarge" => InstanceType::C6i32xlarge,
            "c6i.4xlarge" => InstanceType::C6i4xlarge,
            "c6i.8xlarge" => InstanceType::C6i8xlarge,
            "c6i.large" => InstanceType::C6iLarge,
            "c6i.metal" => InstanceType::C6iMetal,
            "c6i.xlarge" => InstanceType::C6iXlarge,
            "c6id.12xlarge" => InstanceType::C6id12xlarge,
            "c6id.16xlarge" => InstanceType::C6id16xlarge,
            "c6id.24xlarge" => InstanceType::C6id24xlarge,
            "c6id.2xlarge" => InstanceType::C6id2xlarge,
            "c6id.32xlarge" => InstanceType::C6id32xlarge,
            "c6id.4xlarge" => InstanceType::C6id4xlarge,
            "c6id.8xlarge" => InstanceType::C6id8xlarge,
            "c6id.large" => InstanceType::C6idLarge,
            "c6id.metal" => InstanceType::C6idMetal,
            "c6id.xlarge" => InstanceType::C6idXlarge,
            "c6in.12xlarge" => InstanceType::C6in12xlarge,
            "c6in.16xlarge" => InstanceType::C6in16xlarge,
            "c6in.24xlarge" => InstanceType::C6in24xlarge,
            "c6in.2xlarge" => InstanceType::C6in2xlarge,
            "c6in.32xlarge" => InstanceType::C6in32xlarge,
            "c6in.4xlarge" => InstanceType::C6in4xlarge,
            "c6in.8xlarge" => InstanceType::C6in8xlarge,
            "c6in.large" => InstanceType::C6inLarge,
            "c6in.metal" => InstanceType::C6inMetal,
            "c6in.xlarge" => InstanceType::C6inXlarge,
            "c7a.12xlarge" => InstanceType::C7a12xlarge,
            "c7a.16xlarge" => InstanceType::C7a16xlarge,
            "c7a.24xlarge" => InstanceType::C7a24xlarge,
            "c7a.2xlarge" => InstanceType::C7a2xlarge,
            "c7a.32xlarge" => InstanceType::C7a32xlarge,
            "c7a.48xlarge" => InstanceType::C7a48xlarge,
            "c7a.4xlarge" => InstanceType::C7a4xlarge,
            "c7a.8xlarge" => InstanceType::C7a8xlarge,
            "c7a.large" => InstanceType::C7aLarge,
            "c7a.medium" => InstanceType::C7aMedium,
            "c7a.metal-48xl" => InstanceType::C7aMetal48xl,
            "c7a.xlarge" => InstanceType::C7aXlarge,
            "c7g.12xlarge" => InstanceType::C7g12xlarge,
            "c7g.16xlarge" => InstanceType::C7g16xlarge,
            "c7g.2xlarge" => InstanceType::C7g2xlarge,
            "c7g.4xlarge" => InstanceType::C7g4xlarge,
            "c7g.8xlarge" => InstanceType::C7g8xlarge,
            "c7g.large" => InstanceType::C7gLarge,
            "c7g.medium" => InstanceType::C7gMedium,
            "c7g.metal" => InstanceType::C7gMetal,
            "c7g.xlarge" => InstanceType::C7gXlarge,
            "c7gd.12xlarge" => InstanceType::C7gd12xlarge,
            "c7gd.16xlarge" => InstanceType::C7gd16xlarge,
            "c7gd.2xlarge" => InstanceType::C7gd2xlarge,
            "c7gd.4xlarge" => InstanceType::C7gd4xlarge,
            "c7gd.8xlarge" => InstanceType::C7gd8xlarge,
            "c7gd.large" => InstanceType::C7gdLarge,
            "c7gd.medium" => InstanceType::C7gdMedium,
            "c7gd.xlarge" => InstanceType::C7gdXlarge,
            "c7gn.12xlarge" => InstanceType::C7gn12xlarge,
            "c7gn.16xlarge" => InstanceType::C7gn16xlarge,
            "c7gn.2xlarge" => InstanceType::C7gn2xlarge,
            "c7gn.4xlarge" => InstanceType::C7gn4xlarge,
            "c7gn.8xlarge" => InstanceType::C7gn8xlarge,
            "c7gn.large" => InstanceType::C7gnLarge,
            "c7gn.medium" => InstanceType::C7gnMedium,
            "c7gn.xlarge" => InstanceType::C7gnXlarge,
            "c7i.12xlarge" => InstanceType::C7i12xlarge,
            "c7i.16xlarge" => InstanceType::C7i16xlarge,
            "c7i.24xlarge" => InstanceType::C7i24xlarge,
            "c7i.2xlarge" => InstanceType::C7i2xlarge,
            "c7i.48xlarge" => InstanceType::C7i48xlarge,
            "c7i.4xlarge" => InstanceType::C7i4xlarge,
            "c7i.8xlarge" => InstanceType::C7i8xlarge,
            "c7i.large" => InstanceType::C7iLarge,
            "c7i.xlarge" => InstanceType::C7iXlarge,
            "cc1.4xlarge" => InstanceType::Cc14xlarge,
            "cc2.8xlarge" => InstanceType::Cc28xlarge,
            "cg1.4xlarge" => InstanceType::Cg14xlarge,
            "cr1.8xlarge" => InstanceType::Cr18xlarge,
            "d2.2xlarge" => InstanceType::D22xlarge,
            "d2.4xlarge" => InstanceType::D24xlarge,
            "d2.8xlarge" => InstanceType::D28xlarge,
            "d2.xlarge" => InstanceType::D2Xlarge,
            "d3.2xlarge" => InstanceType::D32xlarge,
            "d3.4xlarge" => InstanceType::D34xlarge,
            "d3.8xlarge" => InstanceType::D38xlarge,
            "d3.xlarge" => InstanceType::D3Xlarge,
            "d3en.12xlarge" => InstanceType::D3en12xlarge,
            "d3en.2xlarge" => InstanceType::D3en2xlarge,
            "d3en.4xlarge" => InstanceType::D3en4xlarge,
            "d3en.6xlarge" => InstanceType::D3en6xlarge,
            "d3en.8xlarge" => InstanceType::D3en8xlarge,
            "d3en.xlarge" => InstanceType::D3enXlarge,
            "dl1.24xlarge" => InstanceType::Dl124xlarge,
            "dl2q.24xlarge" => InstanceType::Dl2q24xlarge,
            "f1.16xlarge" => InstanceType::F116xlarge,
            "f1.2xlarge" => InstanceType::F12xlarge,
            "f1.4xlarge" => InstanceType::F14xlarge,
            "g2.2xlarge" => InstanceType::G22xlarge,
            "g2.8xlarge" => InstanceType::G28xlarge,
            "g3.16xlarge" => InstanceType::G316xlarge,
            "g3.4xlarge" => InstanceType::G34xlarge,
            "g3.8xlarge" => InstanceType::G38xlarge,
            "g3s.xlarge" => InstanceType::G3sXlarge,
            "g4ad.16xlarge" => InstanceType::G4ad16xlarge,
            "g4ad.2xlarge" => InstanceType::G4ad2xlarge,
            "g4ad.4xlarge" => InstanceType::G4ad4xlarge,
            "g4ad.8xlarge" => InstanceType::G4ad8xlarge,
            "g4ad.xlarge" => InstanceType::G4adXlarge,
            "g4dn.12xlarge" => InstanceType::G4dn12xlarge,
            "g4dn.16xlarge" => InstanceType::G4dn16xlarge,
            "g4dn.2xlarge" => InstanceType::G4dn2xlarge,
            "g4dn.4xlarge" => InstanceType::G4dn4xlarge,
            "g4dn.8xlarge" => InstanceType::G4dn8xlarge,
            "g4dn.metal" => InstanceType::G4dnMetal,
            "g4dn.xlarge" => InstanceType::G4dnXlarge,
            "g5.12xlarge" => InstanceType::G512xlarge,
            "g5.16xlarge" => InstanceType::G516xlarge,
            "g5.24xlarge" => InstanceType::G524xlarge,
            "g5.2xlarge" => InstanceType::G52xlarge,
            "g5.48xlarge" => InstanceType::G548xlarge,
            "g5.4xlarge" => InstanceType::G54xlarge,
            "g5.8xlarge" => InstanceType::G58xlarge,
            "g5.xlarge" => InstanceType::G5Xlarge,
            "g5g.16xlarge" => InstanceType::G5g16xlarge,
            "g5g.2xlarge" => InstanceType::G5g2xlarge,
            "g5g.4xlarge" => InstanceType::G5g4xlarge,
            "g5g.8xlarge" => InstanceType::G5g8xlarge,
            "g5g.metal" => InstanceType::G5gMetal,
            "g5g.xlarge" => InstanceType::G5gXlarge,
            "h1.16xlarge" => InstanceType::H116xlarge,
            "h1.2xlarge" => InstanceType::H12xlarge,
            "h1.4xlarge" => InstanceType::H14xlarge,
            "h1.8xlarge" => InstanceType::H18xlarge,
            "hi1.4xlarge" => InstanceType::Hi14xlarge,
            "hpc6a.48xlarge" => InstanceType::Hpc6a48xlarge,
            "hpc6id.32xlarge" => InstanceType::Hpc6id32xlarge,
            "hpc7a.12xlarge" => InstanceType::Hpc7a12xlarge,
            "hpc7a.24xlarge" => InstanceType::Hpc7a24xlarge,
            "hpc7a.48xlarge" => InstanceType::Hpc7a48xlarge,
            "hpc7a.96xlarge" => InstanceType::Hpc7a96xlarge,
            "hpc7g.16xlarge" => InstanceType::Hpc7g16xlarge,
            "hpc7g.4xlarge" => InstanceType::Hpc7g4xlarge,
            "hpc7g.8xlarge" => InstanceType::Hpc7g8xlarge,
            "hs1.8xlarge" => InstanceType::Hs18xlarge,
            "i2.2xlarge" => InstanceType::I22xlarge,
            "i2.4xlarge" => InstanceType::I24xlarge,
            "i2.8xlarge" => InstanceType::I28xlarge,
            "i2.xlarge" => InstanceType::I2Xlarge,
            "i3.16xlarge" => InstanceType::I316xlarge,
            "i3.2xlarge" => InstanceType::I32xlarge,
            "i3.4xlarge" => InstanceType::I34xlarge,
            "i3.8xlarge" => InstanceType::I38xlarge,
            "i3.large" => InstanceType::I3Large,
            "i3.metal" => InstanceType::I3Metal,
            "i3.xlarge" => InstanceType::I3Xlarge,
            "i3en.12xlarge" => InstanceType::I3en12xlarge,
            "i3en.24xlarge" => InstanceType::I3en24xlarge,
            "i3en.2xlarge" => InstanceType::I3en2xlarge,
            "i3en.3xlarge" => InstanceType::I3en3xlarge,
            "i3en.6xlarge" => InstanceType::I3en6xlarge,
            "i3en.large" => InstanceType::I3enLarge,
            "i3en.metal" => InstanceType::I3enMetal,
            "i3en.xlarge" => InstanceType::I3enXlarge,
            "i4g.16xlarge" => InstanceType::I4g16xlarge,
            "i4g.2xlarge" => InstanceType::I4g2xlarge,
            "i4g.4xlarge" => InstanceType::I4g4xlarge,
            "i4g.8xlarge" => InstanceType::I4g8xlarge,
            "i4g.large" => InstanceType::I4gLarge,
            "i4g.xlarge" => InstanceType::I4gXlarge,
            "i4i.16xlarge" => InstanceType::I4i16xlarge,
            "i4i.2xlarge" => InstanceType::I4i2xlarge,
            "i4i.32xlarge" => InstanceType::I4i32xlarge,
            "i4i.4xlarge" => InstanceType::I4i4xlarge,
            "i4i.8xlarge" => InstanceType::I4i8xlarge,
            "i4i.large" => InstanceType::I4iLarge,
            "i4i.metal" => InstanceType::I4iMetal,
            "i4i.xlarge" => InstanceType::I4iXlarge,
            "im4gn.16xlarge" => InstanceType::Im4gn16xlarge,
            "im4gn.2xlarge" => InstanceType::Im4gn2xlarge,
            "im4gn.4xlarge" => InstanceType::Im4gn4xlarge,
            "im4gn.8xlarge" => InstanceType::Im4gn8xlarge,
            "im4gn.large" => InstanceType::Im4gnLarge,
            "im4gn.xlarge" => InstanceType::Im4gnXlarge,
            "inf1.24xlarge" => InstanceType::Inf124xlarge,
            "inf1.2xlarge" => InstanceType::Inf12xlarge,
            "inf1.6xlarge" => InstanceType::Inf16xlarge,
            "inf1.xlarge" => InstanceType::Inf1Xlarge,
            "inf2.24xlarge" => InstanceType::Inf224xlarge,
            "inf2.48xlarge" => InstanceType::Inf248xlarge,
            "inf2.8xlarge" => InstanceType::Inf28xlarge,
            "inf2.xlarge" => InstanceType::Inf2Xlarge,
            "is4gen.2xlarge" => InstanceType::Is4gen2xlarge,
            "is4gen.4xlarge" => InstanceType::Is4gen4xlarge,
            "is4gen.8xlarge" => InstanceType::Is4gen8xlarge,
            "is4gen.large" => InstanceType::Is4genLarge,
            "is4gen.medium" => InstanceType::Is4genMedium,
            "is4gen.xlarge" => InstanceType::Is4genXlarge,
            "m1.large" => InstanceType::M1Large,
            "m1.medium" => InstanceType::M1Medium,
            "m1.small" => InstanceType::M1Small,
            "m1.xlarge" => InstanceType::M1Xlarge,
            "m2.2xlarge" => InstanceType::M22xlarge,
            "m2.4xlarge" => InstanceType::M24xlarge,
            "m2.xlarge" => InstanceType::M2Xlarge,
            "m3.2xlarge" => InstanceType::M32xlarge,
            "m3.large" => InstanceType::M3Large,
            "m3.medium" => InstanceType::M3Medium,
            "m3.xlarge" => InstanceType::M3Xlarge,
            "m4.10xlarge" => InstanceType::M410xlarge,
            "m4.16xlarge" => InstanceType::M416xlarge,
            "m4.2xlarge" => InstanceType::M42xlarge,
            "m4.4xlarge" => InstanceType::M44xlarge,
            "m4.large" => InstanceType::M4Large,
            "m4.xlarge" => InstanceType::M4Xlarge,
            "m5.12xlarge" => InstanceType::M512xlarge,
            "m5.16xlarge" => InstanceType::M516xlarge,
            "m5.24xlarge" => InstanceType::M524xlarge,
            "m5.2xlarge" => InstanceType::M52xlarge,
            "m5.4xlarge" => InstanceType::M54xlarge,
            "m5.8xlarge" => InstanceType::M58xlarge,
            "m5.large" => InstanceType::M5Large,
            "m5.metal" => InstanceType::M5Metal,
            "m5.xlarge" => InstanceType::M5Xlarge,
            "m5a.12xlarge" => InstanceType::M5a12xlarge,
            "m5a.16xlarge" => InstanceType::M5a16xlarge,
            "m5a.24xlarge" => InstanceType::M5a24xlarge,
            "m5a.2xlarge" => InstanceType::M5a2xlarge,
            "m5a.4xlarge" => InstanceType::M5a4xlarge,
            "m5a.8xlarge" => InstanceType::M5a8xlarge,
            "m5a.large" => InstanceType::M5aLarge,
            "m5a.xlarge" => InstanceType::M5aXlarge,
            "m5ad.12xlarge" => InstanceType::M5ad12xlarge,
            "m5ad.16xlarge" => InstanceType::M5ad16xlarge,
            "m5ad.24xlarge" => InstanceType::M5ad24xlarge,
            "m5ad.2xlarge" => InstanceType::M5ad2xlarge,
            "m5ad.4xlarge" => InstanceType::M5ad4xlarge,
            "m5ad.8xlarge" => InstanceType::M5ad8xlarge,
            "m5ad.large" => InstanceType::M5adLarge,
            "m5ad.xlarge" => InstanceType::M5adXlarge,
            "m5d.12xlarge" => InstanceType::M5d12xlarge,
            "m5d.16xlarge" => InstanceType::M5d16xlarge,
            "m5d.24xlarge" => InstanceType::M5d24xlarge,
            "m5d.2xlarge" => InstanceType::M5d2xlarge,
            "m5d.4xlarge" => InstanceType::M5d4xlarge,
            "m5d.8xlarge" => InstanceType::M5d8xlarge,
            "m5d.large" => InstanceType::M5dLarge,
            "m5d.metal" => InstanceType::M5dMetal,
            "m5d.xlarge" => InstanceType::M5dXlarge,
            "m5dn.12xlarge" => InstanceType::M5dn12xlarge,
            "m5dn.16xlarge" => InstanceType::M5dn16xlarge,
            "m5dn.24xlarge" => InstanceType::M5dn24xlarge,
            "m5dn.2xlarge" => InstanceType::M5dn2xlarge,
            "m5dn.4xlarge" => InstanceType::M5dn4xlarge,
            "m5dn.8xlarge" => InstanceType::M5dn8xlarge,
            "m5dn.large" => InstanceType::M5dnLarge,
            "m5dn.metal" => InstanceType::M5dnMetal,
            "m5dn.xlarge" => InstanceType::M5dnXlarge,
            "m5n.12xlarge" => InstanceType::M5n12xlarge,
            "m5n.16xlarge" => InstanceType::M5n16xlarge,
            "m5n.24xlarge" => InstanceType::M5n24xlarge,
            "m5n.2xlarge" => InstanceType::M5n2xlarge,
            "m5n.4xlarge" => InstanceType::M5n4xlarge,
            "m5n.8xlarge" => InstanceType::M5n8xlarge,
            "m5n.large" => InstanceType::M5nLarge,
            "m5n.metal" => InstanceType::M5nMetal,
            "m5n.xlarge" => InstanceType::M5nXlarge,
            "m5zn.12xlarge" => InstanceType::M5zn12xlarge,
            "m5zn.2xlarge" => InstanceType::M5zn2xlarge,
            "m5zn.3xlarge" => InstanceType::M5zn3xlarge,
            "m5zn.6xlarge" => InstanceType::M5zn6xlarge,
            "m5zn.large" => InstanceType::M5znLarge,
            "m5zn.metal" => InstanceType::M5znMetal,
            "m5zn.xlarge" => InstanceType::M5znXlarge,
            "m6a.12xlarge" => InstanceType::M6a12xlarge,
            "m6a.16xlarge" => InstanceType::M6a16xlarge,
            "m6a.24xlarge" => InstanceType::M6a24xlarge,
            "m6a.2xlarge" => InstanceType::M6a2xlarge,
            "m6a.32xlarge" => InstanceType::M6a32xlarge,
            "m6a.48xlarge" => InstanceType::M6a48xlarge,
            "m6a.4xlarge" => InstanceType::M6a4xlarge,
            "m6a.8xlarge" => InstanceType::M6a8xlarge,
            "m6a.large" => InstanceType::M6aLarge,
            "m6a.metal" => InstanceType::M6aMetal,
            "m6a.xlarge" => InstanceType::M6aXlarge,
            "m6g.12xlarge" => InstanceType::M6g12xlarge,
            "m6g.16xlarge" => InstanceType::M6g16xlarge,
            "m6g.2xlarge" => InstanceType::M6g2xlarge,
            "m6g.4xlarge" => InstanceType::M6g4xlarge,
            "m6g.8xlarge" => InstanceType::M6g8xlarge,
            "m6g.large" => InstanceType::M6gLarge,
            "m6g.medium" => InstanceType::M6gMedium,
            "m6g.metal" => InstanceType::M6gMetal,
            "m6g.xlarge" => InstanceType::M6gXlarge,
            "m6gd.12xlarge" => InstanceType::M6gd12xlarge,
            "m6gd.16xlarge" => InstanceType::M6gd16xlarge,
            "m6gd.2xlarge" => InstanceType::M6gd2xlarge,
            "m6gd.4xlarge" => InstanceType::M6gd4xlarge,
            "m6gd.8xlarge" => InstanceType::M6gd8xlarge,
            "m6gd.large" => InstanceType::M6gdLarge,
            "m6gd.medium" => InstanceType::M6gdMedium,
            "m6gd.metal" => InstanceType::M6gdMetal,
            "m6gd.xlarge" => InstanceType::M6gdXlarge,
            "m6i.12xlarge" => InstanceType::M6i12xlarge,
            "m6i.16xlarge" => InstanceType::M6i16xlarge,
            "m6i.24xlarge" => InstanceType::M6i24xlarge,
            "m6i.2xlarge" => InstanceType::M6i2xlarge,
            "m6i.32xlarge" => InstanceType::M6i32xlarge,
            "m6i.4xlarge" => InstanceType::M6i4xlarge,
            "m6i.8xlarge" => InstanceType::M6i8xlarge,
            "m6i.large" => InstanceType::M6iLarge,
            "m6i.metal" => InstanceType::M6iMetal,
            "m6i.xlarge" => InstanceType::M6iXlarge,
            "m6id.12xlarge" => InstanceType::M6id12xlarge,
            "m6id.16xlarge" => InstanceType::M6id16xlarge,
            "m6id.24xlarge" => InstanceType::M6id24xlarge,
            "m6id.2xlarge" => InstanceType::M6id2xlarge,
            "m6id.32xlarge" => InstanceType::M6id32xlarge,
            "m6id.4xlarge" => InstanceType::M6id4xlarge,
            "m6id.8xlarge" => InstanceType::M6id8xlarge,
            "m6id.large" => InstanceType::M6idLarge,
            "m6id.metal" => InstanceType::M6idMetal,
            "m6id.xlarge" => InstanceType::M6idXlarge,
            "m6idn.12xlarge" => InstanceType::M6idn12xlarge,
            "m6idn.16xlarge" => InstanceType::M6idn16xlarge,
            "m6idn.24xlarge" => InstanceType::M6idn24xlarge,
            "m6idn.2xlarge" => InstanceType::M6idn2xlarge,
            "m6idn.32xlarge" => InstanceType::M6idn32xlarge,
            "m6idn.4xlarge" => InstanceType::M6idn4xlarge,
            "m6idn.8xlarge" => InstanceType::M6idn8xlarge,
            "m6idn.large" => InstanceType::M6idnLarge,
            "m6idn.metal" => InstanceType::M6idnMetal,
            "m6idn.xlarge" => InstanceType::M6idnXlarge,
            "m6in.12xlarge" => InstanceType::M6in12xlarge,
            "m6in.16xlarge" => InstanceType::M6in16xlarge,
            "m6in.24xlarge" => InstanceType::M6in24xlarge,
            "m6in.2xlarge" => InstanceType::M6in2xlarge,
            "m6in.32xlarge" => InstanceType::M6in32xlarge,
            "m6in.4xlarge" => InstanceType::M6in4xlarge,
            "m6in.8xlarge" => InstanceType::M6in8xlarge,
            "m6in.large" => InstanceType::M6inLarge,
            "m6in.metal" => InstanceType::M6inMetal,
            "m6in.xlarge" => InstanceType::M6inXlarge,
            "m7a.12xlarge" => InstanceType::M7a12xlarge,
            "m7a.16xlarge" => InstanceType::M7a16xlarge,
            "m7a.24xlarge" => InstanceType::M7a24xlarge,
            "m7a.2xlarge" => InstanceType::M7a2xlarge,
            "m7a.32xlarge" => InstanceType::M7a32xlarge,
            "m7a.48xlarge" => InstanceType::M7a48xlarge,
            "m7a.4xlarge" => InstanceType::M7a4xlarge,
            "m7a.8xlarge" => InstanceType::M7a8xlarge,
            "m7a.large" => InstanceType::M7aLarge,
            "m7a.medium" => InstanceType::M7aMedium,
            "m7a.metal-48xl" => InstanceType::M7aMetal48xl,
            "m7a.xlarge" => InstanceType::M7aXlarge,
            "m7g.12xlarge" => InstanceType::M7g12xlarge,
            "m7g.16xlarge" => InstanceType::M7g16xlarge,
            "m7g.2xlarge" => InstanceType::M7g2xlarge,
            "m7g.4xlarge" => InstanceType::M7g4xlarge,
            "m7g.8xlarge" => InstanceType::M7g8xlarge,
            "m7g.large" => InstanceType::M7gLarge,
            "m7g.medium" => InstanceType::M7gMedium,
            "m7g.metal" => InstanceType::M7gMetal,
            "m7g.xlarge" => InstanceType::M7gXlarge,
            "m7gd.12xlarge" => InstanceType::M7gd12xlarge,
            "m7gd.16xlarge" => InstanceType::M7gd16xlarge,
            "m7gd.2xlarge" => InstanceType::M7gd2xlarge,
            "m7gd.4xlarge" => InstanceType::M7gd4xlarge,
            "m7gd.8xlarge" => InstanceType::M7gd8xlarge,
            "m7gd.large" => InstanceType::M7gdLarge,
            "m7gd.medium" => InstanceType::M7gdMedium,
            "m7gd.xlarge" => InstanceType::M7gdXlarge,
            "m7i-flex.2xlarge" => InstanceType::M7iFlex2xlarge,
            "m7i-flex.4xlarge" => InstanceType::M7iFlex4xlarge,
            "m7i-flex.8xlarge" => InstanceType::M7iFlex8xlarge,
            "m7i-flex.large" => InstanceType::M7iFlexLarge,
            "m7i-flex.xlarge" => InstanceType::M7iFlexXlarge,
            "m7i.12xlarge" => InstanceType::M7i12xlarge,
            "m7i.16xlarge" => InstanceType::M7i16xlarge,
            "m7i.24xlarge" => InstanceType::M7i24xlarge,
            "m7i.2xlarge" => InstanceType::M7i2xlarge,
            "m7i.48xlarge" => InstanceType::M7i48xlarge,
            "m7i.4xlarge" => InstanceType::M7i4xlarge,
            "m7i.8xlarge" => InstanceType::M7i8xlarge,
            "m7i.large" => InstanceType::M7iLarge,
            "m7i.xlarge" => InstanceType::M7iXlarge,
            "mac1.metal" => InstanceType::Mac1Metal,
            "mac2-m2pro.metal" => InstanceType::Mac2M2proMetal,
            "mac2.metal" => InstanceType::Mac2Metal,
            "p2.16xlarge" => InstanceType::P216xlarge,
            "p2.8xlarge" => InstanceType::P28xlarge,
            "p2.xlarge" => InstanceType::P2Xlarge,
            "p3.16xlarge" => InstanceType::P316xlarge,
            "p3.2xlarge" => InstanceType::P32xlarge,
            "p3.8xlarge" => InstanceType::P38xlarge,
            "p3dn.24xlarge" => InstanceType::P3dn24xlarge,
            "p4d.24xlarge" => InstanceType::P4d24xlarge,
            "p4de.24xlarge" => InstanceType::P4de24xlarge,
            "p5.48xlarge" => InstanceType::P548xlarge,
            "r3.2xlarge" => InstanceType::R32xlarge,
            "r3.4xlarge" => InstanceType::R34xlarge,
            "r3.8xlarge" => InstanceType::R38xlarge,
            "r3.large" => InstanceType::R3Large,
            "r3.xlarge" => InstanceType::R3Xlarge,
            "r4.16xlarge" => InstanceType::R416xlarge,
            "r4.2xlarge" => InstanceType::R42xlarge,
            "r4.4xlarge" => InstanceType::R44xlarge,
            "r4.8xlarge" => InstanceType::R48xlarge,
            "r4.large" => InstanceType::R4Large,
            "r4.xlarge" => InstanceType::R4Xlarge,
            "r5.12xlarge" => InstanceType::R512xlarge,
            "r5.16xlarge" => InstanceType::R516xlarge,
            "r5.24xlarge" => InstanceType::R524xlarge,
            "r5.2xlarge" => InstanceType::R52xlarge,
            "r5.4xlarge" => InstanceType::R54xlarge,
            "r5.8xlarge" => InstanceType::R58xlarge,
            "r5.large" => InstanceType::R5Large,
            "r5.metal" => InstanceType::R5Metal,
            "r5.xlarge" => InstanceType::R5Xlarge,
            "r5a.12xlarge" => InstanceType::R5a12xlarge,
            "r5a.16xlarge" => InstanceType::R5a16xlarge,
            "r5a.24xlarge" => InstanceType::R5a24xlarge,
            "r5a.2xlarge" => InstanceType::R5a2xlarge,
            "r5a.4xlarge" => InstanceType::R5a4xlarge,
            "r5a.8xlarge" => InstanceType::R5a8xlarge,
            "r5a.large" => InstanceType::R5aLarge,
            "r5a.xlarge" => InstanceType::R5aXlarge,
            "r5ad.12xlarge" => InstanceType::R5ad12xlarge,
            "r5ad.16xlarge" => InstanceType::R5ad16xlarge,
            "r5ad.24xlarge" => InstanceType::R5ad24xlarge,
            "r5ad.2xlarge" => InstanceType::R5ad2xlarge,
            "r5ad.4xlarge" => InstanceType::R5ad4xlarge,
            "r5ad.8xlarge" => InstanceType::R5ad8xlarge,
            "r5ad.large" => InstanceType::R5adLarge,
            "r5ad.xlarge" => InstanceType::R5adXlarge,
            "r5b.12xlarge" => InstanceType::R5b12xlarge,
            "r5b.16xlarge" => InstanceType::R5b16xlarge,
            "r5b.24xlarge" => InstanceType::R5b24xlarge,
            "r5b.2xlarge" => InstanceType::R5b2xlarge,
            "r5b.4xlarge" => InstanceType::R5b4xlarge,
            "r5b.8xlarge" => InstanceType::R5b8xlarge,
            "r5b.large" => InstanceType::R5bLarge,
            "r5b.metal" => InstanceType::R5bMetal,
            "r5b.xlarge" => InstanceType::R5bXlarge,
            "r5d.12xlarge" => InstanceType::R5d12xlarge,
            "r5d.16xlarge" => InstanceType::R5d16xlarge,
            "r5d.24xlarge" => InstanceType::R5d24xlarge,
            "r5d.2xlarge" => InstanceType::R5d2xlarge,
            "r5d.4xlarge" => InstanceType::R5d4xlarge,
            "r5d.8xlarge" => InstanceType::R5d8xlarge,
            "r5d.large" => InstanceType::R5dLarge,
            "r5d.metal" => InstanceType::R5dMetal,
            "r5d.xlarge" => InstanceType::R5dXlarge,
            "r5dn.12xlarge" => InstanceType::R5dn12xlarge,
            "r5dn.16xlarge" => InstanceType::R5dn16xlarge,
            "r5dn.24xlarge" => InstanceType::R5dn24xlarge,
            "r5dn.2xlarge" => InstanceType::R5dn2xlarge,
            "r5dn.4xlarge" => InstanceType::R5dn4xlarge,
            "r5dn.8xlarge" => InstanceType::R5dn8xlarge,
            "r5dn.large" => InstanceType::R5dnLarge,
            "r5dn.metal" => InstanceType::R5dnMetal,
            "r5dn.xlarge" => InstanceType::R5dnXlarge,
            "r5n.12xlarge" => InstanceType::R5n12xlarge,
            "r5n.16xlarge" => InstanceType::R5n16xlarge,
            "r5n.24xlarge" => InstanceType::R5n24xlarge,
            "r5n.2xlarge" => InstanceType::R5n2xlarge,
            "r5n.4xlarge" => InstanceType::R5n4xlarge,
            "r5n.8xlarge" => InstanceType::R5n8xlarge,
            "r5n.large" => InstanceType::R5nLarge,
            "r5n.metal" => InstanceType::R5nMetal,
            "r5n.xlarge" => InstanceType::R5nXlarge,
            "r6a.12xlarge" => InstanceType::R6a12xlarge,
            "r6a.16xlarge" => InstanceType::R6a16xlarge,
            "r6a.24xlarge" => InstanceType::R6a24xlarge,
            "r6a.2xlarge" => InstanceType::R6a2xlarge,
            "r6a.32xlarge" => InstanceType::R6a32xlarge,
            "r6a.48xlarge" => InstanceType::R6a48xlarge,
            "r6a.4xlarge" => InstanceType::R6a4xlarge,
            "r6a.8xlarge" => InstanceType::R6a8xlarge,
            "r6a.large" => InstanceType::R6aLarge,
            "r6a.metal" => InstanceType::R6aMetal,
            "r6a.xlarge" => InstanceType::R6aXlarge,
            "r6g.12xlarge" => InstanceType::R6g12xlarge,
            "r6g.16xlarge" => InstanceType::R6g16xlarge,
            "r6g.2xlarge" => InstanceType::R6g2xlarge,
            "r6g.4xlarge" => InstanceType::R6g4xlarge,
            "r6g.8xlarge" => InstanceType::R6g8xlarge,
            "r6g.large" => InstanceType::R6gLarge,
            "r6g.medium" => InstanceType::R6gMedium,
            "r6g.metal" => InstanceType::R6gMetal,
            "r6g.xlarge" => InstanceType::R6gXlarge,
            "r6gd.12xlarge" => InstanceType::R6gd12xlarge,
            "r6gd.16xlarge" => InstanceType::R6gd16xlarge,
            "r6gd.2xlarge" => InstanceType::R6gd2xlarge,
            "r6gd.4xlarge" => InstanceType::R6gd4xlarge,
            "r6gd.8xlarge" => InstanceType::R6gd8xlarge,
            "r6gd.large" => InstanceType::R6gdLarge,
            "r6gd.medium" => InstanceType::R6gdMedium,
            "r6gd.metal" => InstanceType::R6gdMetal,
            "r6gd.xlarge" => InstanceType::R6gdXlarge,
            "r6i.12xlarge" => InstanceType::R6i12xlarge,
            "r6i.16xlarge" => InstanceType::R6i16xlarge,
            "r6i.24xlarge" => InstanceType::R6i24xlarge,
            "r6i.2xlarge" => InstanceType::R6i2xlarge,
            "r6i.32xlarge" => InstanceType::R6i32xlarge,
            "r6i.4xlarge" => InstanceType::R6i4xlarge,
            "r6i.8xlarge" => InstanceType::R6i8xlarge,
            "r6i.large" => InstanceType::R6iLarge,
            "r6i.metal" => InstanceType::R6iMetal,
            "r6i.xlarge" => InstanceType::R6iXlarge,
            "r6id.12xlarge" => InstanceType::R6id12xlarge,
            "r6id.16xlarge" => InstanceType::R6id16xlarge,
            "r6id.24xlarge" => InstanceType::R6id24xlarge,
            "r6id.2xlarge" => InstanceType::R6id2xlarge,
            "r6id.32xlarge" => InstanceType::R6id32xlarge,
            "r6id.4xlarge" => InstanceType::R6id4xlarge,
            "r6id.8xlarge" => InstanceType::R6id8xlarge,
            "r6id.large" => InstanceType::R6idLarge,
            "r6id.metal" => InstanceType::R6idMetal,
            "r6id.xlarge" => InstanceType::R6idXlarge,
            "r6idn.12xlarge" => InstanceType::R6idn12xlarge,
            "r6idn.16xlarge" => InstanceType::R6idn16xlarge,
            "r6idn.24xlarge" => InstanceType::R6idn24xlarge,
            "r6idn.2xlarge" => InstanceType::R6idn2xlarge,
            "r6idn.32xlarge" => InstanceType::R6idn32xlarge,
            "r6idn.4xlarge" => InstanceType::R6idn4xlarge,
            "r6idn.8xlarge" => InstanceType::R6idn8xlarge,
            "r6idn.large" => InstanceType::R6idnLarge,
            "r6idn.metal" => InstanceType::R6idnMetal,
            "r6idn.xlarge" => InstanceType::R6idnXlarge,
            "r6in.12xlarge" => InstanceType::R6in12xlarge,
            "r6in.16xlarge" => InstanceType::R6in16xlarge,
            "r6in.24xlarge" => InstanceType::R6in24xlarge,
            "r6in.2xlarge" => InstanceType::R6in2xlarge,
            "r6in.32xlarge" => InstanceType::R6in32xlarge,
            "r6in.4xlarge" => InstanceType::R6in4xlarge,
            "r6in.8xlarge" => InstanceType::R6in8xlarge,
            "r6in.large" => InstanceType::R6inLarge,
            "r6in.metal" => InstanceType::R6inMetal,
            "r6in.xlarge" => InstanceType::R6inXlarge,
            "r7a.12xlarge" => InstanceType::R7a12xlarge,
            "r7a.16xlarge" => InstanceType::R7a16xlarge,
            "r7a.24xlarge" => InstanceType::R7a24xlarge,
            "r7a.2xlarge" => InstanceType::R7a2xlarge,
            "r7a.32xlarge" => InstanceType::R7a32xlarge,
            "r7a.48xlarge" => InstanceType::R7a48xlarge,
            "r7a.4xlarge" => InstanceType::R7a4xlarge,
            "r7a.8xlarge" => InstanceType::R7a8xlarge,
            "r7a.large" => InstanceType::R7aLarge,
            "r7a.medium" => InstanceType::R7aMedium,
            "r7a.metal-48xl" => InstanceType::R7aMetal48xl,
            "r7a.xlarge" => InstanceType::R7aXlarge,
            "r7g.12xlarge" => InstanceType::R7g12xlarge,
            "r7g.16xlarge" => InstanceType::R7g16xlarge,
            "r7g.2xlarge" => InstanceType::R7g2xlarge,
            "r7g.4xlarge" => InstanceType::R7g4xlarge,
            "r7g.8xlarge" => InstanceType::R7g8xlarge,
            "r7g.large" => InstanceType::R7gLarge,
            "r7g.medium" => InstanceType::R7gMedium,
            "r7g.metal" => InstanceType::R7gMetal,
            "r7g.xlarge" => InstanceType::R7gXlarge,
            "r7gd.12xlarge" => InstanceType::R7gd12xlarge,
            "r7gd.16xlarge" => InstanceType::R7gd16xlarge,
            "r7gd.2xlarge" => InstanceType::R7gd2xlarge,
            "r7gd.4xlarge" => InstanceType::R7gd4xlarge,
            "r7gd.8xlarge" => InstanceType::R7gd8xlarge,
            "r7gd.large" => InstanceType::R7gdLarge,
            "r7gd.medium" => InstanceType::R7gdMedium,
            "r7gd.xlarge" => InstanceType::R7gdXlarge,
            "r7i.12xlarge" => InstanceType::R7i12xlarge,
            "r7i.16xlarge" => InstanceType::R7i16xlarge,
            "r7i.24xlarge" => InstanceType::R7i24xlarge,
            "r7i.2xlarge" => InstanceType::R7i2xlarge,
            "r7i.48xlarge" => InstanceType::R7i48xlarge,
            "r7i.4xlarge" => InstanceType::R7i4xlarge,
            "r7i.8xlarge" => InstanceType::R7i8xlarge,
            "r7i.large" => InstanceType::R7iLarge,
            "r7i.xlarge" => InstanceType::R7iXlarge,
            "r7iz.12xlarge" => InstanceType::R7iz12xlarge,
            "r7iz.16xlarge" => InstanceType::R7iz16xlarge,
            "r7iz.2xlarge" => InstanceType::R7iz2xlarge,
            "r7iz.32xlarge" => InstanceType::R7iz32xlarge,
            "r7iz.4xlarge" => InstanceType::R7iz4xlarge,
            "r7iz.8xlarge" => InstanceType::R7iz8xlarge,
            "r7iz.large" => InstanceType::R7izLarge,
            "r7iz.xlarge" => InstanceType::R7izXlarge,
            "t1.micro" => InstanceType::T1Micro,
            "t2.2xlarge" => InstanceType::T22xlarge,
            "t2.large" => InstanceType::T2Large,
            "t2.medium" => InstanceType::T2Medium,
            "t2.micro" => InstanceType::T2Micro,
            "t2.nano" => InstanceType::T2Nano,
            "t2.small" => InstanceType::T2Small,
            "t2.xlarge" => InstanceType::T2Xlarge,
            "t3.2xlarge" => InstanceType::T32xlarge,
            "t3.large" => InstanceType::T3Large,
            "t3.medium" => InstanceType::T3Medium,
            "t3.micro" => InstanceType::T3Micro,
            "t3.nano" => InstanceType::T3Nano,
            "t3.small" => InstanceType::T3Small,
            "t3.xlarge" => InstanceType::T3Xlarge,
            "t3a.2xlarge" => InstanceType::T3a2xlarge,
            "t3a.large" => InstanceType::T3aLarge,
            "t3a.medium" => InstanceType::T3aMedium,
            "t3a.micro" => InstanceType::T3aMicro,
            "t3a.nano" => InstanceType::T3aNano,
            "t3a.small" => InstanceType::T3aSmall,
            "t3a.xlarge" => InstanceType::T3aXlarge,
            "t4g.2xlarge" => InstanceType::T4g2xlarge,
            "t4g.large" => InstanceType::T4gLarge,
            "t4g.medium" => InstanceType::T4gMedium,
            "t4g.micro" => InstanceType::T4gMicro,
            "t4g.nano" => InstanceType::T4gNano,
            "t4g.small" => InstanceType::T4gSmall,
            "t4g.xlarge" => InstanceType::T4gXlarge,
            "trn1.2xlarge" => InstanceType::Trn12xlarge,
            "trn1.32xlarge" => InstanceType::Trn132xlarge,
            "trn1n.32xlarge" => InstanceType::Trn1n32xlarge,
            "u-12tb1.112xlarge" => InstanceType::U12tb1112xlarge,
            "u-12tb1.metal" => InstanceType::U12tb1Metal,
            "u-18tb1.112xlarge" => InstanceType::U18tb1112xlarge,
            "u-18tb1.metal" => InstanceType::U18tb1Metal,
            "u-24tb1.112xlarge" => InstanceType::U24tb1112xlarge,
            "u-24tb1.metal" => InstanceType::U24tb1Metal,
            "u-3tb1.56xlarge" => InstanceType::U3tb156xlarge,
            "u-6tb1.112xlarge" => InstanceType::U6tb1112xlarge,
            "u-6tb1.56xlarge" => InstanceType::U6tb156xlarge,
            "u-6tb1.metal" => InstanceType::U6tb1Metal,
            "u-9tb1.112xlarge" => InstanceType::U9tb1112xlarge,
            "u-9tb1.metal" => InstanceType::U9tb1Metal,
            "vt1.24xlarge" => InstanceType::Vt124xlarge,
            "vt1.3xlarge" => InstanceType::Vt13xlarge,
            "vt1.6xlarge" => InstanceType::Vt16xlarge,
            "x1.16xlarge" => InstanceType::X116xlarge,
            "x1.32xlarge" => InstanceType::X132xlarge,
            "x1e.16xlarge" => InstanceType::X1e16xlarge,
            "x1e.2xlarge" => InstanceType::X1e2xlarge,
            "x1e.32xlarge" => InstanceType::X1e32xlarge,
            "x1e.4xlarge" => InstanceType::X1e4xlarge,
            "x1e.8xlarge" => InstanceType::X1e8xlarge,
            "x1e.xlarge" => InstanceType::X1eXlarge,
            "x2gd.12xlarge" => InstanceType::X2gd12xlarge,
            "x2gd.16xlarge" => InstanceType::X2gd16xlarge,
            "x2gd.2xlarge" => InstanceType::X2gd2xlarge,
            "x2gd.4xlarge" => InstanceType::X2gd4xlarge,
            "x2gd.8xlarge" => InstanceType::X2gd8xlarge,
            "x2gd.large" => InstanceType::X2gdLarge,
            "x2gd.medium" => InstanceType::X2gdMedium,
            "x2gd.metal" => InstanceType::X2gdMetal,
            "x2gd.xlarge" => InstanceType::X2gdXlarge,
            "x2idn.16xlarge" => InstanceType::X2idn16xlarge,
            "x2idn.24xlarge" => InstanceType::X2idn24xlarge,
            "x2idn.32xlarge" => InstanceType::X2idn32xlarge,
            "x2idn.metal" => InstanceType::X2idnMetal,
            "x2iedn.16xlarge" => InstanceType::X2iedn16xlarge,
            "x2iedn.24xlarge" => InstanceType::X2iedn24xlarge,
            "x2iedn.2xlarge" => InstanceType::X2iedn2xlarge,
            "x2iedn.32xlarge" => InstanceType::X2iedn32xlarge,
            "x2iedn.4xlarge" => InstanceType::X2iedn4xlarge,
            "x2iedn.8xlarge" => InstanceType::X2iedn8xlarge,
            "x2iedn.metal" => InstanceType::X2iednMetal,
            "x2iedn.xlarge" => InstanceType::X2iednXlarge,
            "x2iezn.12xlarge" => InstanceType::X2iezn12xlarge,
            "x2iezn.2xlarge" => InstanceType::X2iezn2xlarge,
            "x2iezn.4xlarge" => InstanceType::X2iezn4xlarge,
            "x2iezn.6xlarge" => InstanceType::X2iezn6xlarge,
            "x2iezn.8xlarge" => InstanceType::X2iezn8xlarge,
            "x2iezn.metal" => InstanceType::X2ieznMetal,
            "z1d.12xlarge" => InstanceType::Z1d12xlarge,
            "z1d.2xlarge" => InstanceType::Z1d2xlarge,
            "z1d.3xlarge" => InstanceType::Z1d3xlarge,
            "z1d.6xlarge" => InstanceType::Z1d6xlarge,
            "z1d.large" => InstanceType::Z1dLarge,
            "z1d.metal" => InstanceType::Z1dMetal,
            "z1d.xlarge" => InstanceType::Z1dXlarge,
            _other => InstanceType::Unknown,
        }
    }
}
impl ::std::str::FromStr for InstanceType {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(InstanceType::from(s))
    }
}
impl InstanceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            InstanceType::A12xlarge => "a1.2xlarge",
            InstanceType::A14xlarge => "a1.4xlarge",
            InstanceType::A1Large => "a1.large",
            InstanceType::A1Medium => "a1.medium",
            InstanceType::A1Metal => "a1.metal",
            InstanceType::A1Xlarge => "a1.xlarge",
            InstanceType::C1Medium => "c1.medium",
            InstanceType::C1Xlarge => "c1.xlarge",
            InstanceType::C32xlarge => "c3.2xlarge",
            InstanceType::C34xlarge => "c3.4xlarge",
            InstanceType::C38xlarge => "c3.8xlarge",
            InstanceType::C3Large => "c3.large",
            InstanceType::C3Xlarge => "c3.xlarge",
            InstanceType::C42xlarge => "c4.2xlarge",
            InstanceType::C44xlarge => "c4.4xlarge",
            InstanceType::C48xlarge => "c4.8xlarge",
            InstanceType::C4Large => "c4.large",
            InstanceType::C4Xlarge => "c4.xlarge",
            InstanceType::C512xlarge => "c5.12xlarge",
            InstanceType::C518xlarge => "c5.18xlarge",
            InstanceType::C524xlarge => "c5.24xlarge",
            InstanceType::C52xlarge => "c5.2xlarge",
            InstanceType::C54xlarge => "c5.4xlarge",
            InstanceType::C59xlarge => "c5.9xlarge",
            InstanceType::C5Large => "c5.large",
            InstanceType::C5Metal => "c5.metal",
            InstanceType::C5Xlarge => "c5.xlarge",
            InstanceType::C5a12xlarge => "c5a.12xlarge",
            InstanceType::C5a16xlarge => "c5a.16xlarge",
            InstanceType::C5a24xlarge => "c5a.24xlarge",
            InstanceType::C5a2xlarge => "c5a.2xlarge",
            InstanceType::C5a4xlarge => "c5a.4xlarge",
            InstanceType::C5a8xlarge => "c5a.8xlarge",
            InstanceType::C5aLarge => "c5a.large",
            InstanceType::C5aXlarge => "c5a.xlarge",
            InstanceType::C5ad12xlarge => "c5ad.12xlarge",
            InstanceType::C5ad16xlarge => "c5ad.16xlarge",
            InstanceType::C5ad24xlarge => "c5ad.24xlarge",
            InstanceType::C5ad2xlarge => "c5ad.2xlarge",
            InstanceType::C5ad4xlarge => "c5ad.4xlarge",
            InstanceType::C5ad8xlarge => "c5ad.8xlarge",
            InstanceType::C5adLarge => "c5ad.large",
            InstanceType::C5adXlarge => "c5ad.xlarge",
            InstanceType::C5d12xlarge => "c5d.12xlarge",
            InstanceType::C5d18xlarge => "c5d.18xlarge",
            InstanceType::C5d24xlarge => "c5d.24xlarge",
            InstanceType::C5d2xlarge => "c5d.2xlarge",
            InstanceType::C5d4xlarge => "c5d.4xlarge",
            InstanceType::C5d9xlarge => "c5d.9xlarge",
            InstanceType::C5dLarge => "c5d.large",
            InstanceType::C5dMetal => "c5d.metal",
            InstanceType::C5dXlarge => "c5d.xlarge",
            InstanceType::C5n18xlarge => "c5n.18xlarge",
            InstanceType::C5n2xlarge => "c5n.2xlarge",
            InstanceType::C5n4xlarge => "c5n.4xlarge",
            InstanceType::C5n9xlarge => "c5n.9xlarge",
            InstanceType::C5nLarge => "c5n.large",
            InstanceType::C5nMetal => "c5n.metal",
            InstanceType::C5nXlarge => "c5n.xlarge",
            InstanceType::C6a12xlarge => "c6a.12xlarge",
            InstanceType::C6a16xlarge => "c6a.16xlarge",
            InstanceType::C6a24xlarge => "c6a.24xlarge",
            InstanceType::C6a2xlarge => "c6a.2xlarge",
            InstanceType::C6a32xlarge => "c6a.32xlarge",
            InstanceType::C6a48xlarge => "c6a.48xlarge",
            InstanceType::C6a4xlarge => "c6a.4xlarge",
            InstanceType::C6a8xlarge => "c6a.8xlarge",
            InstanceType::C6aLarge => "c6a.large",
            InstanceType::C6aMetal => "c6a.metal",
            InstanceType::C6aXlarge => "c6a.xlarge",
            InstanceType::C6g12xlarge => "c6g.12xlarge",
            InstanceType::C6g16xlarge => "c6g.16xlarge",
            InstanceType::C6g2xlarge => "c6g.2xlarge",
            InstanceType::C6g4xlarge => "c6g.4xlarge",
            InstanceType::C6g8xlarge => "c6g.8xlarge",
            InstanceType::C6gLarge => "c6g.large",
            InstanceType::C6gMedium => "c6g.medium",
            InstanceType::C6gMetal => "c6g.metal",
            InstanceType::C6gXlarge => "c6g.xlarge",
            InstanceType::C6gd12xlarge => "c6gd.12xlarge",
            InstanceType::C6gd16xlarge => "c6gd.16xlarge",
            InstanceType::C6gd2xlarge => "c6gd.2xlarge",
            InstanceType::C6gd4xlarge => "c6gd.4xlarge",
            InstanceType::C6gd8xlarge => "c6gd.8xlarge",
            InstanceType::C6gdLarge => "c6gd.large",
            InstanceType::C6gdMedium => "c6gd.medium",
            InstanceType::C6gdMetal => "c6gd.metal",
            InstanceType::C6gdXlarge => "c6gd.xlarge",
            InstanceType::C6gn12xlarge => "c6gn.12xlarge",
            InstanceType::C6gn16xlarge => "c6gn.16xlarge",
            InstanceType::C6gn2xlarge => "c6gn.2xlarge",
            InstanceType::C6gn4xlarge => "c6gn.4xlarge",
            InstanceType::C6gn8xlarge => "c6gn.8xlarge",
            InstanceType::C6gnLarge => "c6gn.large",
            InstanceType::C6gnMedium => "c6gn.medium",
            InstanceType::C6gnXlarge => "c6gn.xlarge",
            InstanceType::C6i12xlarge => "c6i.12xlarge",
            InstanceType::C6i16xlarge => "c6i.16xlarge",
            InstanceType::C6i24xlarge => "c6i.24xlarge",
            InstanceType::C6i2xlarge => "c6i.2xlarge",
            InstanceType::C6i32xlarge => "c6i.32xlarge",
            InstanceType::C6i4xlarge => "c6i.4xlarge",
            InstanceType::C6i8xlarge => "c6i.8xlarge",
            InstanceType::C6iLarge => "c6i.large",
            InstanceType::C6iMetal => "c6i.metal",
            InstanceType::C6iXlarge => "c6i.xlarge",
            InstanceType::C6id12xlarge => "c6id.12xlarge",
            InstanceType::C6id16xlarge => "c6id.16xlarge",
            InstanceType::C6id24xlarge => "c6id.24xlarge",
            InstanceType::C6id2xlarge => "c6id.2xlarge",
            InstanceType::C6id32xlarge => "c6id.32xlarge",
            InstanceType::C6id4xlarge => "c6id.4xlarge",
            InstanceType::C6id8xlarge => "c6id.8xlarge",
            InstanceType::C6idLarge => "c6id.large",
            InstanceType::C6idMetal => "c6id.metal",
            InstanceType::C6idXlarge => "c6id.xlarge",
            InstanceType::C6in12xlarge => "c6in.12xlarge",
            InstanceType::C6in16xlarge => "c6in.16xlarge",
            InstanceType::C6in24xlarge => "c6in.24xlarge",
            InstanceType::C6in2xlarge => "c6in.2xlarge",
            InstanceType::C6in32xlarge => "c6in.32xlarge",
            InstanceType::C6in4xlarge => "c6in.4xlarge",
            InstanceType::C6in8xlarge => "c6in.8xlarge",
            InstanceType::C6inLarge => "c6in.large",
            InstanceType::C6inMetal => "c6in.metal",
            InstanceType::C6inXlarge => "c6in.xlarge",
            InstanceType::C7a12xlarge => "c7a.12xlarge",
            InstanceType::C7a16xlarge => "c7a.16xlarge",
            InstanceType::C7a24xlarge => "c7a.24xlarge",
            InstanceType::C7a2xlarge => "c7a.2xlarge",
            InstanceType::C7a32xlarge => "c7a.32xlarge",
            InstanceType::C7a48xlarge => "c7a.48xlarge",
            InstanceType::C7a4xlarge => "c7a.4xlarge",
            InstanceType::C7a8xlarge => "c7a.8xlarge",
            InstanceType::C7aLarge => "c7a.large",
            InstanceType::C7aMedium => "c7a.medium",
            InstanceType::C7aMetal48xl => "c7a.metal-48xl",
            InstanceType::C7aXlarge => "c7a.xlarge",
            InstanceType::C7g12xlarge => "c7g.12xlarge",
            InstanceType::C7g16xlarge => "c7g.16xlarge",
            InstanceType::C7g2xlarge => "c7g.2xlarge",
            InstanceType::C7g4xlarge => "c7g.4xlarge",
            InstanceType::C7g8xlarge => "c7g.8xlarge",
            InstanceType::C7gLarge => "c7g.large",
            InstanceType::C7gMedium => "c7g.medium",
            InstanceType::C7gMetal => "c7g.metal",
            InstanceType::C7gXlarge => "c7g.xlarge",
            InstanceType::C7gd12xlarge => "c7gd.12xlarge",
            InstanceType::C7gd16xlarge => "c7gd.16xlarge",
            InstanceType::C7gd2xlarge => "c7gd.2xlarge",
            InstanceType::C7gd4xlarge => "c7gd.4xlarge",
            InstanceType::C7gd8xlarge => "c7gd.8xlarge",
            InstanceType::C7gdLarge => "c7gd.large",
            InstanceType::C7gdMedium => "c7gd.medium",
            InstanceType::C7gdXlarge => "c7gd.xlarge",
            InstanceType::C7gn12xlarge => "c7gn.12xlarge",
            InstanceType::C7gn16xlarge => "c7gn.16xlarge",
            InstanceType::C7gn2xlarge => "c7gn.2xlarge",
            InstanceType::C7gn4xlarge => "c7gn.4xlarge",
            InstanceType::C7gn8xlarge => "c7gn.8xlarge",
            InstanceType::C7gnLarge => "c7gn.large",
            InstanceType::C7gnMedium => "c7gn.medium",
            InstanceType::C7gnXlarge => "c7gn.xlarge",
            InstanceType::C7i12xlarge => "c7i.12xlarge",
            InstanceType::C7i16xlarge => "c7i.16xlarge",
            InstanceType::C7i24xlarge => "c7i.24xlarge",
            InstanceType::C7i2xlarge => "c7i.2xlarge",
            InstanceType::C7i48xlarge => "c7i.48xlarge",
            InstanceType::C7i4xlarge => "c7i.4xlarge",
            InstanceType::C7i8xlarge => "c7i.8xlarge",
            InstanceType::C7iLarge => "c7i.large",
            InstanceType::C7iXlarge => "c7i.xlarge",
            InstanceType::Cc14xlarge => "cc1.4xlarge",
            InstanceType::Cc28xlarge => "cc2.8xlarge",
            InstanceType::Cg14xlarge => "cg1.4xlarge",
            InstanceType::Cr18xlarge => "cr1.8xlarge",
            InstanceType::D22xlarge => "d2.2xlarge",
            InstanceType::D24xlarge => "d2.4xlarge",
            InstanceType::D28xlarge => "d2.8xlarge",
            InstanceType::D2Xlarge => "d2.xlarge",
            InstanceType::D32xlarge => "d3.2xlarge",
            InstanceType::D34xlarge => "d3.4xlarge",
            InstanceType::D38xlarge => "d3.8xlarge",
            InstanceType::D3Xlarge => "d3.xlarge",
            InstanceType::D3en12xlarge => "d3en.12xlarge",
            InstanceType::D3en2xlarge => "d3en.2xlarge",
            InstanceType::D3en4xlarge => "d3en.4xlarge",
            InstanceType::D3en6xlarge => "d3en.6xlarge",
            InstanceType::D3en8xlarge => "d3en.8xlarge",
            InstanceType::D3enXlarge => "d3en.xlarge",
            InstanceType::Dl124xlarge => "dl1.24xlarge",
            InstanceType::Dl2q24xlarge => "dl2q.24xlarge",
            InstanceType::F116xlarge => "f1.16xlarge",
            InstanceType::F12xlarge => "f1.2xlarge",
            InstanceType::F14xlarge => "f1.4xlarge",
            InstanceType::G22xlarge => "g2.2xlarge",
            InstanceType::G28xlarge => "g2.8xlarge",
            InstanceType::G316xlarge => "g3.16xlarge",
            InstanceType::G34xlarge => "g3.4xlarge",
            InstanceType::G38xlarge => "g3.8xlarge",
            InstanceType::G3sXlarge => "g3s.xlarge",
            InstanceType::G4ad16xlarge => "g4ad.16xlarge",
            InstanceType::G4ad2xlarge => "g4ad.2xlarge",
            InstanceType::G4ad4xlarge => "g4ad.4xlarge",
            InstanceType::G4ad8xlarge => "g4ad.8xlarge",
            InstanceType::G4adXlarge => "g4ad.xlarge",
            InstanceType::G4dn12xlarge => "g4dn.12xlarge",
            InstanceType::G4dn16xlarge => "g4dn.16xlarge",
            InstanceType::G4dn2xlarge => "g4dn.2xlarge",
            InstanceType::G4dn4xlarge => "g4dn.4xlarge",
            InstanceType::G4dn8xlarge => "g4dn.8xlarge",
            InstanceType::G4dnMetal => "g4dn.metal",
            InstanceType::G4dnXlarge => "g4dn.xlarge",
            InstanceType::G512xlarge => "g5.12xlarge",
            InstanceType::G516xlarge => "g5.16xlarge",
            InstanceType::G524xlarge => "g5.24xlarge",
            InstanceType::G52xlarge => "g5.2xlarge",
            InstanceType::G548xlarge => "g5.48xlarge",
            InstanceType::G54xlarge => "g5.4xlarge",
            InstanceType::G58xlarge => "g5.8xlarge",
            InstanceType::G5Xlarge => "g5.xlarge",
            InstanceType::G5g16xlarge => "g5g.16xlarge",
            InstanceType::G5g2xlarge => "g5g.2xlarge",
            InstanceType::G5g4xlarge => "g5g.4xlarge",
            InstanceType::G5g8xlarge => "g5g.8xlarge",
            InstanceType::G5gMetal => "g5g.metal",
            InstanceType::G5gXlarge => "g5g.xlarge",
            InstanceType::H116xlarge => "h1.16xlarge",
            InstanceType::H12xlarge => "h1.2xlarge",
            InstanceType::H14xlarge => "h1.4xlarge",
            InstanceType::H18xlarge => "h1.8xlarge",
            InstanceType::Hi14xlarge => "hi1.4xlarge",
            InstanceType::Hpc6a48xlarge => "hpc6a.48xlarge",
            InstanceType::Hpc6id32xlarge => "hpc6id.32xlarge",
            InstanceType::Hpc7a12xlarge => "hpc7a.12xlarge",
            InstanceType::Hpc7a24xlarge => "hpc7a.24xlarge",
            InstanceType::Hpc7a48xlarge => "hpc7a.48xlarge",
            InstanceType::Hpc7a96xlarge => "hpc7a.96xlarge",
            InstanceType::Hpc7g16xlarge => "hpc7g.16xlarge",
            InstanceType::Hpc7g4xlarge => "hpc7g.4xlarge",
            InstanceType::Hpc7g8xlarge => "hpc7g.8xlarge",
            InstanceType::Hs18xlarge => "hs1.8xlarge",
            InstanceType::I22xlarge => "i2.2xlarge",
            InstanceType::I24xlarge => "i2.4xlarge",
            InstanceType::I28xlarge => "i2.8xlarge",
            InstanceType::I2Xlarge => "i2.xlarge",
            InstanceType::I316xlarge => "i3.16xlarge",
            InstanceType::I32xlarge => "i3.2xlarge",
            InstanceType::I34xlarge => "i3.4xlarge",
            InstanceType::I38xlarge => "i3.8xlarge",
            InstanceType::I3Large => "i3.large",
            InstanceType::I3Metal => "i3.metal",
            InstanceType::I3Xlarge => "i3.xlarge",
            InstanceType::I3en12xlarge => "i3en.12xlarge",
            InstanceType::I3en24xlarge => "i3en.24xlarge",
            InstanceType::I3en2xlarge => "i3en.2xlarge",
            InstanceType::I3en3xlarge => "i3en.3xlarge",
            InstanceType::I3en6xlarge => "i3en.6xlarge",
            InstanceType::I3enLarge => "i3en.large",
            InstanceType::I3enMetal => "i3en.metal",
            InstanceType::I3enXlarge => "i3en.xlarge",
            InstanceType::I4g16xlarge => "i4g.16xlarge",
            InstanceType::I4g2xlarge => "i4g.2xlarge",
            InstanceType::I4g4xlarge => "i4g.4xlarge",
            InstanceType::I4g8xlarge => "i4g.8xlarge",
            InstanceType::I4gLarge => "i4g.large",
            InstanceType::I4gXlarge => "i4g.xlarge",
            InstanceType::I4i16xlarge => "i4i.16xlarge",
            InstanceType::I4i2xlarge => "i4i.2xlarge",
            InstanceType::I4i32xlarge => "i4i.32xlarge",
            InstanceType::I4i4xlarge => "i4i.4xlarge",
            InstanceType::I4i8xlarge => "i4i.8xlarge",
            InstanceType::I4iLarge => "i4i.large",
            InstanceType::I4iMetal => "i4i.metal",
            InstanceType::I4iXlarge => "i4i.xlarge",
            InstanceType::Im4gn16xlarge => "im4gn.16xlarge",
            InstanceType::Im4gn2xlarge => "im4gn.2xlarge",
            InstanceType::Im4gn4xlarge => "im4gn.4xlarge",
            InstanceType::Im4gn8xlarge => "im4gn.8xlarge",
            InstanceType::Im4gnLarge => "im4gn.large",
            InstanceType::Im4gnXlarge => "im4gn.xlarge",
            InstanceType::Inf124xlarge => "inf1.24xlarge",
            InstanceType::Inf12xlarge => "inf1.2xlarge",
            InstanceType::Inf16xlarge => "inf1.6xlarge",
            InstanceType::Inf1Xlarge => "inf1.xlarge",
            InstanceType::Inf224xlarge => "inf2.24xlarge",
            InstanceType::Inf248xlarge => "inf2.48xlarge",
            InstanceType::Inf28xlarge => "inf2.8xlarge",
            InstanceType::Inf2Xlarge => "inf2.xlarge",
            InstanceType::Is4gen2xlarge => "is4gen.2xlarge",
            InstanceType::Is4gen4xlarge => "is4gen.4xlarge",
            InstanceType::Is4gen8xlarge => "is4gen.8xlarge",
            InstanceType::Is4genLarge => "is4gen.large",
            InstanceType::Is4genMedium => "is4gen.medium",
            InstanceType::Is4genXlarge => "is4gen.xlarge",
            InstanceType::M1Large => "m1.large",
            InstanceType::M1Medium => "m1.medium",
            InstanceType::M1Small => "m1.small",
            InstanceType::M1Xlarge => "m1.xlarge",
            InstanceType::M22xlarge => "m2.2xlarge",
            InstanceType::M24xlarge => "m2.4xlarge",
            InstanceType::M2Xlarge => "m2.xlarge",
            InstanceType::M32xlarge => "m3.2xlarge",
            InstanceType::M3Large => "m3.large",
            InstanceType::M3Medium => "m3.medium",
            InstanceType::M3Xlarge => "m3.xlarge",
            InstanceType::M410xlarge => "m4.10xlarge",
            InstanceType::M416xlarge => "m4.16xlarge",
            InstanceType::M42xlarge => "m4.2xlarge",
            InstanceType::M44xlarge => "m4.4xlarge",
            InstanceType::M4Large => "m4.large",
            InstanceType::M4Xlarge => "m4.xlarge",
            InstanceType::M512xlarge => "m5.12xlarge",
            InstanceType::M516xlarge => "m5.16xlarge",
            InstanceType::M524xlarge => "m5.24xlarge",
            InstanceType::M52xlarge => "m5.2xlarge",
            InstanceType::M54xlarge => "m5.4xlarge",
            InstanceType::M58xlarge => "m5.8xlarge",
            InstanceType::M5Large => "m5.large",
            InstanceType::M5Metal => "m5.metal",
            InstanceType::M5Xlarge => "m5.xlarge",
            InstanceType::M5a12xlarge => "m5a.12xlarge",
            InstanceType::M5a16xlarge => "m5a.16xlarge",
            InstanceType::M5a24xlarge => "m5a.24xlarge",
            InstanceType::M5a2xlarge => "m5a.2xlarge",
            InstanceType::M5a4xlarge => "m5a.4xlarge",
            InstanceType::M5a8xlarge => "m5a.8xlarge",
            InstanceType::M5aLarge => "m5a.large",
            InstanceType::M5aXlarge => "m5a.xlarge",
            InstanceType::M5ad12xlarge => "m5ad.12xlarge",
            InstanceType::M5ad16xlarge => "m5ad.16xlarge",
            InstanceType::M5ad24xlarge => "m5ad.24xlarge",
            InstanceType::M5ad2xlarge => "m5ad.2xlarge",
            InstanceType::M5ad4xlarge => "m5ad.4xlarge",
            InstanceType::M5ad8xlarge => "m5ad.8xlarge",
            InstanceType::M5adLarge => "m5ad.large",
            InstanceType::M5adXlarge => "m5ad.xlarge",
            InstanceType::M5d12xlarge => "m5d.12xlarge",
            InstanceType::M5d16xlarge => "m5d.16xlarge",
            InstanceType::M5d24xlarge => "m5d.24xlarge",
            InstanceType::M5d2xlarge => "m5d.2xlarge",
            InstanceType::M5d4xlarge => "m5d.4xlarge",
            InstanceType::M5d8xlarge => "m5d.8xlarge",
            InstanceType::M5dLarge => "m5d.large",
            InstanceType::M5dMetal => "m5d.metal",
            InstanceType::M5dXlarge => "m5d.xlarge",
            InstanceType::M5dn12xlarge => "m5dn.12xlarge",
            InstanceType::M5dn16xlarge => "m5dn.16xlarge",
            InstanceType::M5dn24xlarge => "m5dn.24xlarge",
            InstanceType::M5dn2xlarge => "m5dn.2xlarge",
            InstanceType::M5dn4xlarge => "m5dn.4xlarge",
            InstanceType::M5dn8xlarge => "m5dn.8xlarge",
            InstanceType::M5dnLarge => "m5dn.large",
            InstanceType::M5dnMetal => "m5dn.metal",
            InstanceType::M5dnXlarge => "m5dn.xlarge",
            InstanceType::M5n12xlarge => "m5n.12xlarge",
            InstanceType::M5n16xlarge => "m5n.16xlarge",
            InstanceType::M5n24xlarge => "m5n.24xlarge",
            InstanceType::M5n2xlarge => "m5n.2xlarge",
            InstanceType::M5n4xlarge => "m5n.4xlarge",
            InstanceType::M5n8xlarge => "m5n.8xlarge",
            InstanceType::M5nLarge => "m5n.large",
            InstanceType::M5nMetal => "m5n.metal",
            InstanceType::M5nXlarge => "m5n.xlarge",
            InstanceType::M5zn12xlarge => "m5zn.12xlarge",
            InstanceType::M5zn2xlarge => "m5zn.2xlarge",
            InstanceType::M5zn3xlarge => "m5zn.3xlarge",
            InstanceType::M5zn6xlarge => "m5zn.6xlarge",
            InstanceType::M5znLarge => "m5zn.large",
            InstanceType::M5znMetal => "m5zn.metal",
            InstanceType::M5znXlarge => "m5zn.xlarge",
            InstanceType::M6a12xlarge => "m6a.12xlarge",
            InstanceType::M6a16xlarge => "m6a.16xlarge",
            InstanceType::M6a24xlarge => "m6a.24xlarge",
            InstanceType::M6a2xlarge => "m6a.2xlarge",
            InstanceType::M6a32xlarge => "m6a.32xlarge",
            InstanceType::M6a48xlarge => "m6a.48xlarge",
            InstanceType::M6a4xlarge => "m6a.4xlarge",
            InstanceType::M6a8xlarge => "m6a.8xlarge",
            InstanceType::M6aLarge => "m6a.large",
            InstanceType::M6aMetal => "m6a.metal",
            InstanceType::M6aXlarge => "m6a.xlarge",
            InstanceType::M6g12xlarge => "m6g.12xlarge",
            InstanceType::M6g16xlarge => "m6g.16xlarge",
            InstanceType::M6g2xlarge => "m6g.2xlarge",
            InstanceType::M6g4xlarge => "m6g.4xlarge",
            InstanceType::M6g8xlarge => "m6g.8xlarge",
            InstanceType::M6gLarge => "m6g.large",
            InstanceType::M6gMedium => "m6g.medium",
            InstanceType::M6gMetal => "m6g.metal",
            InstanceType::M6gXlarge => "m6g.xlarge",
            InstanceType::M6gd12xlarge => "m6gd.12xlarge",
            InstanceType::M6gd16xlarge => "m6gd.16xlarge",
            InstanceType::M6gd2xlarge => "m6gd.2xlarge",
            InstanceType::M6gd4xlarge => "m6gd.4xlarge",
            InstanceType::M6gd8xlarge => "m6gd.8xlarge",
            InstanceType::M6gdLarge => "m6gd.large",
            InstanceType::M6gdMedium => "m6gd.medium",
            InstanceType::M6gdMetal => "m6gd.metal",
            InstanceType::M6gdXlarge => "m6gd.xlarge",
            InstanceType::M6i12xlarge => "m6i.12xlarge",
            InstanceType::M6i16xlarge => "m6i.16xlarge",
            InstanceType::M6i24xlarge => "m6i.24xlarge",
            InstanceType::M6i2xlarge => "m6i.2xlarge",
            InstanceType::M6i32xlarge => "m6i.32xlarge",
            InstanceType::M6i4xlarge => "m6i.4xlarge",
            InstanceType::M6i8xlarge => "m6i.8xlarge",
            InstanceType::M6iLarge => "m6i.large",
            InstanceType::M6iMetal => "m6i.metal",
            InstanceType::M6iXlarge => "m6i.xlarge",
            InstanceType::M6id12xlarge => "m6id.12xlarge",
            InstanceType::M6id16xlarge => "m6id.16xlarge",
            InstanceType::M6id24xlarge => "m6id.24xlarge",
            InstanceType::M6id2xlarge => "m6id.2xlarge",
            InstanceType::M6id32xlarge => "m6id.32xlarge",
            InstanceType::M6id4xlarge => "m6id.4xlarge",
            InstanceType::M6id8xlarge => "m6id.8xlarge",
            InstanceType::M6idLarge => "m6id.large",
            InstanceType::M6idMetal => "m6id.metal",
            InstanceType::M6idXlarge => "m6id.xlarge",
            InstanceType::M6idn12xlarge => "m6idn.12xlarge",
            InstanceType::M6idn16xlarge => "m6idn.16xlarge",
            InstanceType::M6idn24xlarge => "m6idn.24xlarge",
            InstanceType::M6idn2xlarge => "m6idn.2xlarge",
            InstanceType::M6idn32xlarge => "m6idn.32xlarge",
            InstanceType::M6idn4xlarge => "m6idn.4xlarge",
            InstanceType::M6idn8xlarge => "m6idn.8xlarge",
            InstanceType::M6idnLarge => "m6idn.large",
            InstanceType::M6idnMetal => "m6idn.metal",
            InstanceType::M6idnXlarge => "m6idn.xlarge",
            InstanceType::M6in12xlarge => "m6in.12xlarge",
            InstanceType::M6in16xlarge => "m6in.16xlarge",
            InstanceType::M6in24xlarge => "m6in.24xlarge",
            InstanceType::M6in2xlarge => "m6in.2xlarge",
            InstanceType::M6in32xlarge => "m6in.32xlarge",
            InstanceType::M6in4xlarge => "m6in.4xlarge",
            InstanceType::M6in8xlarge => "m6in.8xlarge",
            InstanceType::M6inLarge => "m6in.large",
            InstanceType::M6inMetal => "m6in.metal",
            InstanceType::M6inXlarge => "m6in.xlarge",
            InstanceType::M7a12xlarge => "m7a.12xlarge",
            InstanceType::M7a16xlarge => "m7a.16xlarge",
            InstanceType::M7a24xlarge => "m7a.24xlarge",
            InstanceType::M7a2xlarge => "m7a.2xlarge",
            InstanceType::M7a32xlarge => "m7a.32xlarge",
            InstanceType::M7a48xlarge => "m7a.48xlarge",
            InstanceType::M7a4xlarge => "m7a.4xlarge",
            InstanceType::M7a8xlarge => "m7a.8xlarge",
            InstanceType::M7aLarge => "m7a.large",
            InstanceType::M7aMedium => "m7a.medium",
            InstanceType::M7aMetal48xl => "m7a.metal-48xl",
            InstanceType::M7aXlarge => "m7a.xlarge",
            InstanceType::M7g12xlarge => "m7g.12xlarge",
            InstanceType::M7g16xlarge => "m7g.16xlarge",
            InstanceType::M7g2xlarge => "m7g.2xlarge",
            InstanceType::M7g4xlarge => "m7g.4xlarge",
            InstanceType::M7g8xlarge => "m7g.8xlarge",
            InstanceType::M7gLarge => "m7g.large",
            InstanceType::M7gMedium => "m7g.medium",
            InstanceType::M7gMetal => "m7g.metal",
            InstanceType::M7gXlarge => "m7g.xlarge",
            InstanceType::M7gd12xlarge => "m7gd.12xlarge",
            InstanceType::M7gd16xlarge => "m7gd.16xlarge",
            InstanceType::M7gd2xlarge => "m7gd.2xlarge",
            InstanceType::M7gd4xlarge => "m7gd.4xlarge",
            InstanceType::M7gd8xlarge => "m7gd.8xlarge",
            InstanceType::M7gdLarge => "m7gd.large",
            InstanceType::M7gdMedium => "m7gd.medium",
            InstanceType::M7gdXlarge => "m7gd.xlarge",
            InstanceType::M7iFlex2xlarge => "m7i-flex.2xlarge",
            InstanceType::M7iFlex4xlarge => "m7i-flex.4xlarge",
            InstanceType::M7iFlex8xlarge => "m7i-flex.8xlarge",
            InstanceType::M7iFlexLarge => "m7i-flex.large",
            InstanceType::M7iFlexXlarge => "m7i-flex.xlarge",
            InstanceType::M7i12xlarge => "m7i.12xlarge",
            InstanceType::M7i16xlarge => "m7i.16xlarge",
            InstanceType::M7i24xlarge => "m7i.24xlarge",
            InstanceType::M7i2xlarge => "m7i.2xlarge",
            InstanceType::M7i48xlarge => "m7i.48xlarge",
            InstanceType::M7i4xlarge => "m7i.4xlarge",
            InstanceType::M7i8xlarge => "m7i.8xlarge",
            InstanceType::M7iLarge => "m7i.large",
            InstanceType::M7iXlarge => "m7i.xlarge",
            InstanceType::Mac1Metal => "mac1.metal",
            InstanceType::Mac2M2proMetal => "mac2-m2pro.metal",
            InstanceType::Mac2Metal => "mac2.metal",
            InstanceType::P216xlarge => "p2.16xlarge",
            InstanceType::P28xlarge => "p2.8xlarge",
            InstanceType::P2Xlarge => "p2.xlarge",
            InstanceType::P316xlarge => "p3.16xlarge",
            InstanceType::P32xlarge => "p3.2xlarge",
            InstanceType::P38xlarge => "p3.8xlarge",
            InstanceType::P3dn24xlarge => "p3dn.24xlarge",
            InstanceType::P4d24xlarge => "p4d.24xlarge",
            InstanceType::P4de24xlarge => "p4de.24xlarge",
            InstanceType::P548xlarge => "p5.48xlarge",
            InstanceType::R32xlarge => "r3.2xlarge",
            InstanceType::R34xlarge => "r3.4xlarge",
            InstanceType::R38xlarge => "r3.8xlarge",
            InstanceType::R3Large => "r3.large",
            InstanceType::R3Xlarge => "r3.xlarge",
            InstanceType::R416xlarge => "r4.16xlarge",
            InstanceType::R42xlarge => "r4.2xlarge",
            InstanceType::R44xlarge => "r4.4xlarge",
            InstanceType::R48xlarge => "r4.8xlarge",
            InstanceType::R4Large => "r4.large",
            InstanceType::R4Xlarge => "r4.xlarge",
            InstanceType::R512xlarge => "r5.12xlarge",
            InstanceType::R516xlarge => "r5.16xlarge",
            InstanceType::R524xlarge => "r5.24xlarge",
            InstanceType::R52xlarge => "r5.2xlarge",
            InstanceType::R54xlarge => "r5.4xlarge",
            InstanceType::R58xlarge => "r5.8xlarge",
            InstanceType::R5Large => "r5.large",
            InstanceType::R5Metal => "r5.metal",
            InstanceType::R5Xlarge => "r5.xlarge",
            InstanceType::R5a12xlarge => "r5a.12xlarge",
            InstanceType::R5a16xlarge => "r5a.16xlarge",
            InstanceType::R5a24xlarge => "r5a.24xlarge",
            InstanceType::R5a2xlarge => "r5a.2xlarge",
            InstanceType::R5a4xlarge => "r5a.4xlarge",
            InstanceType::R5a8xlarge => "r5a.8xlarge",
            InstanceType::R5aLarge => "r5a.large",
            InstanceType::R5aXlarge => "r5a.xlarge",
            InstanceType::R5ad12xlarge => "r5ad.12xlarge",
            InstanceType::R5ad16xlarge => "r5ad.16xlarge",
            InstanceType::R5ad24xlarge => "r5ad.24xlarge",
            InstanceType::R5ad2xlarge => "r5ad.2xlarge",
            InstanceType::R5ad4xlarge => "r5ad.4xlarge",
            InstanceType::R5ad8xlarge => "r5ad.8xlarge",
            InstanceType::R5adLarge => "r5ad.large",
            InstanceType::R5adXlarge => "r5ad.xlarge",
            InstanceType::R5b12xlarge => "r5b.12xlarge",
            InstanceType::R5b16xlarge => "r5b.16xlarge",
            InstanceType::R5b24xlarge => "r5b.24xlarge",
            InstanceType::R5b2xlarge => "r5b.2xlarge",
            InstanceType::R5b4xlarge => "r5b.4xlarge",
            InstanceType::R5b8xlarge => "r5b.8xlarge",
            InstanceType::R5bLarge => "r5b.large",
            InstanceType::R5bMetal => "r5b.metal",
            InstanceType::R5bXlarge => "r5b.xlarge",
            InstanceType::R5d12xlarge => "r5d.12xlarge",
            InstanceType::R5d16xlarge => "r5d.16xlarge",
            InstanceType::R5d24xlarge => "r5d.24xlarge",
            InstanceType::R5d2xlarge => "r5d.2xlarge",
            InstanceType::R5d4xlarge => "r5d.4xlarge",
            InstanceType::R5d8xlarge => "r5d.8xlarge",
            InstanceType::R5dLarge => "r5d.large",
            InstanceType::R5dMetal => "r5d.metal",
            InstanceType::R5dXlarge => "r5d.xlarge",
            InstanceType::R5dn12xlarge => "r5dn.12xlarge",
            InstanceType::R5dn16xlarge => "r5dn.16xlarge",
            InstanceType::R5dn24xlarge => "r5dn.24xlarge",
            InstanceType::R5dn2xlarge => "r5dn.2xlarge",
            InstanceType::R5dn4xlarge => "r5dn.4xlarge",
            InstanceType::R5dn8xlarge => "r5dn.8xlarge",
            InstanceType::R5dnLarge => "r5dn.large",
            InstanceType::R5dnMetal => "r5dn.metal",
            InstanceType::R5dnXlarge => "r5dn.xlarge",
            InstanceType::R5n12xlarge => "r5n.12xlarge",
            InstanceType::R5n16xlarge => "r5n.16xlarge",
            InstanceType::R5n24xlarge => "r5n.24xlarge",
            InstanceType::R5n2xlarge => "r5n.2xlarge",
            InstanceType::R5n4xlarge => "r5n.4xlarge",
            InstanceType::R5n8xlarge => "r5n.8xlarge",
            InstanceType::R5nLarge => "r5n.large",
            InstanceType::R5nMetal => "r5n.metal",
            InstanceType::R5nXlarge => "r5n.xlarge",
            InstanceType::R6a12xlarge => "r6a.12xlarge",
            InstanceType::R6a16xlarge => "r6a.16xlarge",
            InstanceType::R6a24xlarge => "r6a.24xlarge",
            InstanceType::R6a2xlarge => "r6a.2xlarge",
            InstanceType::R6a32xlarge => "r6a.32xlarge",
            InstanceType::R6a48xlarge => "r6a.48xlarge",
            InstanceType::R6a4xlarge => "r6a.4xlarge",
            InstanceType::R6a8xlarge => "r6a.8xlarge",
            InstanceType::R6aLarge => "r6a.large",
            InstanceType::R6aMetal => "r6a.metal",
            InstanceType::R6aXlarge => "r6a.xlarge",
            InstanceType::R6g12xlarge => "r6g.12xlarge",
            InstanceType::R6g16xlarge => "r6g.16xlarge",
            InstanceType::R6g2xlarge => "r6g.2xlarge",
            InstanceType::R6g4xlarge => "r6g.4xlarge",
            InstanceType::R6g8xlarge => "r6g.8xlarge",
            InstanceType::R6gLarge => "r6g.large",
            InstanceType::R6gMedium => "r6g.medium",
            InstanceType::R6gMetal => "r6g.metal",
            InstanceType::R6gXlarge => "r6g.xlarge",
            InstanceType::R6gd12xlarge => "r6gd.12xlarge",
            InstanceType::R6gd16xlarge => "r6gd.16xlarge",
            InstanceType::R6gd2xlarge => "r6gd.2xlarge",
            InstanceType::R6gd4xlarge => "r6gd.4xlarge",
            InstanceType::R6gd8xlarge => "r6gd.8xlarge",
            InstanceType::R6gdLarge => "r6gd.large",
            InstanceType::R6gdMedium => "r6gd.medium",
            InstanceType::R6gdMetal => "r6gd.metal",
            InstanceType::R6gdXlarge => "r6gd.xlarge",
            InstanceType::R6i12xlarge => "r6i.12xlarge",
            InstanceType::R6i16xlarge => "r6i.16xlarge",
            InstanceType::R6i24xlarge => "r6i.24xlarge",
            InstanceType::R6i2xlarge => "r6i.2xlarge",
            InstanceType::R6i32xlarge => "r6i.32xlarge",
            InstanceType::R6i4xlarge => "r6i.4xlarge",
            InstanceType::R6i8xlarge => "r6i.8xlarge",
            InstanceType::R6iLarge => "r6i.large",
            InstanceType::R6iMetal => "r6i.metal",
            InstanceType::R6iXlarge => "r6i.xlarge",
            InstanceType::R6id12xlarge => "r6id.12xlarge",
            InstanceType::R6id16xlarge => "r6id.16xlarge",
            InstanceType::R6id24xlarge => "r6id.24xlarge",
            InstanceType::R6id2xlarge => "r6id.2xlarge",
            InstanceType::R6id32xlarge => "r6id.32xlarge",
            InstanceType::R6id4xlarge => "r6id.4xlarge",
            InstanceType::R6id8xlarge => "r6id.8xlarge",
            InstanceType::R6idLarge => "r6id.large",
            InstanceType::R6idMetal => "r6id.metal",
            InstanceType::R6idXlarge => "r6id.xlarge",
            InstanceType::R6idn12xlarge => "r6idn.12xlarge",
            InstanceType::R6idn16xlarge => "r6idn.16xlarge",
            InstanceType::R6idn24xlarge => "r6idn.24xlarge",
            InstanceType::R6idn2xlarge => "r6idn.2xlarge",
            InstanceType::R6idn32xlarge => "r6idn.32xlarge",
            InstanceType::R6idn4xlarge => "r6idn.4xlarge",
            InstanceType::R6idn8xlarge => "r6idn.8xlarge",
            InstanceType::R6idnLarge => "r6idn.large",
            InstanceType::R6idnMetal => "r6idn.metal",
            InstanceType::R6idnXlarge => "r6idn.xlarge",
            InstanceType::R6in12xlarge => "r6in.12xlarge",
            InstanceType::R6in16xlarge => "r6in.16xlarge",
            InstanceType::R6in24xlarge => "r6in.24xlarge",
            InstanceType::R6in2xlarge => "r6in.2xlarge",
            InstanceType::R6in32xlarge => "r6in.32xlarge",
            InstanceType::R6in4xlarge => "r6in.4xlarge",
            InstanceType::R6in8xlarge => "r6in.8xlarge",
            InstanceType::R6inLarge => "r6in.large",
            InstanceType::R6inMetal => "r6in.metal",
            InstanceType::R6inXlarge => "r6in.xlarge",
            InstanceType::R7a12xlarge => "r7a.12xlarge",
            InstanceType::R7a16xlarge => "r7a.16xlarge",
            InstanceType::R7a24xlarge => "r7a.24xlarge",
            InstanceType::R7a2xlarge => "r7a.2xlarge",
            InstanceType::R7a32xlarge => "r7a.32xlarge",
            InstanceType::R7a48xlarge => "r7a.48xlarge",
            InstanceType::R7a4xlarge => "r7a.4xlarge",
            InstanceType::R7a8xlarge => "r7a.8xlarge",
            InstanceType::R7aLarge => "r7a.large",
            InstanceType::R7aMedium => "r7a.medium",
            InstanceType::R7aMetal48xl => "r7a.metal-48xl",
            InstanceType::R7aXlarge => "r7a.xlarge",
            InstanceType::R7g12xlarge => "r7g.12xlarge",
            InstanceType::R7g16xlarge => "r7g.16xlarge",
            InstanceType::R7g2xlarge => "r7g.2xlarge",
            InstanceType::R7g4xlarge => "r7g.4xlarge",
            InstanceType::R7g8xlarge => "r7g.8xlarge",
            InstanceType::R7gLarge => "r7g.large",
            InstanceType::R7gMedium => "r7g.medium",
            InstanceType::R7gMetal => "r7g.metal",
            InstanceType::R7gXlarge => "r7g.xlarge",
            InstanceType::R7gd12xlarge => "r7gd.12xlarge",
            InstanceType::R7gd16xlarge => "r7gd.16xlarge",
            InstanceType::R7gd2xlarge => "r7gd.2xlarge",
            InstanceType::R7gd4xlarge => "r7gd.4xlarge",
            InstanceType::R7gd8xlarge => "r7gd.8xlarge",
            InstanceType::R7gdLarge => "r7gd.large",
            InstanceType::R7gdMedium => "r7gd.medium",
            InstanceType::R7gdXlarge => "r7gd.xlarge",
            InstanceType::R7i12xlarge => "r7i.12xlarge",
            InstanceType::R7i16xlarge => "r7i.16xlarge",
            InstanceType::R7i24xlarge => "r7i.24xlarge",
            InstanceType::R7i2xlarge => "r7i.2xlarge",
            InstanceType::R7i48xlarge => "r7i.48xlarge",
            InstanceType::R7i4xlarge => "r7i.4xlarge",
            InstanceType::R7i8xlarge => "r7i.8xlarge",
            InstanceType::R7iLarge => "r7i.large",
            InstanceType::R7iXlarge => "r7i.xlarge",
            InstanceType::R7iz12xlarge => "r7iz.12xlarge",
            InstanceType::R7iz16xlarge => "r7iz.16xlarge",
            InstanceType::R7iz2xlarge => "r7iz.2xlarge",
            InstanceType::R7iz32xlarge => "r7iz.32xlarge",
            InstanceType::R7iz4xlarge => "r7iz.4xlarge",
            InstanceType::R7iz8xlarge => "r7iz.8xlarge",
            InstanceType::R7izLarge => "r7iz.large",
            InstanceType::R7izXlarge => "r7iz.xlarge",
            InstanceType::T1Micro => "t1.micro",
            InstanceType::T22xlarge => "t2.2xlarge",
            InstanceType::T2Large => "t2.large",
            InstanceType::T2Medium => "t2.medium",
            InstanceType::T2Micro => "t2.micro",
            InstanceType::T2Nano => "t2.nano",
            InstanceType::T2Small => "t2.small",
            InstanceType::T2Xlarge => "t2.xlarge",
            InstanceType::T32xlarge => "t3.2xlarge",
            InstanceType::T3Large => "t3.large",
            InstanceType::T3Medium => "t3.medium",
            InstanceType::T3Micro => "t3.micro",
            InstanceType::T3Nano => "t3.nano",
            InstanceType::T3Small => "t3.small",
            InstanceType::T3Xlarge => "t3.xlarge",
            InstanceType::T3a2xlarge => "t3a.2xlarge",
            InstanceType::T3aLarge => "t3a.large",
            InstanceType::T3aMedium => "t3a.medium",
            InstanceType::T3aMicro => "t3a.micro",
            InstanceType::T3aNano => "t3a.nano",
            InstanceType::T3aSmall => "t3a.small",
            InstanceType::T3aXlarge => "t3a.xlarge",
            InstanceType::T4g2xlarge => "t4g.2xlarge",
            InstanceType::T4gLarge => "t4g.large",
            InstanceType::T4gMedium => "t4g.medium",
            InstanceType::T4gMicro => "t4g.micro",
            InstanceType::T4gNano => "t4g.nano",
            InstanceType::T4gSmall => "t4g.small",
            InstanceType::T4gXlarge => "t4g.xlarge",
            InstanceType::Trn12xlarge => "trn1.2xlarge",
            InstanceType::Trn132xlarge => "trn1.32xlarge",
            InstanceType::Trn1n32xlarge => "trn1n.32xlarge",
            InstanceType::U12tb1112xlarge => "u-12tb1.112xlarge",
            InstanceType::U12tb1Metal => "u-12tb1.metal",
            InstanceType::U18tb1112xlarge => "u-18tb1.112xlarge",
            InstanceType::U18tb1Metal => "u-18tb1.metal",
            InstanceType::U24tb1112xlarge => "u-24tb1.112xlarge",
            InstanceType::U24tb1Metal => "u-24tb1.metal",
            InstanceType::U3tb156xlarge => "u-3tb1.56xlarge",
            InstanceType::U6tb1112xlarge => "u-6tb1.112xlarge",
            InstanceType::U6tb156xlarge => "u-6tb1.56xlarge",
            InstanceType::U6tb1Metal => "u-6tb1.metal",
            InstanceType::U9tb1112xlarge => "u-9tb1.112xlarge",
            InstanceType::U9tb1Metal => "u-9tb1.metal",
            InstanceType::Vt124xlarge => "vt1.24xlarge",
            InstanceType::Vt13xlarge => "vt1.3xlarge",
            InstanceType::Vt16xlarge => "vt1.6xlarge",
            InstanceType::X116xlarge => "x1.16xlarge",
            InstanceType::X132xlarge => "x1.32xlarge",
            InstanceType::X1e16xlarge => "x1e.16xlarge",
            InstanceType::X1e2xlarge => "x1e.2xlarge",
            InstanceType::X1e32xlarge => "x1e.32xlarge",
            InstanceType::X1e4xlarge => "x1e.4xlarge",
            InstanceType::X1e8xlarge => "x1e.8xlarge",
            InstanceType::X1eXlarge => "x1e.xlarge",
            InstanceType::X2gd12xlarge => "x2gd.12xlarge",
            InstanceType::X2gd16xlarge => "x2gd.16xlarge",
            InstanceType::X2gd2xlarge => "x2gd.2xlarge",
            InstanceType::X2gd4xlarge => "x2gd.4xlarge",
            InstanceType::X2gd8xlarge => "x2gd.8xlarge",
            InstanceType::X2gdLarge => "x2gd.large",
            InstanceType::X2gdMedium => "x2gd.medium",
            InstanceType::X2gdMetal => "x2gd.metal",
            InstanceType::X2gdXlarge => "x2gd.xlarge",
            InstanceType::X2idn16xlarge => "x2idn.16xlarge",
            InstanceType::X2idn24xlarge => "x2idn.24xlarge",
            InstanceType::X2idn32xlarge => "x2idn.32xlarge",
            InstanceType::X2idnMetal => "x2idn.metal",
            InstanceType::X2iedn16xlarge => "x2iedn.16xlarge",
            InstanceType::X2iedn24xlarge => "x2iedn.24xlarge",
            InstanceType::X2iedn2xlarge => "x2iedn.2xlarge",
            InstanceType::X2iedn32xlarge => "x2iedn.32xlarge",
            InstanceType::X2iedn4xlarge => "x2iedn.4xlarge",
            InstanceType::X2iedn8xlarge => "x2iedn.8xlarge",
            InstanceType::X2iednMetal => "x2iedn.metal",
            InstanceType::X2iednXlarge => "x2iedn.xlarge",
            InstanceType::X2iezn12xlarge => "x2iezn.12xlarge",
            InstanceType::X2iezn2xlarge => "x2iezn.2xlarge",
            InstanceType::X2iezn4xlarge => "x2iezn.4xlarge",
            InstanceType::X2iezn6xlarge => "x2iezn.6xlarge",
            InstanceType::X2iezn8xlarge => "x2iezn.8xlarge",
            InstanceType::X2ieznMetal => "x2iezn.metal",
            InstanceType::Z1d12xlarge => "z1d.12xlarge",
            InstanceType::Z1d2xlarge => "z1d.2xlarge",
            InstanceType::Z1d3xlarge => "z1d.3xlarge",
            InstanceType::Z1d6xlarge => "z1d.6xlarge",
            InstanceType::Z1dLarge => "z1d.large",
            InstanceType::Z1dMetal => "z1d.metal",
            InstanceType::Z1dXlarge => "z1d.xlarge",
            InstanceType::Unknown => "unknown",
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "a1.2xlarge",
            "a1.4xlarge",
            "a1.large",
            "a1.medium",
            "a1.metal",
            "a1.xlarge",
            "c1.medium",
            "c1.xlarge",
            "c3.2xlarge",
            "c3.4xlarge",
            "c3.8xlarge",
            "c3.large",
            "c3.xlarge",
            "c4.2xlarge",
            "c4.4xlarge",
            "c4.8xlarge",
            "c4.large",
            "c4.xlarge",
            "c5.12xlarge",
            "c5.18xlarge",
            "c5.24xlarge",
            "c5.2xlarge",
            "c5.4xlarge",
            "c5.9xlarge",
            "c5.large",
            "c5.metal",
            "c5.xlarge",
            "c5a.12xlarge",
            "c5a.16xlarge",
            "c5a.24xlarge",
            "c5a.2xlarge",
            "c5a.4xlarge",
            "c5a.8xlarge",
            "c5a.large",
            "c5a.xlarge",
            "c5ad.12xlarge",
            "c5ad.16xlarge",
            "c5ad.24xlarge",
            "c5ad.2xlarge",
            "c5ad.4xlarge",
            "c5ad.8xlarge",
            "c5ad.large",
            "c5ad.xlarge",
            "c5d.12xlarge",
            "c5d.18xlarge",
            "c5d.24xlarge",
            "c5d.2xlarge",
            "c5d.4xlarge",
            "c5d.9xlarge",
            "c5d.large",
            "c5d.metal",
            "c5d.xlarge",
            "c5n.18xlarge",
            "c5n.2xlarge",
            "c5n.4xlarge",
            "c5n.9xlarge",
            "c5n.large",
            "c5n.metal",
            "c5n.xlarge",
            "c6a.12xlarge",
            "c6a.16xlarge",
            "c6a.24xlarge",
            "c6a.2xlarge",
            "c6a.32xlarge",
            "c6a.48xlarge",
            "c6a.4xlarge",
            "c6a.8xlarge",
            "c6a.large",
            "c6a.metal",
            "c6a.xlarge",
            "c6g.12xlarge",
            "c6g.16xlarge",
            "c6g.2xlarge",
            "c6g.4xlarge",
            "c6g.8xlarge",
            "c6g.large",
            "c6g.medium",
            "c6g.metal",
            "c6g.xlarge",
            "c6gd.12xlarge",
            "c6gd.16xlarge",
            "c6gd.2xlarge",
            "c6gd.4xlarge",
            "c6gd.8xlarge",
            "c6gd.large",
            "c6gd.medium",
            "c6gd.metal",
            "c6gd.xlarge",
            "c6gn.12xlarge",
            "c6gn.16xlarge",
            "c6gn.2xlarge",
            "c6gn.4xlarge",
            "c6gn.8xlarge",
            "c6gn.large",
            "c6gn.medium",
            "c6gn.xlarge",
            "c6i.12xlarge",
            "c6i.16xlarge",
            "c6i.24xlarge",
            "c6i.2xlarge",
            "c6i.32xlarge",
            "c6i.4xlarge",
            "c6i.8xlarge",
            "c6i.large",
            "c6i.metal",
            "c6i.xlarge",
            "c6id.12xlarge",
            "c6id.16xlarge",
            "c6id.24xlarge",
            "c6id.2xlarge",
            "c6id.32xlarge",
            "c6id.4xlarge",
            "c6id.8xlarge",
            "c6id.large",
            "c6id.metal",
            "c6id.xlarge",
            "c6in.12xlarge",
            "c6in.16xlarge",
            "c6in.24xlarge",
            "c6in.2xlarge",
            "c6in.32xlarge",
            "c6in.4xlarge",
            "c6in.8xlarge",
            "c6in.large",
            "c6in.metal",
            "c6in.xlarge",
            "c7a.12xlarge",
            "c7a.16xlarge",
            "c7a.24xlarge",
            "c7a.2xlarge",
            "c7a.32xlarge",
            "c7a.48xlarge",
            "c7a.4xlarge",
            "c7a.8xlarge",
            "c7a.large",
            "c7a.medium",
            "c7a.metal-48xl",
            "c7a.xlarge",
            "c7g.12xlarge",
            "c7g.16xlarge",
            "c7g.2xlarge",
            "c7g.4xlarge",
            "c7g.8xlarge",
            "c7g.large",
            "c7g.medium",
            "c7g.metal",
            "c7g.xlarge",
            "c7gd.12xlarge",
            "c7gd.16xlarge",
            "c7gd.2xlarge",
            "c7gd.4xlarge",
            "c7gd.8xlarge",
            "c7gd.large",
            "c7gd.medium",
            "c7gd.xlarge",
            "c7gn.12xlarge",
            "c7gn.16xlarge",
            "c7gn.2xlarge",
            "c7gn.4xlarge",
            "c7gn.8xlarge",
            "c7gn.large",
            "c7gn.medium",
            "c7gn.xlarge",
            "c7i.12xlarge",
            "c7i.16xlarge",
            "c7i.24xlarge",
            "c7i.2xlarge",
            "c7i.48xlarge",
            "c7i.4xlarge",
            "c7i.8xlarge",
            "c7i.large",
            "c7i.xlarge",
            "cc1.4xlarge",
            "cc2.8xlarge",
            "cg1.4xlarge",
            "cr1.8xlarge",
            "d2.2xlarge",
            "d2.4xlarge",
            "d2.8xlarge",
            "d2.xlarge",
            "d3.2xlarge",
            "d3.4xlarge",
            "d3.8xlarge",
            "d3.xlarge",
            "d3en.12xlarge",
            "d3en.2xlarge",
            "d3en.4xlarge",
            "d3en.6xlarge",
            "d3en.8xlarge",
            "d3en.xlarge",
            "dl1.24xlarge",
            "dl2q.24xlarge",
            "f1.16xlarge",
            "f1.2xlarge",
            "f1.4xlarge",
            "g2.2xlarge",
            "g2.8xlarge",
            "g3.16xlarge",
            "g3.4xlarge",
            "g3.8xlarge",
            "g3s.xlarge",
            "g4ad.16xlarge",
            "g4ad.2xlarge",
            "g4ad.4xlarge",
            "g4ad.8xlarge",
            "g4ad.xlarge",
            "g4dn.12xlarge",
            "g4dn.16xlarge",
            "g4dn.2xlarge",
            "g4dn.4xlarge",
            "g4dn.8xlarge",
            "g4dn.metal",
            "g4dn.xlarge",
            "g5.12xlarge",
            "g5.16xlarge",
            "g5.24xlarge",
            "g5.2xlarge",
            "g5.48xlarge",
            "g5.4xlarge",
            "g5.8xlarge",
            "g5.xlarge",
            "g5g.16xlarge",
            "g5g.2xlarge",
            "g5g.4xlarge",
            "g5g.8xlarge",
            "g5g.metal",
            "g5g.xlarge",
            "h1.16xlarge",
            "h1.2xlarge",
            "h1.4xlarge",
            "h1.8xlarge",
            "hi1.4xlarge",
            "hpc6a.48xlarge",
            "hpc6id.32xlarge",
            "hpc7a.12xlarge",
            "hpc7a.24xlarge",
            "hpc7a.48xlarge",
            "hpc7a.96xlarge",
            "hpc7g.16xlarge",
            "hpc7g.4xlarge",
            "hpc7g.8xlarge",
            "hs1.8xlarge",
            "i2.2xlarge",
            "i2.4xlarge",
            "i2.8xlarge",
            "i2.xlarge",
            "i3.16xlarge",
            "i3.2xlarge",
            "i3.4xlarge",
            "i3.8xlarge",
            "i3.large",
            "i3.metal",
            "i3.xlarge",
            "i3en.12xlarge",
            "i3en.24xlarge",
            "i3en.2xlarge",
            "i3en.3xlarge",
            "i3en.6xlarge",
            "i3en.large",
            "i3en.metal",
            "i3en.xlarge",
            "i4g.16xlarge",
            "i4g.2xlarge",
            "i4g.4xlarge",
            "i4g.8xlarge",
            "i4g.large",
            "i4g.xlarge",
            "i4i.16xlarge",
            "i4i.2xlarge",
            "i4i.32xlarge",
            "i4i.4xlarge",
            "i4i.8xlarge",
            "i4i.large",
            "i4i.metal",
            "i4i.xlarge",
            "im4gn.16xlarge",
            "im4gn.2xlarge",
            "im4gn.4xlarge",
            "im4gn.8xlarge",
            "im4gn.large",
            "im4gn.xlarge",
            "inf1.24xlarge",
            "inf1.2xlarge",
            "inf1.6xlarge",
            "inf1.xlarge",
            "inf2.24xlarge",
            "inf2.48xlarge",
            "inf2.8xlarge",
            "inf2.xlarge",
            "is4gen.2xlarge",
            "is4gen.4xlarge",
            "is4gen.8xlarge",
            "is4gen.large",
            "is4gen.medium",
            "is4gen.xlarge",
            "m1.large",
            "m1.medium",
            "m1.small",
            "m1.xlarge",
            "m2.2xlarge",
            "m2.4xlarge",
            "m2.xlarge",
            "m3.2xlarge",
            "m3.large",
            "m3.medium",
            "m3.xlarge",
            "m4.10xlarge",
            "m4.16xlarge",
            "m4.2xlarge",
            "m4.4xlarge",
            "m4.large",
            "m4.xlarge",
            "m5.12xlarge",
            "m5.16xlarge",
            "m5.24xlarge",
            "m5.2xlarge",
            "m5.4xlarge",
            "m5.8xlarge",
            "m5.large",
            "m5.metal",
            "m5.xlarge",
            "m5a.12xlarge",
            "m5a.16xlarge",
            "m5a.24xlarge",
            "m5a.2xlarge",
            "m5a.4xlarge",
            "m5a.8xlarge",
            "m5a.large",
            "m5a.xlarge",
            "m5ad.12xlarge",
            "m5ad.16xlarge",
            "m5ad.24xlarge",
            "m5ad.2xlarge",
            "m5ad.4xlarge",
            "m5ad.8xlarge",
            "m5ad.large",
            "m5ad.xlarge",
            "m5d.12xlarge",
            "m5d.16xlarge",
            "m5d.24xlarge",
            "m5d.2xlarge",
            "m5d.4xlarge",
            "m5d.8xlarge",
            "m5d.large",
            "m5d.metal",
            "m5d.xlarge",
            "m5dn.12xlarge",
            "m5dn.16xlarge",
            "m5dn.24xlarge",
            "m5dn.2xlarge",
            "m5dn.4xlarge",
            "m5dn.8xlarge",
            "m5dn.large",
            "m5dn.metal",
            "m5dn.xlarge",
            "m5n.12xlarge",
            "m5n.16xlarge",
            "m5n.24xlarge",
            "m5n.2xlarge",
            "m5n.4xlarge",
            "m5n.8xlarge",
            "m5n.large",
            "m5n.metal",
            "m5n.xlarge",
            "m5zn.12xlarge",
            "m5zn.2xlarge",
            "m5zn.3xlarge",
            "m5zn.6xlarge",
            "m5zn.large",
            "m5zn.metal",
            "m5zn.xlarge",
            "m6a.12xlarge",
            "m6a.16xlarge",
            "m6a.24xlarge",
            "m6a.2xlarge",
            "m6a.32xlarge",
            "m6a.48xlarge",
            "m6a.4xlarge",
            "m6a.8xlarge",
            "m6a.large",
            "m6a.metal",
            "m6a.xlarge",
            "m6g.12xlarge",
            "m6g.16xlarge",
            "m6g.2xlarge",
            "m6g.4xlarge",
            "m6g.8xlarge",
            "m6g.large",
            "m6g.medium",
            "m6g.metal",
            "m6g.xlarge",
            "m6gd.12xlarge",
            "m6gd.16xlarge",
            "m6gd.2xlarge",
            "m6gd.4xlarge",
            "m6gd.8xlarge",
            "m6gd.large",
            "m6gd.medium",
            "m6gd.metal",
            "m6gd.xlarge",
            "m6i.12xlarge",
            "m6i.16xlarge",
            "m6i.24xlarge",
            "m6i.2xlarge",
            "m6i.32xlarge",
            "m6i.4xlarge",
            "m6i.8xlarge",
            "m6i.large",
            "m6i.metal",
            "m6i.xlarge",
            "m6id.12xlarge",
            "m6id.16xlarge",
            "m6id.24xlarge",
            "m6id.2xlarge",
            "m6id.32xlarge",
            "m6id.4xlarge",
            "m6id.8xlarge",
            "m6id.large",
            "m6id.metal",
            "m6id.xlarge",
            "m6idn.12xlarge",
            "m6idn.16xlarge",
            "m6idn.24xlarge",
            "m6idn.2xlarge",
            "m6idn.32xlarge",
            "m6idn.4xlarge",
            "m6idn.8xlarge",
            "m6idn.large",
            "m6idn.metal",
            "m6idn.xlarge",
            "m6in.12xlarge",
            "m6in.16xlarge",
            "m6in.24xlarge",
            "m6in.2xlarge",
            "m6in.32xlarge",
            "m6in.4xlarge",
            "m6in.8xlarge",
            "m6in.large",
            "m6in.metal",
            "m6in.xlarge",
            "m7a.12xlarge",
            "m7a.16xlarge",
            "m7a.24xlarge",
            "m7a.2xlarge",
            "m7a.32xlarge",
            "m7a.48xlarge",
            "m7a.4xlarge",
            "m7a.8xlarge",
            "m7a.large",
            "m7a.medium",
            "m7a.metal-48xl",
            "m7a.xlarge",
            "m7g.12xlarge",
            "m7g.16xlarge",
            "m7g.2xlarge",
            "m7g.4xlarge",
            "m7g.8xlarge",
            "m7g.large",
            "m7g.medium",
            "m7g.metal",
            "m7g.xlarge",
            "m7gd.12xlarge",
            "m7gd.16xlarge",
            "m7gd.2xlarge",
            "m7gd.4xlarge",
            "m7gd.8xlarge",
            "m7gd.large",
            "m7gd.medium",
            "m7gd.xlarge",
            "m7i-flex.2xlarge",
            "m7i-flex.4xlarge",
            "m7i-flex.8xlarge",
            "m7i-flex.large",
            "m7i-flex.xlarge",
            "m7i.12xlarge",
            "m7i.16xlarge",
            "m7i.24xlarge",
            "m7i.2xlarge",
            "m7i.48xlarge",
            "m7i.4xlarge",
            "m7i.8xlarge",
            "m7i.large",
            "m7i.xlarge",
            "mac1.metal",
            "mac2-m2pro.metal",
            "mac2.metal",
            "p2.16xlarge",
            "p2.8xlarge",
            "p2.xlarge",
            "p3.16xlarge",
            "p3.2xlarge",
            "p3.8xlarge",
            "p3dn.24xlarge",
            "p4d.24xlarge",
            "p4de.24xlarge",
            "p5.48xlarge",
            "r3.2xlarge",
            "r3.4xlarge",
            "r3.8xlarge",
            "r3.large",
            "r3.xlarge",
            "r4.16xlarge",
            "r4.2xlarge",
            "r4.4xlarge",
            "r4.8xlarge",
            "r4.large",
            "r4.xlarge",
            "r5.12xlarge",
            "r5.16xlarge",
            "r5.24xlarge",
            "r5.2xlarge",
            "r5.4xlarge",
            "r5.8xlarge",
            "r5.large",
            "r5.metal",
            "r5.xlarge",
            "r5a.12xlarge",
            "r5a.16xlarge",
            "r5a.24xlarge",
            "r5a.2xlarge",
            "r5a.4xlarge",
            "r5a.8xlarge",
            "r5a.large",
            "r5a.xlarge",
            "r5ad.12xlarge",
            "r5ad.16xlarge",
            "r5ad.24xlarge",
            "r5ad.2xlarge",
            "r5ad.4xlarge",
            "r5ad.8xlarge",
            "r5ad.large",
            "r5ad.xlarge",
            "r5b.12xlarge",
            "r5b.16xlarge",
            "r5b.24xlarge",
            "r5b.2xlarge",
            "r5b.4xlarge",
            "r5b.8xlarge",
            "r5b.large",
            "r5b.metal",
            "r5b.xlarge",
            "r5d.12xlarge",
            "r5d.16xlarge",
            "r5d.24xlarge",
            "r5d.2xlarge",
            "r5d.4xlarge",
            "r5d.8xlarge",
            "r5d.large",
            "r5d.metal",
            "r5d.xlarge",
            "r5dn.12xlarge",
            "r5dn.16xlarge",
            "r5dn.24xlarge",
            "r5dn.2xlarge",
            "r5dn.4xlarge",
            "r5dn.8xlarge",
            "r5dn.large",
            "r5dn.metal",
            "r5dn.xlarge",
            "r5n.12xlarge",
            "r5n.16xlarge",
            "r5n.24xlarge",
            "r5n.2xlarge",
            "r5n.4xlarge",
            "r5n.8xlarge",
            "r5n.large",
            "r5n.metal",
            "r5n.xlarge",
            "r6a.12xlarge",
            "r6a.16xlarge",
            "r6a.24xlarge",
            "r6a.2xlarge",
            "r6a.32xlarge",
            "r6a.48xlarge",
            "r6a.4xlarge",
            "r6a.8xlarge",
            "r6a.large",
            "r6a.metal",
            "r6a.xlarge",
            "r6g.12xlarge",
            "r6g.16xlarge",
            "r6g.2xlarge",
            "r6g.4xlarge",
            "r6g.8xlarge",
            "r6g.large",
            "r6g.medium",
            "r6g.metal",
            "r6g.xlarge",
            "r6gd.12xlarge",
            "r6gd.16xlarge",
            "r6gd.2xlarge",
            "r6gd.4xlarge",
            "r6gd.8xlarge",
            "r6gd.large",
            "r6gd.medium",
            "r6gd.metal",
            "r6gd.xlarge",
            "r6i.12xlarge",
            "r6i.16xlarge",
            "r6i.24xlarge",
            "r6i.2xlarge",
            "r6i.32xlarge",
            "r6i.4xlarge",
            "r6i.8xlarge",
            "r6i.large",
            "r6i.metal",
            "r6i.xlarge",
            "r6id.12xlarge",
            "r6id.16xlarge",
            "r6id.24xlarge",
            "r6id.2xlarge",
            "r6id.32xlarge",
            "r6id.4xlarge",
            "r6id.8xlarge",
            "r6id.large",
            "r6id.metal",
            "r6id.xlarge",
            "r6idn.12xlarge",
            "r6idn.16xlarge",
            "r6idn.24xlarge",
            "r6idn.2xlarge",
            "r6idn.32xlarge",
            "r6idn.4xlarge",
            "r6idn.8xlarge",
            "r6idn.large",
            "r6idn.metal",
            "r6idn.xlarge",
            "r6in.12xlarge",
            "r6in.16xlarge",
            "r6in.24xlarge",
            "r6in.2xlarge",
            "r6in.32xlarge",
            "r6in.4xlarge",
            "r6in.8xlarge",
            "r6in.large",
            "r6in.metal",
            "r6in.xlarge",
            "r7a.12xlarge",
            "r7a.16xlarge",
            "r7a.24xlarge",
            "r7a.2xlarge",
            "r7a.32xlarge",
            "r7a.48xlarge",
            "r7a.4xlarge",
            "r7a.8xlarge",
            "r7a.large",
            "r7a.medium",
            "r7a.metal-48xl",
            "r7a.xlarge",
            "r7g.12xlarge",
            "r7g.16xlarge",
            "r7g.2xlarge",
            "r7g.4xlarge",
            "r7g.8xlarge",
            "r7g.large",
            "r7g.medium",
            "r7g.metal",
            "r7g.xlarge",
            "r7gd.12xlarge",
            "r7gd.16xlarge",
            "r7gd.2xlarge",
            "r7gd.4xlarge",
            "r7gd.8xlarge",
            "r7gd.large",
            "r7gd.medium",
            "r7gd.xlarge",
            "r7i.12xlarge",
            "r7i.16xlarge",
            "r7i.24xlarge",
            "r7i.2xlarge",
            "r7i.48xlarge",
            "r7i.4xlarge",
            "r7i.8xlarge",
            "r7i.large",
            "r7i.xlarge",
            "r7iz.12xlarge",
            "r7iz.16xlarge",
            "r7iz.2xlarge",
            "r7iz.32xlarge",
            "r7iz.4xlarge",
            "r7iz.8xlarge",
            "r7iz.large",
            "r7iz.xlarge",
            "t1.micro",
            "t2.2xlarge",
            "t2.large",
            "t2.medium",
            "t2.micro",
            "t2.nano",
            "t2.small",
            "t2.xlarge",
            "t3.2xlarge",
            "t3.large",
            "t3.medium",
            "t3.micro",
            "t3.nano",
            "t3.small",
            "t3.xlarge",
            "t3a.2xlarge",
            "t3a.large",
            "t3a.medium",
            "t3a.micro",
            "t3a.nano",
            "t3a.small",
            "t3a.xlarge",
            "t4g.2xlarge",
            "t4g.large",
            "t4g.medium",
            "t4g.micro",
            "t4g.nano",
            "t4g.small",
            "t4g.xlarge",
            "trn1.2xlarge",
            "trn1.32xlarge",
            "trn1n.32xlarge",
            "u-12tb1.112xlarge",
            "u-12tb1.metal",
            "u-18tb1.112xlarge",
            "u-18tb1.metal",
            "u-24tb1.112xlarge",
            "u-24tb1.metal",
            "u-3tb1.56xlarge",
            "u-6tb1.112xlarge",
            "u-6tb1.56xlarge",
            "u-6tb1.metal",
            "u-9tb1.112xlarge",
            "u-9tb1.metal",
            "vt1.24xlarge",
            "vt1.3xlarge",
            "vt1.6xlarge",
            "x1.16xlarge",
            "x1.32xlarge",
            "x1e.16xlarge",
            "x1e.2xlarge",
            "x1e.32xlarge",
            "x1e.4xlarge",
            "x1e.8xlarge",
            "x1e.xlarge",
            "x2gd.12xlarge",
            "x2gd.16xlarge",
            "x2gd.2xlarge",
            "x2gd.4xlarge",
            "x2gd.8xlarge",
            "x2gd.large",
            "x2gd.medium",
            "x2gd.metal",
            "x2gd.xlarge",
            "x2idn.16xlarge",
            "x2idn.24xlarge",
            "x2idn.32xlarge",
            "x2idn.metal",
            "x2iedn.16xlarge",
            "x2iedn.24xlarge",
            "x2iedn.2xlarge",
            "x2iedn.32xlarge",
            "x2iedn.4xlarge",
            "x2iedn.8xlarge",
            "x2iedn.metal",
            "x2iedn.xlarge",
            "x2iezn.12xlarge",
            "x2iezn.2xlarge",
            "x2iezn.4xlarge",
            "x2iezn.6xlarge",
            "x2iezn.8xlarge",
            "x2iezn.metal",
            "z1d.12xlarge",
            "z1d.2xlarge",
            "z1d.3xlarge",
            "z1d.6xlarge",
            "z1d.large",
            "z1d.metal",
            "z1d.xlarge",
        ]
    }
}

pub enum CpuArch {
    X86_64,
    Aarch64,
}

impl CpuArch {
    pub fn get_ubuntu_arch_identifier(&self) -> &'static str {
        match self {
            CpuArch::X86_64 => "amd64",
            CpuArch::Aarch64 => "arm64",
        }
    }
}

pub fn get_arch_of_instance_type(instance_type: InstanceType) -> CpuArch {
    // Instance names look something like:
    // type + revision_number + subtypes + '.' + size
    // So say for example `Im4gn.large` would be split into:
    // type = "Im"
    // revision_number = 4
    // subtypes = "gn"
    // size = "large"
    //
    // The 'g' character existing in subtypes indicates that the instance type is a gravitron aka arm instance.
    // We can check for the existence of 'g' to determine if we are aarch64 or x86_64
    // This is a bit hacky because this format is not explicitly documented anywhere but the instance type naming does consistently follow this pattern.
    let mut reached_revision_number = false;
    for c in instance_type.as_str().chars() {
        if !reached_revision_number {
            if c.is_ascii_digit() {
                reached_revision_number = true;
            }
        } else if c == '.' {
            return CpuArch::X86_64;
        } else if c == 'g' {
            return CpuArch::Aarch64;
        }
    }
    unreachable!("Cannot parse instance type: {instance_type:?}")
}
