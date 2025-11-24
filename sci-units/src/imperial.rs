use crate::NativeType;
use sci_units_proc_macro::{SiAddSubtract, SiConvert, SiDisplay, SiMultiplyDivideScalar};

use bincode::{Decode, Encode};

use crate::{
    Acceleration, AngularAcceleration, AngularVelocity, Area, Force, Length, Mass, PlaneAngle,
    Pressure, Temperature, ThermodynamicTemperature, Torque, Velocity, Volume,
};

const ZERO_OFFSET: NativeType = 0.0;
const FEET_PER_METER: NativeType = 1.0 / 0.3048;
const INCHES_PER_METER: NativeType = FEET_PER_METER * 12.0;
const YARDS_PER_METER: NativeType = FEET_PER_METER / 3.0;
const MILES_PER_METER: NativeType = FEET_PER_METER / 5280.0;

const PINTS_PER_METER_CUBED: NativeType = 1.0 / 0.000_568_261_25;
const QUARTS_PER_METER_CUBED: NativeType = 1.0 / 0.001_136_522_5;
const GALLONS_PER_METER_CUBED: NativeType = 1.0 / 0.004_546_09;
const DEGREES_PER_RADIAN: NativeType = 180.0 / crate::PI;
const FAHRENHEIT_PER_CELCIUS: NativeType = 9.0 / 5.0;
const FAHRENHEIT_OFFSET: NativeType = 32.0;
const RANKIN_PER_KELVIN: NativeType = 9.0 / 5.0;
const REVOLUTIONS_PER_RADIAN: NativeType = 1.0 / (2.0 * crate::PI);
const REVOLUTIONS_PER_RADIAN_SECONDS_PER_MINUTE: NativeType = REVOLUTIONS_PER_RADIAN * 60.0;
const NAUTICAL_MILES_PER_METER: NativeType = 1.0 / 1_852.0;
const NAUTICAL_MILES_PER_METER_SECONDS_PER_HOUR: NativeType = 1.0 / (1_852.0 / 3600.0);
const G_PER_ACCELERATION: NativeType = 1.0 / crate::constants::g.native;
const PSI_PER_PASCAL: NativeType = 0.0001450377;
const PSF_PER_PASCAL: NativeType = PSI_PER_PASCAL * 144.0;
const INCHES_MERCURY_PER_PASCAL: NativeType = 0.0002953006;

#[cfg(feature = "f32")]
mod sealed {
    use crate::NativeType;
    pub const ACRES_PER_METER_SQUARED: NativeType = 1.0 / 4_046.856_4;
    pub const SQUARE_MILES_PER_METER_SQUARED: NativeType = 1.0 / 2_589_988.110_336;
    pub const POUNDS_FORCE_PER_NEWTON: NativeType = 0.224_808_95;
    pub const OUNCES_PER_KILOGRAM: NativeType = 1.0 / 0.028_349_523;
    pub const POUNDS_PER_KILOGRAM: NativeType = 2.204_622_5;
    pub const FOOT_POUNDS_PER_NEWTON_METER: NativeType = 0.737_562_1;
}

#[cfg(not(feature = "f32"))]
mod sealed {
    use crate::NativeType;
    pub const ACRES_PER_METER_SQUARED: NativeType = 1.0 / 4_046.856_422_4;
    pub const SQUARE_MILES_PER_METER_SQUARED: NativeType = 1.0 / 2_589_988.110_336;
    pub const POUNDS_FORCE_PER_NEWTON: NativeType = 0.2248089431;
    pub const OUNCES_PER_KILOGRAM: NativeType = 1.0 / 0.028_349_523_125;
    pub const POUNDS_PER_KILOGRAM: NativeType = 2.2046226218;
    pub const FOOT_POUNDS_PER_NEWTON_METER: NativeType = 0.7375621212;
}

const ACRES_PER_METER_SQUARED: NativeType = sealed::ACRES_PER_METER_SQUARED;
const SQUARE_MILES_PER_METER_SQUARED: NativeType = sealed::SQUARE_MILES_PER_METER_SQUARED;
const POUNDS_FORCE_PER_NEWTON: NativeType = sealed::POUNDS_FORCE_PER_NEWTON;
const OUNCES_PER_KILOGRAM: NativeType = sealed::OUNCES_PER_KILOGRAM;
const POUNDS_PER_KILOGRAM: NativeType = sealed::POUNDS_PER_KILOGRAM;
const FOOT_POUNDS_PER_NEWTON_METER: NativeType = sealed::FOOT_POUNDS_PER_NEWTON_METER;

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = FEET_PER_METER, offset = ZERO_OFFSET, into = Length)]
pub struct Feet {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = YARDS_PER_METER, offset = ZERO_OFFSET, into = Length)]
pub struct Yard {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = INCHES_PER_METER, offset = ZERO_OFFSET, into = Length)]
pub struct Inch {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = MILES_PER_METER, offset = ZERO_OFFSET, into = Length)]
pub struct Miles {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = ACRES_PER_METER_SQUARED, offset = ZERO_OFFSET, into = Area)]
pub struct Acres {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = SQUARE_MILES_PER_METER_SQUARED, offset = ZERO_OFFSET, into = Area)]
pub struct SquareMiles {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = PINTS_PER_METER_CUBED, offset = ZERO_OFFSET, into = Volume)]
pub struct Pints {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = QUARTS_PER_METER_CUBED, offset = ZERO_OFFSET, into = Volume)]
pub struct Quarts {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = GALLONS_PER_METER_CUBED, offset = ZERO_OFFSET, into = Volume)]
pub struct Gallons {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = DEGREES_PER_RADIAN, offset = ZERO_OFFSET, into = PlaneAngle)]
pub struct Degrees {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = DEGREES_PER_RADIAN, offset = ZERO_OFFSET, into = AngularVelocity)]
pub struct DegreesPerSecond {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = DEGREES_PER_RADIAN, offset = ZERO_OFFSET, into = AngularAcceleration)]
pub struct DegreesPerSecondSquared {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = FAHRENHEIT_PER_CELCIUS, offset = FAHRENHEIT_OFFSET, into = Temperature)]
pub struct DegreesFahrenheit {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = RANKIN_PER_KELVIN, offset = ZERO_OFFSET, into = ThermodynamicTemperature)]
pub struct DegreesRankine {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = REVOLUTIONS_PER_RADIAN, offset = ZERO_OFFSET, into = PlaneAngle)]
pub struct Revolutions {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = REVOLUTIONS_PER_RADIAN_SECONDS_PER_MINUTE, offset = ZERO_OFFSET, into = AngularVelocity)]
pub struct RevolutionsPerMinute {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = NAUTICAL_MILES_PER_METER, offset = ZERO_OFFSET, into = Length)]
pub struct NauticalMiles {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = NAUTICAL_MILES_PER_METER_SECONDS_PER_HOUR, offset = ZERO_OFFSET, into = Velocity)]
pub struct Knots {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = FEET_PER_METER, offset = ZERO_OFFSET, into = Velocity)]
pub struct FeetPerSecond {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = FEET_PER_METER, offset = ZERO_OFFSET, into = Acceleration)]
pub struct FeetPerSecondSquared {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = G_PER_ACCELERATION, offset = ZERO_OFFSET, into = Acceleration)]
pub struct G {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = POUNDS_FORCE_PER_NEWTON, offset = ZERO_OFFSET, into = Force)]
pub struct PoundsForce {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = POUNDS_PER_KILOGRAM, offset = ZERO_OFFSET, into = Mass)]
pub struct Pounds {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = OUNCES_PER_KILOGRAM, offset = ZERO_OFFSET, into = Mass)]
pub struct Ounces {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = PSI_PER_PASCAL, offset = ZERO_OFFSET, into = Pressure)]
pub struct PoundsPerSquareInch {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = PSF_PER_PASCAL, offset = ZERO_OFFSET, into = Pressure)]
pub struct PoundsPerSquareFoot {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = INCHES_MERCURY_PER_PASCAL, offset = ZERO_OFFSET, into = Pressure)]
pub struct InchesMercury {
    native: NativeType,
}

#[derive(
    Copy, Clone, SiAddSubtract, SiMultiplyDivideScalar, SiDisplay, SiConvert, Decode, Encode,
)]
#[parameters(multiplier = FOOT_POUNDS_PER_NEWTON_METER, offset = ZERO_OFFSET, into = Torque)]
pub struct FootPounds {
    native: NativeType,
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    use std::any::{Any, TypeId};

    use crate::basic;

    basic!(test_feet, Feet);
    basic!(test_degrees, Degrees);
    basic!(test_degrees_squared, DegreesPerSecond);
    basic!(test_degrees_per_second_squared, DegreesPerSecondSquared);
    basic!(test_degrees_fahrenheit, DegreesFahrenheit);
    basic!(test_degrees_rankine, DegreesRankine);
    basic!(test_revolutions, Revolutions);
    basic!(test_revolutions_per_minute, RevolutionsPerMinute);
    basic!(test_nautical_miles, NauticalMiles);
    basic!(test_knots, Knots);
    basic!(test_feet_per_second, FeetPerSecond);
    basic!(test_feet_per_second_squared, FeetPerSecondSquared);
    basic!(test_g, G);
    basic!(test_pounds_force, PoundsForce);
    basic!(test_pounds, Pounds);
    basic!(test_psi, PoundsPerSquareInch);
    basic!(test_psf, PoundsPerSquareFoot);
    basic!(test_inches_mercury, InchesMercury);
    basic!(test_foot_pounds, FootPounds);

    #[test]
    fn conversion_test() {
        assert_eq!(
            DegreesFahrenheit::from(crate::Temperature::from(0.0 as NativeType)),
            DegreesFahrenheit::from(32.0 as NativeType)
        );

        assert_eq!(
            DegreesFahrenheit::from(crate::Temperature::from(100.0 as NativeType)),
            DegreesFahrenheit::from(212.0 as NativeType)
        );

        assert_eq!(
            Feet::from(crate::Length::from(2.0 as NativeType)),
            Feet::from(6.561679790026246 as NativeType)
        );

        assert_eq!(
            Revolutions::from(crate::PlaneAngle::from(2.0 * crate::PI as NativeType)),
            Revolutions::from(1.0 as NativeType)
        );

        assert_eq!(
            RevolutionsPerMinute::from(crate::AngularVelocity::from(2.0 * crate::PI as NativeType)),
            RevolutionsPerMinute::from(60.0 as NativeType)
        );

        assert_eq!(
            NauticalMiles::from(crate::Length::from(1_852.0 as NativeType)),
            NauticalMiles::from(1.0 as NativeType)
        );

        assert_eq!(
            Knots::from(crate::Velocity::from(1_852.0 / 3600.0 as NativeType)),
            Knots::from(1.0 as NativeType)
        );

        let psi: crate::Pressure = PoundsPerSquareInch::new(1.0).into();
        let psf: crate::Pressure = PoundsPerSquareFoot::new(144.0).into();
        assert_eq!(psi, psf);

        let mile: crate::Length = Miles::new(1.0).into();
        let feet: crate::Length = Feet::new(5280.0).into();
        assert_eq!(mile, feet);

        assert_eq!(G::from(crate::constants::g), G::from(1.0 as NativeType));
    }
}
