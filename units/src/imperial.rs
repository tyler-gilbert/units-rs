use crate::NativeType;
use units_si::{SiAddSubtract, SiConvert, SiDisplay, SiMultiplyDivideScalar};

use bincode::{Decode, Encode};

use crate::{
    Acceleration, AngularAcceleration, AngularVelocity, Force, Length, Mass, PlaneAngle, Pressure,
    Temperature, ThermodynamicTemperature, Torque, Velocity,
};

const PI: crate::NativeType = 3.1415926535897932384626433 as crate::NativeType;
const ZERO_OFFSET: crate::NativeType = 0.0;
const FEET_PER_METER: crate::NativeType = 1.0 / 0.3048;
const DEGREES_PER_RADIAN: crate::NativeType = 180.0 / PI;
const FAHRENHEIT_PER_CELCIUS: crate::NativeType = 9.0 / 5.0;
const FAHRENHEIT_OFFSET: crate::NativeType = 32.0;
const RANKIN_PER_KELVIN: crate::NativeType = 9.0 / 5.0;
const REVOLUTIONS_PER_RADIAN: crate::NativeType = 1.0 / (2.0 * PI);
const REVOLUTIONS_PER_RADIAN_SECONDS_PER_MINUTE: crate::NativeType = REVOLUTIONS_PER_RADIAN * 60.0;
const NAUTICAL_MILES_PER_METER: crate::NativeType = 1.0 / 1_852.0;
const NAUTICAL_MILES_PER_METER_SECONDS_PER_HOUR: crate::NativeType = 1.0 / (1_852.0 / 3600.0);
const G_PER_ACCELERATION: crate::NativeType = 1.0 / crate::constants::g.native;
const POUNDS_FORCE_PER_NEWTON: crate::NativeType = 0.2248089431;
const POUNDS_PER_KILOGRAM: crate::NativeType = 2.2046226218;
const PSI_PER_PASCAL: crate::NativeType = 0.0001450377;
const PSF_PER_PASCAL: crate::NativeType = PSI_PER_PASCAL * 144.0;
const INCHES_MERCURY_PER_PASCAL: crate::NativeType = 0.0002953006;
const FOOT_POUNDS_PER_NEWTON_METER: crate::NativeType = 0.7375621212;

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
            DegreesFahrenheit::from(crate::Temperature::from(0.0 as crate::NativeType)),
            DegreesFahrenheit::from(32.0 as crate::NativeType)
        );

        assert_eq!(
            DegreesFahrenheit::from(crate::Temperature::from(100.0 as crate::NativeType)),
            DegreesFahrenheit::from(212.0 as crate::NativeType)
        );

        assert_eq!(
            Feet::from(crate::Length::from(2.0 as crate::NativeType)),
            Feet::from(6.561679790026246 as crate::NativeType)
        );

        assert_eq!(
            Revolutions::from(crate::PlaneAngle::from(2.0 * PI as crate::NativeType)),
            Revolutions::from(1.0 as crate::NativeType)
        );

        assert_eq!(
            RevolutionsPerMinute::from(crate::AngularVelocity::from(2.0 * PI as crate::NativeType)),
            RevolutionsPerMinute::from(60.0 as crate::NativeType)
        );

        assert_eq!(
            NauticalMiles::from(crate::Length::from(1_852.0 as crate::NativeType)),
            NauticalMiles::from(1.0 as crate::NativeType)
        );

        assert_eq!(
            Knots::from(crate::Velocity::from(1_852.0 / 3600.0 as crate::NativeType)),
            Knots::from(1.0 as crate::NativeType)
        );

        assert_eq!(
            G::from(crate::constants::g),
            G::from(1.0 as crate::NativeType)
        );
    }
}
