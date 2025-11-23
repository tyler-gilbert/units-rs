#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod imperial;
mod test;

use bincode::{Decode, Encode};
use units_si::{
    SiAddSubtract, SiDisplay, SiDivide, SiInvert, SiMultiply, SiMultiplyDivideScalar, SiSquare,
};

// These are used with the macros in units-proc-macro
#[cfg(feature = "f32")]
pub type NativeType = f32;
#[cfg(not(feature = "f32"))]
pub type NativeType = f64;

pub const SIGNIFICANT_FIGURES: i32 = 12;

#[derive(Copy, Clone, Decode, Encode)]
pub enum DecibelType {
    Power,
    Signal,
}

impl Into<NativeType> for DecibelType {
    fn into(self) -> NativeType {
        match self {
            DecibelType::Power => 10.0,
            DecibelType::Signal => 20.0,
        }
    }
}

#[derive(Copy, Clone, Decode, Encode)]
pub struct Decibel<UnitType: Into<NativeType> + Copy + core::ops::Mul<Scalar, Output = UnitType>> {
    value: NativeType, //dB ratio value
    reference: UnitType,
    multiplier: DecibelType,
}

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<Scalar, Output = UnitType>>
    Decibel<UnitType>
{
    pub fn new_from_raw(value: UnitType, reference: UnitType, multiplier: DecibelType) -> Self {
        let ratio: NativeType = value.into() / reference.into();
        let mult: NativeType = multiplier.into();

        #[cfg(feature = "f32")]
        let value = mult * libm::log10f(ratio);
        #[cfg(not(feature = "f32"))]
        let value = mult * libm::log10(ratio);

        Self {
            value,
            reference,
            multiplier,
        }
    }

    pub fn new(ratio: NativeType, reference: UnitType, multiplier: DecibelType) -> Self {
        Self {
            value: ratio,
            reference,
            multiplier,
        }
    }

    pub fn ratio(&self) -> Scalar {
        let mult: NativeType = self.multiplier.into();

        #[cfg(feature = "f32")]
        {
            Scalar::from(libm::powf(10.0, self.value / mult))
        }
        #[cfg(not(feature = "f32"))]
        {
            Scalar::from(libm::pow(10.0, self.value / mult))
        }
    }

    pub fn to_units(&self) -> UnitType {
        self.reference * self.ratio()
    }
}

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<Scalar, Output = UnitType>>
    core::ops::Add<Decibel<UnitType>> for Decibel<UnitType>
{
    type Output = Decibel<UnitType>;

    fn add(self, rhs: Decibel<UnitType>) -> Self::Output {
        let value = self.value + rhs.value;
        Self {
            value,
            reference: self.reference,
            multiplier: self.multiplier,
        }
    }
}

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<Scalar, Output = UnitType>>
    core::ops::Sub<Decibel<UnitType>> for Decibel<UnitType>
{
    type Output = Decibel<UnitType>;

    fn sub(self, rhs: Decibel<UnitType>) -> Self::Output {
        let value = self.value - rhs.value;
        Self {
            value,
            reference: self.reference,
            multiplier: self.multiplier,
        }
    }
}

impl<
        UnitType: Into<NativeType> + From<NativeType> + Copy + core::ops::Mul<Scalar, Output = UnitType>,
    > core::ops::Mul<UnitType> for Decibel<UnitType>
{
    type Output = UnitType;

    fn mul(self, rhs: UnitType) -> UnitType {
        UnitType::from(self.ratio().native * rhs.into())
    }
}

// Unitless
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Scalar {
    native: NativeType,
}

impl Scalar {
    pub fn sqrt(&self) -> Self {
        #[cfg(feature = "f32")]
        {
            Self::from(libm::sqrtf(self.native))
        }

        #[cfg(not(feature = "f32"))]
        {
            Self::from(libm::sqrt(self.native))
        }
    }

    pub fn atan2(&self, other: Scalar) -> Self {
        #[cfg(feature = "f32")]
        {
            Self::from(libm::atan2f(self.native, other.native))
        }

        #[cfg(not(feature = "f32"))]
        {
            Self::from(libm::atan2(self.native, other.native))
        }
    }

    pub fn log(&self) -> Self {
        #[cfg(feature = "f32")]
        {
            Self::from(libm::logf(self.native))
        }

        #[cfg(not(feature = "f32"))]
        {
            Self::from(libm::log(self.native))
        }
    }
}

// Mechanical

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct Length {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiInvert, SiDisplay, Decode, Encode,
)]
#[parameters(inv = Length)]
pub struct LengthInverse {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiSquare, SiDisplay, Decode, Encode,
)]
#[parameters(square = Length)]
pub struct Area {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiMultiply, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_mult = Area, rhs_mult = Length)]
pub struct Volume {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct OrthogonalLength {
    native: NativeType,
}

impl From<Length> for OrthogonalLength {
    fn from(value: Length) -> OrthogonalLength {
        OrthogonalLength::from(value.native)
    }
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct Time {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiInvert, SiDisplay, Decode, Encode,
)]
#[parameters(inv = Time)]
pub struct Frequency {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiSquare, SiDisplay, Decode, Encode,
)]
#[parameters(square = Frequency)]
pub struct FrequencySquared {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct Mass {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Mass, rhs_div = Volume)]
pub struct MassDensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct PlaneAngle {
    native: NativeType,
}

impl PlaneAngle {
    pub fn sin(&self) -> Scalar {
        #[cfg(feature = "f32")]
        {
            Scalar::from(libm::sinf(self.native))
        }

        #[cfg(not(feature = "f32"))]
        {
            Scalar::from(libm::sin(self.native))
        }
    }

    pub fn cos(&self) -> Scalar {
        #[cfg(feature = "f32")]
        {
            Scalar::from(libm::cosf(self.native))
        }

        #[cfg(not(feature = "f32"))]
        {
            Scalar::from(libm::cos(self.native))
        }
    }
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiInvert, SiDisplay, Decode, Encode,
)]
#[parameters(inv = PlaneAngle)]
pub struct PlaneAngleInverse {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct SolidAngle {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Length, rhs_div = Time)]
pub struct Velocity {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiSquare, SiDisplay, Decode, Encode,
)]
#[parameters(square = Velocity)]
pub struct VelocitySquared {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Velocity, rhs_div = Time)]
pub struct Acceleration {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Acceleration, rhs_div = Time)]
pub struct Jerk {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiMultiply, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_mult = Mass, rhs_mult = Acceleration)]
pub struct Force {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Force, rhs_div = Area)]
pub struct Pressure {
    native: NativeType,
}

// Electrical

#[derive(
    Copy,
    Clone,
    SiAddSubtract,
    SiMultiplyDivideScalar,
    SiMultiply,
    SiDivide,
    SiDisplay,
    Decode,
    Encode,
)]
#[parameters(lhs_mult = Energy, rhs_mult = Frequency, lhs_div = Energy, rhs_div = Time)]
pub struct Power {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiMultiply, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_mult = Force, rhs_mult = Length)]
pub struct Energy {
    native: NativeType,
}

#[derive(
    Copy,
    Clone,
    SiAddSubtract,
    SiMultiplyDivideScalar,
    SiMultiply,
    SiDivide,
    SiDisplay,
    Decode,
    Encode,
)]
#[parameters(lhs_mult = Energy, rhs_mult = Time, lhs_div = Energy, rhs_div = Frequency)]
pub struct EnergyPerFrequency {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = Power, rhs_div = ElectricCurrent)]
pub struct ElectricPotential {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct ElectricCurrent {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = Time)]
pub struct ElectricCharge {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = ElectricCharge, rhs_div = ElectricPotential)]
pub struct Capacitance {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = ElectricPotential, rhs_div = ElectricCurrent)]
pub struct ElectricResistance {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = ElectricPotential)]
pub struct ElectricConductance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct MagneticFlux {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = MagneticFlux, rhs_div = Area)]
pub struct MagneticFluxDensity {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDivide, SiDisplay, Decode, Encode,
)]
#[parameters(lhs_div = MagneticFlux, rhs_div = ElectricCurrent)]
pub struct Inductance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct ThermodynamicTemperature {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct Temperature {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, Decode, Encode)]
pub struct AmountOfSubstance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiInvert, SiDisplay, Decode, Encode)]
#[parameters(inv = AmountOfSubstance)]
pub struct PerAmountOfSubstance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct LuminousIntensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = LuminousIntensity, rhs_mult = SolidAngle)]
pub struct LuminousFlux {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = LuminousIntensity, rhs_div = Area)]
pub struct Illuminance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Pressure, rhs_mult = Time)]
pub struct DynamicViscosity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Force, rhs_mult = OrthogonalLength)]
pub struct MomentOfForce {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Torque {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = PlaneAngle, rhs_mult = Frequency, lhs_div = PlaneAngle, rhs_div = Time)]
pub struct AngularVelocity {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiSquare, SiDisplay, Decode, Encode,
)]
#[parameters(square = AngularVelocity)]
pub struct AngularVelocitySquared {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = AngularVelocity, rhs_mult = Frequency, lhs_div = AngularVelocity, rhs_div = Time)]
pub struct AngularAcceleration {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Force, rhs_div = Length)]
pub struct SurfaceTension {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Power, rhs_div = Area)]
pub struct HeatFluxDensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = ThermodynamicTemperature)]
pub struct HeatCapacity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = MassThermodynamicTemperature)]
pub struct SpecificHeatCapacity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = Mass)]
pub struct SpecificEnergy {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = Volume)]
pub struct EnergyDensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricPotential, rhs_div = Length)]
pub struct ElectricFieldStrength {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricCharge, rhs_div = Area)]
pub struct ElectricFluxDensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = ElectricFluxDensity, rhs_mult = Length, lhs_div = ElectricCharge, rhs_div = Volume)]
pub struct ElectricChargeDensity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Capacitance, rhs_div = Length)]
pub struct Permittivity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Inductance, rhs_div = Length)]
pub struct Permeability {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = AmountOfSubstance)]
pub struct MolarEnergy {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = AmountOfSubstance, rhs_mult = ThermodynamicTemperature)]
pub struct AmountOfSubstanceThermodynamicTemperature {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = AmountOfSubstanceThermodynamicTemperature)]
pub struct MolarHeatCapacity {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Area, rhs_mult = SolidAngle)]
pub struct AreaSolidAngle {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Power, rhs_div = AreaSolidAngle)]
pub struct Radiance {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Mass, rhs_mult = ThermodynamicTemperature)]
pub struct MassThermodynamicTemperature {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Length, rhs_mult = ThermodynamicTemperature)]
pub struct LengthThermodynamicTemperature {
    native: NativeType,
}

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Length, rhs_div = LengthThermodynamicTemperature)]
pub struct ThermalConductivity {
    native: NativeType,
}

#[allow(non_upper_case_globals)]
pub mod constants {
    use super::*;

    /// Speed of light in a vacuum
    pub const c: Velocity = Velocity::new(299_792_458.0 as NativeType);
    /// Planck constant
    pub const h: EnergyPerFrequency = EnergyPerFrequency::new(6.62607015E-34 as NativeType);
    /// Sound pressure level of 0 dB
    pub const p0: Pressure = Pressure::new(2.0E-5 as NativeType);
    /// Elementary charge
    pub const e: ElectricCharge = ElectricCharge::new(1.602176634E-19 as NativeType);
    /// Boltzmann constant
    pub const k: HeatCapacity = HeatCapacity::new(1.380649E-23 as NativeType);
    /// Avogadro constant
    pub const N_A: PerAmountOfSubstance = PerAmountOfSubstance::new(6.02214076E23 as NativeType);
    /// the luminous efficacy of monochromatic radiation of frequency 540 × 1012 hertz
    pub const K_cd: LuminousFlux = LuminousFlux::new(683.0 as NativeType);
    /// electron mass
    pub const m_e: Mass = Mass::new(9.1093837015E-31 as NativeType);
    /// Proton mass
    pub const m_p: Mass = Mass::new(1.67262192369E-27 as NativeType);
    /// neutron mass
    pub const m_n: Mass = Mass::new(1.67492749804E-27 as NativeType);
    /// muon mass
    pub const m_mu: Mass = Mass::new(1.883531627E-28 as NativeType);
    /// tau mass
    pub const m_tau: Mass = Mass::new(3.16754E-27 as NativeType);
    /// gravity
    pub const g: Acceleration = Acceleration::new(9.80665 as NativeType);
    /// characteristic impedance of vacuum
    pub const Z_0: ElectricResistance = ElectricResistance::new(376.730313668 as NativeType);
    /// Wien wavelength displacement law constant
    pub const b: LengthThermodynamicTemperature =
        LengthThermodynamicTemperature::new(2.897771955E-3 as NativeType);
    /// Wien entropy displacement law constant
    pub const b_entropy: LengthThermodynamicTemperature =
        LengthThermodynamicTemperature::new(3.002916077E-3 as NativeType);
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    use std::any::{Any, TypeId};

    use crate::{basic, divide, invert, multiply};

    #[test]
    fn config_operations() {
        let native_type = if TypeId::of::<NativeType>() == TypeId::of::<f32>() {
            "f32"
        } else {
            "f64"
        };
        println!("NativeType: {}", native_type);
        println!("SIGNIFICANT_FIGURES: {}", SIGNIFICANT_FIGURES);
    }

    basic!(test_length, Length);
    basic!(test_area, Area);
    multiply!(test_multiply_area, Area, Length, Length);
    basic!(test_volume, Volume);
    multiply!(test_multiply_volume, Volume, Area, Length);
    basic!(test_orthogonal_length, OrthogonalLength);
    basic!(test_time, Time);
    basic!(test_frequency, Frequency);
    invert!(test_invert_frequency, Frequency, Time);
    basic!(test_mass, Mass);
    basic!(test_mass_density, MassDensity);
    divide!(test_divide_mass_density, MassDensity, Mass, Volume);
    basic!(test_plane_angle, PlaneAngle);
    basic!(test_solid_angle, SolidAngle);
    basic!(test_velocity, Velocity);
    divide!(test_multiply_velocity, Velocity, Length, Time);
    multiply!(test_multiply_velocity_alt0, Length, Time, Velocity);
    multiply!(test_multiply_velocity_alt1, Length, Velocity, Time);
    basic!(test_acceleration, Acceleration);
    divide!(test_divide_acceleration, Acceleration, Velocity, Time);
    basic!(test_force, Force);
    multiply!(test_multiply_force, Force, Mass, Acceleration);
    basic!(test_pressure, Pressure);
    divide!(test_divide_pressure, Pressure, Force, Area);
    basic!(test_power, Power);
    multiply!(test_multiply_power, Power, Energy, Frequency);
    divide!(test_divide_power, Power, Energy, Time);
    basic!(test_energy, Energy);
    multiply!(test_multiply_energy, Energy, Force, Length);
    basic!(test_energy_per_frequency, EnergyPerFrequency);
    multiply!(
        test_multiply_energy_per_frequency,
        EnergyPerFrequency,
        Energy,
        Time
    );
    divide!(
        test_divide_energy_per_frequency,
        EnergyPerFrequency,
        Energy,
        Frequency
    );
    basic!(test_electric_potential, ElectricPotential);
    divide!(
        test_divide_electric_potential,
        ElectricPotential,
        Power,
        ElectricCurrent
    );
    basic!(test_electric_current, ElectricCurrent);
    basic!(test_electric_charge, ElectricCharge);
    divide!(
        test_divide_electric_charge,
        ElectricCharge,
        ElectricCurrent,
        Time
    );
    basic!(test_capacitance, Capacitance);
    divide!(
        test_divide_capacitance,
        Capacitance,
        ElectricCharge,
        ElectricPotential
    );
    basic!(test_electric_resistance, ElectricResistance);
    divide!(
        test_divide_electric_resistance,
        ElectricResistance,
        ElectricPotential,
        ElectricCurrent
    );
    basic!(test_electric_conductance, ElectricConductance);
    divide!(
        test_divide_electric_conductance,
        ElectricConductance,
        ElectricCurrent,
        ElectricPotential
    );
    basic!(test_magnetic_flux, MagneticFlux);
    basic!(test_magnetic_flux_density, MagneticFluxDensity);
    divide!(
        test_divide_magnetic_flux_density,
        MagneticFluxDensity,
        MagneticFlux,
        Area
    );
    basic!(test_inductance, Inductance);
    divide!(
        test_divide_inductance,
        Inductance,
        MagneticFlux,
        ElectricCurrent
    );
    basic!(test_thermodynamic_temperature, ThermodynamicTemperature);
    basic!(test_temperature, Temperature);
    basic!(test_amount_of_substance, AmountOfSubstance);
    invert!(
        test_invert_amount_of_substance,
        PerAmountOfSubstance,
        AmountOfSubstance
    );
    basic!(test_luminous_intensity, LuminousIntensity);
    basic!(test_luminous_flux, LuminousFlux);
    multiply!(
        test_multiply_luminous_flux,
        LuminousFlux,
        LuminousIntensity,
        SolidAngle
    );
    basic!(test_illuminance, Illuminance);
    divide!(
        test_divide_illuminance,
        Illuminance,
        LuminousIntensity,
        Area
    );
    basic!(test_dynamic_viscosity, DynamicViscosity);
    multiply!(
        test_multiply_dynamic_viscosity,
        DynamicViscosity,
        Pressure,
        Time
    );
    basic!(test_moment_of_force, MomentOfForce);
    multiply!(
        test_multiply_moment_of_force,
        MomentOfForce,
        Force,
        OrthogonalLength
    );
    basic!(test_angular_velocity, AngularVelocity);
    multiply!(
        test_multiply_angular_velocity,
        AngularVelocity,
        PlaneAngle,
        Frequency
    );
    divide!(
        test_divide_angular_velocity,
        AngularVelocity,
        PlaneAngle,
        Time
    );
    basic!(test_angular_acceleration, AngularAcceleration);
    multiply!(
        test_multiply_angular_acceleration,
        AngularAcceleration,
        AngularVelocity,
        Frequency
    );
    divide!(
        test_divide_angular_acceleration,
        AngularAcceleration,
        AngularVelocity,
        Time
    );
    basic!(test_surface_tension, SurfaceTension);
    divide!(test_divide_surface_tension, SurfaceTension, Force, Length);
    basic!(test_heat_flux_density, HeatFluxDensity);
    divide!(test_divide_heat_flux_density, HeatFluxDensity, Power, Area);
    basic!(test_heat_capacity, HeatCapacity);
    divide!(
        test_divide_heat_capacity,
        HeatCapacity,
        Energy,
        ThermodynamicTemperature
    );
    basic!(test_specific_heat_capacity, SpecificHeatCapacity);
    divide!(
        test_divide_specific_heat_capacity,
        SpecificHeatCapacity,
        Energy,
        MassThermodynamicTemperature
    );
    basic!(test_specific_energy, SpecificEnergy);
    divide!(test_divide_specific_energy, SpecificEnergy, Energy, Mass);
    basic!(test_energy_density, EnergyDensity);
    divide!(test_divide_energy_density, EnergyDensity, Energy, Volume);
    basic!(test_electric_field_strength, ElectricFieldStrength);
    divide!(
        test_divide_electric_field_strength,
        ElectricFieldStrength,
        ElectricPotential,
        Length
    );
    basic!(test_electric_flux_density, ElectricFluxDensity);
    divide!(
        test_divide_electric_flux_density,
        ElectricFluxDensity,
        ElectricCharge,
        Area
    );
    basic!(test_electric_charge_density, ElectricChargeDensity);
    divide!(
        test_divide_electric_charge_density,
        ElectricChargeDensity,
        ElectricCharge,
        Volume
    );
    multiply!(
        test_multiply_electric_charge_density,
        ElectricChargeDensity,
        ElectricFluxDensity,
        Length
    );
    basic!(test_permittivity, Permittivity);
    divide!(test_divide_permittivity, Permittivity, Capacitance, Length);
    basic!(test_permeability, Permeability);
    divide!(test_divide_permeability, Permeability, Inductance, Length);
    basic!(test_molar_energy, MolarEnergy);
    divide!(
        test_divide_molar_energy,
        MolarEnergy,
        Energy,
        AmountOfSubstance
    );
    basic!(
        test_amount_of_substance_thermodynamic_temperature,
        AmountOfSubstanceThermodynamicTemperature
    );
    multiply!(
        test_multiply_amount_of_substance_thermodynamic_temperature,
        AmountOfSubstanceThermodynamicTemperature,
        AmountOfSubstance,
        ThermodynamicTemperature
    );
    basic!(test_molar_heat_capacity, MolarHeatCapacity);
    divide!(
        test_divide_molar_heat_capacity,
        MolarHeatCapacity,
        Energy,
        AmountOfSubstanceThermodynamicTemperature
    );
    basic!(test_area_solid_angle, AreaSolidAngle);
    multiply!(
        test_multiply_area_solid_angle,
        AreaSolidAngle,
        Area,
        SolidAngle
    );
    basic!(test_radiance, Radiance);
    divide!(test_divide_radiance, Radiance, Power, AreaSolidAngle);
    basic!(
        test_mass_thermodynamic_temperature,
        MassThermodynamicTemperature
    );
    multiply!(
        test_multiply_mass_thermodynamic_temperature,
        MassThermodynamicTemperature,
        Mass,
        ThermodynamicTemperature
    );
    basic!(
        test_length_thermodynamic_temperature,
        LengthThermodynamicTemperature
    );
    multiply!(
        test_multiply_length_thermodynamic_temperature,
        LengthThermodynamicTemperature,
        Length,
        ThermodynamicTemperature
    );
    basic!(test_thermal_conductivity, ThermalConductivity);
    divide!(
        test_divide_thermal_conductivity,
        ThermalConductivity,
        Length,
        LengthThermodynamicTemperature
    );

    #[test]
    fn scalar_operations() {
        assert_eq!(Power::new(1.0) * Scalar::new(1000.), Power::new(1000.));
        let db = Decibel::new_from_raw(Power::new(5.), Power::new(1.), DecibelType::Power);
        assert_eq!(db.to_units(), Power::new(5.));
        let db = Decibel::new_from_raw(
            ElectricPotential::new(50.),
            ElectricPotential::new(1.),
            DecibelType::Signal,
        );
        assert_eq!(db.to_units(), ElectricPotential::new(50.));
    }

    #[test]
    fn readme_test() {
        let length = Length::new(32.);
        let duration = Time::new(0.01);
        let velocity = length / duration;
        println!("Velocity: {}", velocity);

        let current = ElectricCurrent::new(10.);
        let resistance = ElectricResistance::new(100.);
        let voltage = current * resistance;
        println!("Voltage: {}", voltage);

        //doing and ADC conversion
        let adc_value = 100_u16;
        let volts_per_bit = ElectricPotential::new(0.004);
        let v_out = Scalar::from(adc_value as NativeType) * volts_per_bit;
        let r_top = ElectricResistance::new(4000.);
        let r_bottom = ElectricResistance::new(1000.);
        //calculate the input voltage before a voltage divider
        let sensor_voltage = v_out * ((r_top + r_bottom) / r_bottom);
        println!("Sensor Voltage: {}", sensor_voltage);
    }

    #[test]
    fn serde_test() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Values {
            volts: ElectricPotential,
            current: ElectricCurrent,
            charge: ElectricCharge,
        }

        let values = Values {
            volts: ElectricPotential::new(10.0),
            current: ElectricCurrent::new(100.0),
            charge: ElectricCharge::new(1000.0),
        };

        let json_string =
            serde_json::to_string_pretty(&values).expect("Failed to create a json string");

        let value: serde_json::Value =
            serde_json::from_str(&json_string).expect("Failed to parse JSON string");

        let map = value.as_object().expect("Not a map");
        let volt_string = map
            .get("volts")
            .expect("volts missing")
            .as_str()
            .expect("Not a string");
        assert_eq!(volt_string, "10_volts");

        let volt_string = map
            .get("current")
            .expect("current missing")
            .as_str()
            .expect("Not a string");
        assert_eq!(volt_string, "100_amps");

        let volt_string = map
            .get("charge")
            .expect("charge missing")
            .as_str()
            .expect("Not a string");
        assert_eq!(volt_string, "1000_coulombs");

        let deserde_values: Values = serde_json::from_str(&json_string).expect("Failed to deserde");

        assert_eq!(values.volts, deserde_values.volts);
        assert_eq!(values.current, deserde_values.current);
        assert_eq!(values.charge, deserde_values.charge);
    }
}
