extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Token};

struct Parameters {
    parameters: Vec<(String, syn::Ident)>,
}

impl Parameters {
    fn get_token(&self, name: &str) -> syn::Ident {
        let expected_value = {
            let mut value = String::from("missing parameter:");
            value.push_str(name);
            value
        };
        self.parameters
            .iter()
            .filter(|e| e.0 == name)
            .nth(0)
            .expect(expected_value.as_str())
            .1
            .clone()
    }
}

impl Parse for Parameters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        syn::parenthesized!(content in input);
        let mut result = Parameters { parameters: vec![] };

        loop {
            let key: syn::Ident = content.parse()?;
            content.parse::<Token![=]>()?;
            let value = content.parse()?;
            result.parameters.push((key.to_string(), value));

            match content.parse::<Token![,]>() {
                Err(_) => break,
                _ => continue,
            }
        }
        Ok(result)
    }
}

#[proc_macro_derive(SiDisplay)]
pub fn display_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_display_macro(&ast)
}

#[proc_macro_derive(SiAddSubtract)]
pub fn add_subtract_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_add_subtract_macro(&ast)
}

fn impl_add_subtract_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generate = quote::quote! {
        impl core::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
                #[cfg(feature = "f32")]
                {
                    let lhs_log = libm::floorf(libm::log10f(self.native)) as i32;
                    let rhs_log = libm::floorf(libm::log10f(rhs.native)) as i32;
                    if( lhs_log != rhs_log ){
                        return false;
                    }
                    let power_of = (crate::SIGNIFICANT_FIGURES - lhs_log - 1) as crate::NativeType;
                    (libm::roundf((self.native - rhs.native) * libm::powf(10.0 as crate::NativeType, power_of))) as i32 == 0
                }

                #[cfg(not(feature = "f32"))]
                {
                    let lhs_log = libm::floor(libm::log10(self.native)) as i32;
                    let rhs_log = libm::floor(libm::log10(rhs.native)) as i32;
                    if( lhs_log != rhs_log ){
                        return false;
                    }
                    let power_of = (crate::SIGNIFICANT_FIGURES - lhs_log - 1) as crate::NativeType;
                    (libm::round((self.native - rhs.native) * libm::pow(10.0 as crate::NativeType, power_of))) as i32 == 0
                }
            }
        }

        impl #name {
            pub const fn new(native: crate::NativeType) -> Self {
                Self{ native }
            }

            pub fn abs(&self) -> Self {
                Self{ native: self.native.abs() }
            }

        }

        impl Into<crate::NativeType> for #name {
            fn into(self) -> crate::NativeType {
                self.native
            }
        }

        impl From<crate::NativeType> for #name {
            fn from(native: crate::NativeType) -> #name {
                #name{ native }
            }
        }

        impl core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.native.partial_cmp(&other.native)
            }
        }

        impl core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
               Self::from(self.native + rhs.native)
            }
        }

        impl core::ops::Mul<crate::Scalar> for #name {
            type Output = Self;
            fn mul(self, rhs: crate::Scalar) -> Self {
               Self::from(self.native * rhs.native)
            }
        }

        impl core::ops::Div<#name> for #name {
            type Output = crate::Scalar;
            fn div(self, rhs: #name) -> crate::Scalar {
               crate::Scalar::from(self.native / rhs.native)
            }
        }

        impl core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
               *self = Self::from(self.native + rhs.native);
            }
        }

        impl core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
               Self::from(self.native - rhs.native)
            }
        }

        impl core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
               *self = Self::from(self.native - rhs.native);
            }
        }
    };
    generate.into()
}

#[proc_macro_derive(SiMultiplyDivideScalar)]
pub fn add_subtract_no_divide_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_multiply_divide_scalar_macro(&ast)
}

fn impl_multiply_divide_scalar_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generate = quote::quote! {

        impl core::ops::Mul<crate::Decibel<#name>> for #name
        {
            type Output = Self;
            fn mul(self, rhs: crate::Decibel<#name>) -> Self {
                Self::from(self * rhs.ratio())
            }
        }

        impl core::ops::Mul<#name> for crate::Scalar {
            type Output = #name;
            fn mul(self, rhs: #name) -> #name {
               #name::from(self.native * rhs.native)
            }
        }

        impl core::ops::Div<crate::Scalar> for #name {
            type Output = #name;
            fn div(self, rhs: crate::Scalar) -> #name {
               #name::from(self.native / rhs.native)
            }
        }

    };
    generate.into()
}

fn get_parameters(ast: &DeriveInput, expect: &'static str) -> Parameters {
    let attribute = ast
        .attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "parameters")
        .expect(expect);

    syn::parse2(attribute.tokens.clone()).expect("Invalid parameters attribute!")
}

#[proc_macro_derive(SiMultiply, attributes(parameters))]
pub fn mult_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiMultiply");

    // Build the trait implementation
    let name = &ast.ident;
    let lhs = parameters.get_token("lhs_mult");
    let rhs = parameters.get_token("rhs_mult");
    gen_multiply(name, lhs, rhs)
}

#[proc_macro_derive(SiMultiplyAlt, attributes(parameters))]
pub fn mult_alt_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiMultiply");

    // Build the trait implementation
    let name = &ast.ident;
    let lhs = parameters.get_token("lhs_mult_alt");
    let rhs = parameters.get_token("rhs_mult_alt");
    gen_multiply(name, lhs, rhs)
}

fn gen_multiply(name: &syn::Ident, lhs: syn::Ident, rhs: syn::Ident) -> TokenStream {
    let generate = quote::quote! {
        impl core::ops::Mul<#rhs> for #lhs {
            type Output = #name;
            fn mul(self, rhs: #rhs) -> #name {
               #name::from(self.native * rhs.native)
            }
        }

        impl core::ops::Mul<#lhs> for #rhs {
            type Output = #name;
            fn mul(self, rhs: #lhs) -> #name {
               #name::from(self.native * rhs.native)
            }
        }


        impl core::ops::Div<#rhs> for #name {
            type Output = #lhs;
            fn div(self, rhs: #rhs) -> #lhs {
               #lhs::from(self.native / rhs.native)
            }
        }

        impl core::ops::Div<#lhs> for #name {
            type Output = #rhs;
            fn div(self, rhs: #lhs) -> #rhs {
               #rhs::from(self.native / rhs.native)
            }
        }
    };
    generate.into()
}

#[proc_macro_derive(SiSquare, attributes(parameters))]
pub fn square_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiSquare");
    let name = &ast.ident;
    let square = parameters.get_token("square");
    let generate = quote::quote! {
        impl core::ops::Mul<#square> for #square {
            type Output = #name;
            fn mul(self, square: #square) -> #name {
               #name::from(self.native * square.native)
            }
        }

        impl core::ops::Div<#square> for #name {
            type Output = #square;
            fn div(self, square: #square) -> #square {
               #square::from(self.native / square.native)
            }
        }

        impl #name {
            pub fn sqrt(&self) -> #square {
                #[cfg(feature = "f32")]
                {
                    #square::from(libm::sqrtf(self.native))
                }

                #[cfg(not(feature = "f32"))]
                {
                    #square::from(libm::sqrt(self.native))
                }
            }
        }

    };
    generate.into()
}

#[proc_macro_derive(SiInvert, attributes(parameters))]
pub fn invert_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiSquare");
    let name = &ast.ident;
    let inv = parameters.get_token("inv");
    let generate = quote::quote! {
        impl core::ops::Mul<#inv> for #name {
            type Output = crate::Scalar;
            fn mul(self, inv: #inv) -> crate::Scalar {
               crate::Scalar::from(self.native * inv.native)
            }
        }

        impl core::ops::Mul<#name> for #inv {
            type Output = crate::Scalar;
            fn mul(self, name: #name) -> crate::Scalar {
               crate::Scalar::from(self.native * name.native)
            }
        }


        impl core::ops::Div<#inv> for crate::Scalar {
            type Output = #name;
            fn div(self, inv: #inv) -> #name {
               #name::from(self.native / inv.native)
            }
        }


        impl core::ops::Div<#name> for crate::Scalar {
            type Output = #inv;
            fn div(self, name: #name) -> #inv {
               #inv::from(self.native / name.native)
            }
        }

    };
    generate.into()
}

#[proc_macro_derive(SiDivide, attributes(parameters))]
pub fn divide_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiDivide");

    let name = &ast.ident;
    let lhs = parameters.get_token("lhs_div");
    let rhs = parameters.get_token("rhs_div");
    let generate = quote::quote! {
        impl core::ops::Div<#rhs> for #lhs {
            type Output = #name;
            fn div(self, rhs: #rhs) -> #name {
               #name::from(self.native / rhs.native)
            }
        }

        impl core::ops::Div<#name> for #lhs {
            type Output = #rhs;
            fn div(self, rhs: #name) -> #rhs {
               #rhs::from(self.native / rhs.native)
            }
        }

        impl core::ops::Mul<#rhs> for #name {
            type Output = #lhs;
            fn mul(self, rhs: #rhs) -> #lhs {
               #lhs::from(self.native * rhs.native)
            }
        }

        impl core::ops::Mul<#name> for #rhs {
            type Output = #lhs;
            fn mul(self, rhs: #name) -> #lhs {
               #lhs::from(self.native * rhs.native)
            }
        }
    };
    generate.into()
}

#[proc_macro_derive(SiConvert, attributes(parameters))]
pub fn convert_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiDivide");

    let name = &ast.ident;
    let multiplier = parameters.get_token("multiplier");
    let offset = parameters.get_token("offset");
    let into = parameters.get_token("into");
    let generate = quote::quote! {

        impl From<#into> for #name {
            fn from(value: #into) -> Self {
                Self::from(value.native * #multiplier + #offset)
            }
        }

        impl Into<#into> for #name {
            fn into(self) -> #into {
                #into::from((self.native - #offset) / #multiplier)
            }
        }
    };
    generate.into()
}

struct UnitType {
    name: &'static str,
    label: &'static str,
}

const UNITS: &[UnitType] = &[
    UnitType {
        name: "Scalar",
        label: "scalar",
    },
    UnitType {
        name: "DecibelV",
        label: "decibelsV",
    },
    UnitType {
        name: "Decibel",
        label: "decibels",
    },
    UnitType {
        name: "Mass",
        label: "kilograms",
    },
    UnitType {
        name: "Time",
        label: "seconds",
    },
    UnitType {
        name: "ElectricCurrent",
        label: "amps",
    },
    UnitType {
        name: "ThermodynamicTemperature",
        label: "kelvin",
    },
    UnitType {
        name: "AmountOfSubstance",
        label: "moles",
    },
    UnitType {
        name: "Length",
        label: "meters",
    },
    UnitType {
        name: "LengthInverse",
        label: "1/meter",
    },
    UnitType {
        name: "OrthogonalLength",
        label: "meters",
    },
    UnitType {
        name: "LuminousIntensity",
        label: "candelas",
    },
    UnitType {
        name: "PlaneAngle",
        label: "radians",
    },
    UnitType {
        name: "PlaneAngleInverse",
        label: "1/radians",
    },
    UnitType {
        name: "SolidAngle",
        label: "steradians",
    },
    UnitType {
        name: "Frequency",
        label: "hertz",
    },
    UnitType {
        name: "FrequencySquared",
        label: "hertz^2",
    },
    UnitType {
        name: "Area",
        label: "meters^2",
    },
    UnitType {
        name: "Volume",
        label: "meters^3",
    },
    UnitType {
        name: "Liters",
        label: "liters",
    },
    UnitType {
        name: "Velocity",
        label: "meters/second",
    },
    UnitType {
        name: "VelocitySquared",
        label: "(meters/second)^2",
    },
    UnitType {
        name: "Acceleration",
        label: "meters/second^2",
    },
    UnitType {
        name: "Jerk",
        label: "meters/second^3",
    },
    UnitType {
        name: "Force",
        label: "newtons",
    },
    UnitType {
        name: "Pressure",
        label: "pascals",
    },
    UnitType {
        name: "Energy",
        label: "joules",
    },
    UnitType {
        name: "EnergyPerFrequency",
        label: "joules/hertz",
    },
    UnitType {
        name: "Power",
        label: "watts",
    },
    UnitType {
        name: "ElectricCharge",
        label: "coulombs",
    },
    UnitType {
        name: "ElectricPotential",
        label: "volts",
    },
    UnitType {
        name: "Capacitance",
        label: "farads",
    },
    UnitType {
        name: "ElectricResistance",
        label: "ohms",
    },
    UnitType {
        name: "ElectricConductance",
        label: "siemens",
    },
    UnitType {
        name: "MagneticFlux",
        label: "webers",
    },
    UnitType {
        name: "MagneticFluxDensity",
        label: "teslas",
    },
    UnitType {
        name: "Inductance",
        label: "henries",
    },
    UnitType {
        name: "Temperature",
        label: "celcius",
    },
    UnitType {
        name: "LuminousFlux",
        label: "lumens",
    },
    UnitType {
        name: "Illuminance",
        label: "lux",
    },
    UnitType {
        name: "DynamicViscosity",
        label: "pascals*seconds",
    },
    UnitType {
        name: "MomentOfForce",
        label: "newtons*meters",
    },
    UnitType {
        name: "Torque",
        label: "newtons*meters",
    },
    UnitType {
        name: "AngularVelocity",
        label: "radians/second",
    },
    UnitType {
        name: "AngularVelocitySquared",
        label: "(radians/second)^2",
    },
    UnitType {
        name: "AngularAcceleration",
        label: "radians/second^2",
    },
    UnitType {
        name: "SurfaceTension",
        label: "newtons/meter",
    },
    UnitType {
        name: "HeatFluxDensity",
        label: "watts/meter^2",
    },
    UnitType {
        name: "HeatCapacity",
        label: "joules/kelvin",
    },
    UnitType {
        name: "SpecificHeatCapacity",
        label: "joules/(kilogram*kelvin)",
    },
    UnitType {
        name: "SpecificEnergy",
        label: "joules/kilogram",
    },
    UnitType {
        name: "ThermalConductivity",
        label: "watts/(meter*kelvin)",
    },
    UnitType {
        name: "EnergyDensity",
        label: "joules/meter^3",
    },
    UnitType {
        name: "ElectricFieldStrength",
        label: "volts/meter",
    },
    UnitType {
        name: "ElectricChargeDensity",
        label: "coulombs/meter^3",
    },
    UnitType {
        name: "ElectricFluxDensity",
        label: "coulombs/meter^2",
    },
    UnitType {
        name: "Permittivity",
        label: "farads/meter",
    },
    UnitType {
        name: "Permeability",
        label: "henries/meter",
    },
    UnitType {
        name: "MolarEnergy",
        label: "joules/mole",
    },
    UnitType {
        name: "MolarHeatCapacity",
        label: "joules/(mole*kelvin)",
    },
    UnitType {
        name: "Radiance",
        label: "watts/(meter^2*steradian)",
    },
    UnitType {
        name: "MassThermodynamicTemperature",
        label: "kilograms*kelvin",
    },
    UnitType {
        name: "LengthThermodynamicTemperature",
        label: "meters*kelvin",
    },
    UnitType {
        name: "AmountOfSubstanceThermodynamicTemperature",
        label: "moles*kelvin",
    },
    UnitType {
        name: "AreaSolidAngle",
        label: "meters^2/steradian",
    },
    UnitType {
        name: "PerAmountOfSubstance",
        label: "1/mole",
    },
    UnitType {
        name: "EnergyPerFrequency",
        label: "joules/hertz",
    },
    UnitType {
        name: "MassDensity",
        label: "kilograms/meter^3",
    },
    // Imperial units
    UnitType {
        name: "Feet",
        label: "feet",
    },
    UnitType {
        name: "Yard",
        label: "yards",
    },
    UnitType {
        name: "Inch",
        label: "inches",
    },
    UnitType {
        name: "Miles",
        label: "miles",
    },
    UnitType {
        name: "Acres",
        label: "acres",
    },
    UnitType {
        name: "SquareMiles",
        label: "squaremiles",
    },
    UnitType {
        name: "Pints",
        label: "pt",
    },
    UnitType {
        name: "Quarts",
        label: "qt",
    },
    UnitType {
        name: "Gallons",
        label: "gal",
    },
    UnitType {
        name: "Degrees",
        label: "degrees",
    },
    UnitType {
        name: "DegreesPerSecond",
        label: "degrees/second",
    },
    UnitType {
        name: "DegreesPerSecondSquared",
        label: "degrees/second^2",
    },
    UnitType {
        name: "DegreesFahrenheit",
        label: "degreesF",
    },
    UnitType {
        name: "DegreesRankine",
        label: "degreesR",
    },
    UnitType {
        name: "Revolutions",
        label: "revolutions",
    },
    UnitType {
        name: "RevolutionsPerMinute",
        label: "rpm",
    },
    UnitType {
        name: "NauticalMiles",
        label: "nauticalmiles",
    },
    UnitType {
        name: "Knots",
        label: "knots",
    },
    UnitType {
        name: "FeetPerSecond",
        label: "feet/second",
    },
    UnitType {
        name: "FeetPerSecondSquared",
        label: "feet/second^2",
    },
    UnitType {
        name: "FeetPerMinute",
        label: "feet/minute",
    },
    UnitType {
        name: "G",
        label: "g",
    },
    UnitType {
        name: "PoundsForce",
        label: "lbsforce",
    },
    UnitType {
        name: "Pounds",
        label: "lbs",
    },
    UnitType {
        name: "Ounces",
        label: "oz",
    },
    UnitType {
        name: "PoundsPerSquareInch",
        label: "psi",
    },
    UnitType {
        name: "PoundsPerSquareFoot",
        label: "psf",
    },
    UnitType {
        name: "InchesMercury",
        label: "inHg",
    },
    UnitType {
        name: "FootPounds",
        label: "ftlbs",
    },
];

fn find_unit(name: String) -> &'static UnitType {
    for unit in UNITS {
        if name == unit.name {
            return unit;
        }
    }
    panic!("no unit named {} was found", name);
}

fn impl_display_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();
    let current_unit = find_unit(name_string);
    let label: &'static str = current_unit.label;

    let generate = quote::quote! {

      #[cfg(feature = "use_defmt")]
      impl defmt::Format for #name {
        fn format(&self, f: defmt::Formatter<'_>) {
          defmt::write!(f, "{} {}", self.native, #label);
        }
      }

      #[cfg(feature = "std")]
      impl serde::Serialize for #name {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
              S: serde::Serializer,
          {
              // Format however you want
              let s = std::format!("{}_{}", self.native, #label);
              serializer.serialize_str(&s)
          }
      }

      #[cfg(feature = "std")]
      impl<'de> serde::Deserialize<'de> for #name {
          fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where
              D: serde::Deserializer<'de>,
          {
              struct UnitsVisitor;

              impl<'de> serde::de::Visitor<'de> for UnitsVisitor {
                  type Value = #name;

                  fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                      f.write_str(format!("a string like `10.0_{}`", #label).as_str())
                  }

                  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                  where
                      E: serde::de::Error,
                  {
                      // Expect format "<value>_meters"
                      if let Some((value_str, #label)) = v.rsplit_once('_') {
                          let native: crate::NativeType = value_str
                              .parse()
                              .map_err(|_| E::custom(format!("invalid float in {}", #label).as_str()))?;
                          return Ok(#name { native });
                      }

                      Err(E::custom(format!("expected format `<float>_{}`", #label).as_str()))
                  }
              }

              deserializer.deserialize_str(UnitsVisitor)
          }
      }

      #[cfg(feature = "std")]
      impl std::fmt::Display for #name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{} {}", self.native, #label)
        }
      }

      #[cfg(feature = "std")]
      impl std::fmt::Debug for #name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
          f.debug_struct(stringify!(#name))
              .field("value", &self.to_string())
              .field("label", &#label)
              .finish()
        }
      }
    };
    generate.into()
}
