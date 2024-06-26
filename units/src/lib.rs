#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use bincode::{Decode, Encode};
use units_si::{SiAddSubtract, SiDisplay, SiDivide, SiInvert, SiMultiply, SiSquare};

pub use units_si::si;

// These are used with the macros in units-proc-macro
#[cfg(feature = "f32")]
pub type NativeType = f32;
#[cfg(not(feature = "f32"))]
pub type NativeType = f64;

pub const SIGNIFICANT_FIGURES: i32 = 6;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Decode, Encode)]
pub struct Decibel<
    UnitType: Into<NativeType> + Copy + core::ops::Mul<NativeType, Output = UnitType>,
> {
    value: NativeType, //dB ratio value
    reference: UnitType,
    multiplier: DecibelType,
}

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<NativeType, Output = UnitType>>
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

    pub fn ratio(&self) -> NativeType {
        let mult: NativeType = self.multiplier.into();

        #[cfg(feature = "f32")]
        {
            libm::powf(10.0, self.value / mult)
        }
        #[cfg(not(feature = "f32"))]
        {
            libm::pow(10.0, self.value / mult)
        }
    }

    pub fn to_units(&self) -> UnitType {
        self.reference * self.ratio()
    }
}

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<NativeType, Output = UnitType>>
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

impl<UnitType: Into<NativeType> + Copy + core::ops::Mul<NativeType, Output = UnitType>>
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
        UnitType: Into<NativeType> + From<NativeType> + Copy + core::ops::Mul<NativeType, Output = UnitType>,
    > core::ops::Mul<UnitType> for Decibel<UnitType>
{
    type Output = UnitType;

    fn mul(self, rhs: UnitType) -> UnitType {
        UnitType::from(self.ratio() * rhs.into())
    }
}

// Mechanical
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Length(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiSquare, SiDisplay, Decode, Encode)]
#[parameters(square = Length)]
pub struct Area(pub NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Area, rhs_mult = Length)]
pub struct Volume(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct OrthogonalLength(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Time(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiInvert, SiDisplay, Decode, Encode)]
#[parameters(inv = Time)]
pub struct Frequency(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Mass(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Mass, rhs_div = Volume)]
pub struct MassDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct PlaneAngle(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct SolidAngle(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Length, rhs_div = Time)]
pub struct Velocity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Velocity, rhs_div = Time)]
pub struct Acceleration(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Mass, rhs_mult = Acceleration)]
pub struct Force(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Force, rhs_div = Area)]
pub struct Pressure(pub NativeType);

// Electrical
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Energy, rhs_mult = Frequency, lhs_div = Energy, rhs_div = Time)]
pub struct Power(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Force, rhs_mult = Length)]
pub struct Energy(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Energy, rhs_mult = Time, lhs_div = Energy, rhs_div = Frequency)]
pub struct EnergyPerFrequency(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Power, rhs_div = ElectricCurrent)]
pub struct ElectricPotential(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct ElectricCurrent(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = Time)]
pub struct ElectricCharge(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricCharge, rhs_div = ElectricPotential)]
pub struct Capacitance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricPotential, rhs_div = ElectricCurrent)]
pub struct ElectricResistance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = ElectricPotential)]
pub struct ElectricConductance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct MagneticFlux(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = MagneticFlux, rhs_div = Area)]
pub struct MagneticFluxDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = MagneticFlux, rhs_div = ElectricCurrent)]
pub struct Inductance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct ThermodynamicTemperature(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct Temperature(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct AmountOfSubstance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiInvert, SiDisplay, Decode, Encode)]
#[parameters(inv = AmountOfSubstance)]
pub struct PerAmountOfSubstance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDisplay, Decode, Encode)]
pub struct LuminousIntensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = LuminousIntensity, rhs_mult = SolidAngle)]
pub struct LuminousFlux(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = LuminousIntensity, rhs_div = Area)]
pub struct Illuminance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Pressure, rhs_mult = Time)]
pub struct DynamicViscosity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Force, rhs_mult = OrthogonalLength)]
pub struct MomentOfForce(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = PlaneAngle, rhs_mult = Frequency, lhs_div = PlaneAngle, rhs_div = Time)]
pub struct AngularVelocity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = AngularVelocity, rhs_mult = Frequency, lhs_div = AngularVelocity, rhs_div = Time)]
pub struct AngularAcceleration(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Force, rhs_div = Length)]
pub struct SurfaceTension(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Power, rhs_div = Area)]
pub struct HeatFluxDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = ThermodynamicTemperature)]
pub struct HeatCapacity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = MassThermodynamicTemperature)]
pub struct SpecificHeatCapacity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = Mass)]
pub struct SpecificEnergy(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = Volume)]
pub struct EnergyDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricPotential, rhs_div = Length)]
pub struct ElectricFieldStrength(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = ElectricCharge, rhs_div = Area)]
pub struct ElectricFluxDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = ElectricFluxDensity, rhs_mult = Length, lhs_div = ElectricCharge, rhs_div = Volume)]
pub struct ElectricChargeDensity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Capacitance, rhs_div = Length)]
pub struct Permittivity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Inductance, rhs_div = Length)]
pub struct Permeability(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = AmountOfSubstance)]
pub struct MolarEnergy(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = AmountOfSubstance, rhs_mult = ThermodynamicTemperature)]
pub struct AmountOfSubstanceThermodynamicTemperature(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Energy, rhs_div = AmountOfSubstanceThermodynamicTemperature)]
pub struct MolarHeatCapacity(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Area, rhs_mult = SolidAngle)]
pub struct AreaSolidAngle(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Power, rhs_div = AreaSolidAngle)]
pub struct Radiance(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Mass, rhs_mult = ThermodynamicTemperature)]
pub struct MassThermodynamicTemperature(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay, Decode, Encode)]
#[parameters(lhs_mult = Length, rhs_mult = ThermodynamicTemperature)]
pub struct LengthThermodynamicTemperature(pub NativeType);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay, Decode, Encode)]
#[parameters(lhs_div = Length, rhs_div = LengthThermodynamicTemperature)]
pub struct ThermalConductivity(pub NativeType);

#[allow(non_upper_case_globals)]
pub mod constants {
    use super::*;

    /// Speed of light in a vacuum
    pub const c: Velocity = Velocity(299_792_458.0 as NativeType);
    /// Planck constant
    pub const h: EnergyPerFrequency = EnergyPerFrequency(6.62607015E-34 as NativeType);
    /// Sound pressure level of 0 dB
    pub const p0: Pressure = Pressure(2.0E-5 as NativeType);
    /// Elementary charge
    pub const e: ElectricCharge = ElectricCharge(1.602176634E-19 as NativeType);
    /// Boltzmann constant
    pub const k: HeatCapacity = HeatCapacity(1.380649E-23 as NativeType);
    /// Avogadro constant
    pub const N_A: PerAmountOfSubstance = PerAmountOfSubstance(6.02214076E23 as NativeType);
    /// the luminous efficacy of monochromatic radiation of frequency 540 × 1012 hertz
    pub const K_cd: LuminousFlux = LuminousFlux(683.0 as NativeType);
    /// electron mass
    pub const m_e: Mass = Mass(9.1093837015E-31 as NativeType);
    /// Proton mass
    pub const m_p: Mass = Mass(1.67262192369E-27 as NativeType);
    /// neutron mass
    pub const m_n: Mass = Mass(1.67492749804E-27 as NativeType);
    /// muon mass
    pub const m_mu: Mass = Mass(1.883531627E-28 as NativeType);
    /// tau mass
    pub const m_tau: Mass = Mass(3.16754E-27 as NativeType);
    /// gravity
    pub const g: Acceleration = Acceleration(9.80665 as NativeType);
    /// characteristic impedance of vacuum
    pub const Z_0: ElectricResistance = ElectricResistance(376.730313668 as NativeType);
    /// Wien wavelength displacement law constant
    pub const b: LengthThermodynamicTemperature =
        LengthThermodynamicTemperature(2.897771955E-3 as NativeType);
    /// Wien entropy displacement law constant
    pub const b_entropy: LengthThermodynamicTemperature =
        LengthThermodynamicTemperature(3.002916077E-3 as NativeType);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::any::{Any, TypeId};
    use units_si::si;

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

    macro_rules! basic {
        ($name: ident, $type_name:ident) => {
            #[test]
            fn $name() {
                let v0 = $type_name(5.0 as NativeType);
                println!("v0 = {}", v0);
                let mut v1 = v0;
                assert_eq!(v1, v0);
                v1 = $type_name(10.0 as NativeType);
                assert!(v0 < v1);
                assert!(v1 > v0);
                assert!(v0 <= v1);
                assert!(v1 >= v0);
                let mut v2 = v0.clone();
                assert_eq!(v0, v2);
                v2 = $type_name(20.0 as NativeType);
                let v3 = v1 + v2;
                let v4 = (v3 - v0) * (40.0 as NativeType);
                assert_ne!(v0, v1);
                assert_eq!(v3, $type_name(30.0 as NativeType));
                let v5 = v1 / v2;
                assert_eq!(TypeId::of::<NativeType>(), v5.type_id());
                println!("[{},{},{},{},{}]", v0, v1, v2, v3, v4);
            }
        };
    }

    macro_rules! invert {
        ($name: ident, $type_name:ident, $inverted_type_name: ident) => {
            #[test]
            fn $name() {
                let v0 = $type_name(5.0 as NativeType);
                println!("v0 = {}", v0);
                let v1 = (1.0 as NativeType) / v0;
                assert_eq!(TypeId::of::<$inverted_type_name>(), v1.type_id());
                let v2 = (1.0 as NativeType) / v1;
                assert_eq!(TypeId::of::<$type_name>(), v2.type_id());
            }
        };
    }

    macro_rules! divide {
        ($name: ident, $type_name:ident, $lhs: ident, $rhs: ident) => {
            #[test]
            fn $name() {
                let lhs = $lhs(100.0 as NativeType);
                let rhs = $rhs(10.0 as NativeType);
                let v0 = lhs / rhs;
                assert_eq!(v0, $type_name(10.0 as NativeType));
                let v1 = rhs * v0;
                assert_eq!(v1, $lhs(100.0 as NativeType));
                let v2 = v0 * rhs;
                assert_eq!(v2, $lhs(100.0 as NativeType));
            }
        };
    }

    macro_rules! multiply {
        ($name: ident, $type_name:ident, $lhs: ident, $rhs: ident) => {
            #[test]
            fn $name() {
                let lhs = $lhs(100.0 as NativeType);
                let rhs = $rhs(10.0 as NativeType);
                let v0 = lhs * rhs;
                assert_eq!(v0, $type_name(1000.0 as NativeType));
                let v1 = v0 / rhs;
                assert_eq!(v1, $lhs(100.0 as NativeType));
                let v2 = v0 / lhs;
                assert_eq!(v2, $rhs(10.0 as NativeType));
            }
        };
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
    fn suffix_operations() {
        let v0 = si!(1g);
        std::assert_eq!(v0, Mass(0.001 as NativeType));
        let v0 = si!(1kg);
        assert_eq!(v0, Mass(1.0 as NativeType));
        assert_eq!(TypeId::of::<Mass>(), v0.type_id());
        let v0 = si!(1s);
        assert_eq!(TypeId::of::<Time>(), v0.type_id());
        let v0 = si!(1A);
        assert_eq!(TypeId::of::<ElectricCurrent>(), v0.type_id());
        let v0 = si!(1K);
        assert_eq!(TypeId::of::<ThermodynamicTemperature>(), v0.type_id());
        let v0 = si!(1mol);
        assert_eq!(TypeId::of::<AmountOfSubstance>(), v0.type_id());
        let v0 = si!(1m);
        assert_eq!(TypeId::of::<Length>(), v0.type_id());
        let v0 = si!(1cd);
        assert_eq!(TypeId::of::<LuminousIntensity>(), v0.type_id());
        let v0 = si!(1rad);
        assert_eq!(TypeId::of::<PlaneAngle>(), v0.type_id());
        let v0 = si!(1sr);
        assert_eq!(TypeId::of::<SolidAngle>(), v0.type_id());
        let v0 = si!(1Hz);
        assert_eq!(TypeId::of::<Frequency>(), v0.type_id());
        let v0 = si!(1m2);
        assert_eq!(TypeId::of::<Area>(), v0.type_id());
        let v0 = si!(1m3);
        assert_eq!(TypeId::of::<Volume>(), v0.type_id());
        let v0 = si!(1mps);
        assert_eq!(TypeId::of::<Velocity>(), v0.type_id());
        let v0 = si!(1mps2);
        assert_eq!(TypeId::of::<Acceleration>(), v0.type_id());
        let v0 = si!(1Pa);
        assert_eq!(TypeId::of::<Pressure>(), v0.type_id());
        let v0 = si!(1J);
        assert_eq!(TypeId::of::<Energy>(), v0.type_id());
        let v0 = si!(1W);
        assert_eq!(TypeId::of::<Power>(), v0.type_id());
        let v0 = si!(1C);
        assert_eq!(TypeId::of::<ElectricCharge>(), v0.type_id());
        let v0 = si!(1V);
        assert_eq!(TypeId::of::<ElectricPotential>(), v0.type_id());
        let v0 = si!(1F);
        assert_eq!(TypeId::of::<Capacitance>(), v0.type_id());
        let v0 = si!(1ohms);
        assert_eq!(TypeId::of::<ElectricResistance>(), v0.type_id());
        let v0 = si!(1S);
        assert_eq!(TypeId::of::<ElectricConductance>(), v0.type_id());
        let v0 = si!(1Wb);
        assert_eq!(TypeId::of::<MagneticFlux>(), v0.type_id());
        let v0 = si!(1T);
        assert_eq!(TypeId::of::<MagneticFluxDensity>(), v0.type_id());
        let v0 = si!(1H);
        assert_eq!(TypeId::of::<Inductance>(), v0.type_id());
        let v0 = si!(1degreeC);
        assert_eq!(TypeId::of::<Temperature>(), v0.type_id());
        let v0 = si!(1lm);
        assert_eq!(TypeId::of::<LuminousFlux>(), v0.type_id());
        let v0 = si!(1lx);
        assert_eq!(TypeId::of::<Illuminance>(), v0.type_id());
        let v0 = si!(1Pas);
        assert_eq!(TypeId::of::<DynamicViscosity>(), v0.type_id());
        let v0 = si!(1Nm);
        assert_eq!(TypeId::of::<MomentOfForce>(), v0.type_id());
    }

    #[test]
    fn modifying_operations() {
        assert_eq!(Power(0.000_000_000_000_001 as NativeType), si!(1fW));
        assert_eq!(Power(0.000_000_000_001 as NativeType), si!(1pW));
        assert_eq!(Power(0.000_000_001 as NativeType), si!(1nW));
        assert_eq!(Power(0.000_001 as NativeType), si!(1uW));
        assert_eq!(Power(0.001 as NativeType), si!(1mW));
        assert_eq!(Power(1.0 as NativeType), si!(1W));
        assert_eq!(Power(1_000.0 as NativeType), si!(1kW));
        assert_eq!(Power(1_000_000.0 as NativeType), si!(1MW));
        assert_eq!(Power(1_000_000_000.0 as NativeType), si!(1GW));
        assert_eq!(Power(1E12 as NativeType), si!(1TW));
    }

    #[test]
    fn scalar_operations() {
        assert_eq!(Power(1.0 as NativeType) * 1000., si!(1kW));
        let db = Decibel::new(-3., si!(1W), DecibelType::Power);
        assert_eq!(db * si!(10W), si!(5011872uW));
        println!("10W * -3dB = {}", si!(10W) * db);
        let db = Decibel::new_from_raw(si!(5W), si!(1W), DecibelType::Power);
        assert_eq!(db.to_units(), si!(5W));
        let db = Decibel::new_from_raw(si!(50V), si!(1V), DecibelType::Signal);
        assert_eq!(db.to_units(), si!(50V));
    }

    #[test]
    fn rounding_operations() {
        let power = si!(50W);
        assert_eq!(TypeId::of::<Power>(), power.type_id());
        assert_eq!(power, si!(50W));
        assert_eq!(power, si!(50000mW));
        assert_eq!(power, si!(50000011uW));

        //These require SIGNIFICANT_FIGURES=6
        assert_ne!(power, si!(50000111uW));
        assert_eq!(si!(500mV), si!(500_000uV));
        assert_ne!(si!(500mV), si!(500_001uV));
        assert_eq!(si!(50mV), si!(50_000uV));
        assert_ne!(si!(50mV), si!(50_000_100nV));
        assert_eq!(si!(5mV), si!(5_000_001nV));
        assert_ne!(si!(5mV), si!(5_000_010nV));
    }

    #[test]
    fn readme_test() {
        let length = si!(32m);
        let duration = si!(10ms);
        let velocity = length / duration;
        println!("Velocity: {}", velocity);

        let voltage = si!(10A * 100ohms);
        println!("Voltage: {}", voltage);

        //doing and ADC conversion
        let adc_value = 100_u16;
        let volts_per_bit = si!(4mV);
        let v_out = (adc_value as NativeType) * volts_per_bit;
        let r_top = si!(4kohms);
        let r_bottom = si!(1kohms);
        //calculate the input voltage before a voltage divider
        let sensor_voltage = v_out * ((r_top + r_bottom) / r_bottom);
        println!("Sensor Voltage: {}", sensor_voltage);
    }
}
