extern crate units_proc_macro;

use std::fmt;
use units_proc_macro::{
    SiAddSubtract, SiDisplay, SiDivide, SiInvert, SiMultiply, SiSquare,
};

// These are used with the macros in units-proc-macro
type NativeType = f64;
const SIGNIFICANT_FIGURES: i32 = 6;

// Mechanical
#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Length(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct OrthogonalLength(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Time(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiInvert, SiDisplay)]
#[parameters(inv = Time)]
struct Frequency(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Mass(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct PlaneAngle(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct SolidAngle(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiSquare, SiDisplay)]
#[parameters(square = Length)]
struct Area(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Area, rhs_mult = Length)]
struct Volume(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Length, rhs_div = Time)]
struct Velocity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Velocity, rhs_div = Time)]
struct Acceleration(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Mass, rhs_mult = Acceleration)]
struct Force(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Force, rhs_div = Area)]
struct Pressure(NativeType);

// Electrical
#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay)]
#[parameters(lhs_mult = Energy, rhs_mult = Frequency, lhs_div = Energy, rhs_div = Time)]
struct Power(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Force, rhs_mult = Length)]
struct Energy(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay)]
#[parameters(lhs_mult = Energy, rhs_mult = Time, lhs_div = Energy, rhs_div = Frequency)]
struct EnergyPerFrequency(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Power, rhs_div = ElectricCurrent)]
struct ElectricPotential(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct ElectricCurrent(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = Time)]
struct ElectricCharge(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricCharge, rhs_div = ElectricPotential)]
struct Capacitance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricPotential, rhs_div = ElectricCurrent)]
struct ElectricResistance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricCurrent, rhs_div = ElectricPotential)]
struct ElectricConductance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct MagneticFlux(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = MagneticFlux, rhs_div = Area)]
struct MagneticFluxDensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = MagneticFlux, rhs_div = ElectricCurrent)]
struct Inductance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct ThermodynamicTemperature(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Temperature(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct AmountOfSubstance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct LuminousIntensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = LuminousIntensity, rhs_mult = SolidAngle)]
struct LuminousFlux(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = LuminousIntensity, rhs_div = Area)]
struct Illuminance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Pressure, rhs_mult = Time)]
struct DynamicViscosity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Force, rhs_mult = OrthogonalLength)]
struct MomentOfForce(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDivide, SiDisplay)]
#[parameters(lhs_mult = PlaneAngle, rhs_mult = Frequency, lhs_div = PlaneAngle, rhs_div = Time)]
struct AngularVelocity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = AngularVelocity, rhs_div = Time)]
struct AngularAcceleration(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Force, rhs_div = Length)]
struct SurfaceTension(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Power, rhs_div = Area)]
struct HeatFluxDensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Energy, rhs_div = ThermodynamicTemperature)]
struct HeatCapacity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Energy, rhs_div = MassThermodynamicTemperature)]
struct SpecificHeatCapacity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Energy, rhs_div = Mass)]
struct SpecificEnergy(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Energy, rhs_div = Volume)]
struct EnergyDensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricPotential, rhs_div = Length)]
struct ElectricFieldStrength(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = ElectricCharge, rhs_div = Volume)]
struct ElectricChargeDensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct ElectricFluxDensity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Permittivity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Permeability(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct MolarEnergy(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct MolarHeatCapacity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct Radiance(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Mass, rhs_mult = ThermodynamicTemperature)]
struct MassThermodynamicTemperature(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiMultiply, SiDisplay)]
#[parameters(lhs_mult = Length, rhs_mult = ThermodynamicTemperature)]
struct LengthThermodynamicTemperature(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDivide, SiDisplay)]
#[parameters(lhs_div = Length, rhs_div = LengthThermodynamicTemperature)]
struct ThermalConductivity(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct AmountOfSubstanceThermodynamicTemperature(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct AreaSolidAngle(NativeType);

#[derive(Copy, Clone, SiAddSubtract, SiDisplay)]
struct PerAmountOfSubstance(NativeType);


#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};
    use units_proc_macro::si;

    #[test]
    fn temporal_operations() {
        let frequency = 1.0 as NativeType / si!(5s);
        assert_eq!(TypeId::of::<Frequency>(), frequency.type_id());
        let time = 1.0 as NativeType / si!(100Hz);
        assert_eq!(TypeId::of::<Time>(), time.type_id());
        let (a,b) = (frequency * time, time * frequency);
        assert_eq!(TypeId::of::<NativeType>(), a.type_id());
        assert_eq!(TypeId::of::<NativeType>(), b.type_id());
    }

    #[test]
    fn mechanical_operations() {
        let length = si!(5m);
        let area = si!(5m) * length;
        assert_eq!(area, (si!(5m) * si!(5m)));
        assert_eq!(TypeId::of::<Area>(), area.type_id());

        let time = si!(5s);
        let velocity = length / time;
        assert_eq!(TypeId::of::<Velocity>(), velocity.type_id());
        let acceleration = velocity / time;
        assert_eq!(TypeId::of::<Acceleration>(), acceleration.type_id());

        let acceleration0 = si!(10m / 1s / 1s);
        assert_eq!(TypeId::of::<Acceleration>(), acceleration0.type_id());


        let force = si!(1kg) * si!(1mps2);
        assert_eq!(TypeId::of::<Force>(), force.type_id());
        assert_eq!(si!(1kg) * si!(1mps2), si!(1N));

        //native mass value is kg
        let one_kilogram = si!(1kg);
        assert_eq!(one_kilogram.0.round(), (1.0 as NativeType).round());
        let one_kilometer = si!(1km);
        assert_eq!(one_kilometer.0.round(), (1000.0 as NativeType).round());

        let energy = si!(10N * 10m);
        assert_eq!(TypeId::of::<Energy>(), energy.type_id());

    }

    #[test]
    fn electric_operations() {
        let power = si!(10V) * si!(5A);
        assert_eq!(TypeId::of::<Power>(), power.type_id());
        assert_eq!(power, si!(50W));
        assert_eq!(power, si!(50000mW));
        assert_eq!(power, si!(50000011uW));
        assert_ne!(power, si!(50000111uW));

        assert_eq!(si!(500mV), si!(500_000uV));
        assert_ne!(si!(500mV), si!(500_001uV));
        assert_eq!(si!(50mV), si!(50_000uV));
        assert_eq!(si!(5mV), si!(5000001nV));
        assert_ne!(si!(5mV), si!(5000010nV));
        assert_ne!(si!(50mV), si!(50000100nV));
    }
}
