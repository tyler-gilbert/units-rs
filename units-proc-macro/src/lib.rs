extern crate proc_macro;

use proc_macro::TokenStream;
use quote;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    DeriveInput, Expr, ExprLit, Lit, LitInt, Token,
};

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
            result
                .parameters
                .push((String::from(key.to_string()), value));

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
    let gen = quote::quote! {
        impl std::cmp::PartialEq for #name {
            fn eq(&self, rhs: &Self) -> bool {
              let lhs_log = self.0.log10().floor() as i32;
              let rhs_log = rhs.0.log10().floor() as i32;
              if( lhs_log != rhs_log ){
                  return false;
              }
              let power_of = SIGNIFICANT_FIGURES - lhs_log - 1;
              (((self.0 - rhs.0) * (10.0 as NativeType).powi(power_of)).round() as i32) == 0
            }
        }

        impl std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }

            /*
            fn le(&self, other: #name) -> bool {
                self.0 < other.0 || self == other
            }

            fn ge(&self, other: #name) -> bool {
                self.0 > other.0 || self == other
            }
            */
        }

        impl std::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
               Self(self.0 + rhs.0)
            }
        }

        impl std::ops::Mul<#name> for NativeType {
            type Output = #name;
            fn mul(self, rhs: #name) -> #name {
               #name(self * rhs.0)
            }
        }

        impl std::ops::Mul<NativeType> for #name {
            type Output = Self;
            fn mul(self, rhs: NativeType) -> Self {
               Self(self.0 * rhs)
            }
        }

        impl std::ops::Div<NativeType> for #name {
            type Output = Self;
            fn div(self, rhs: NativeType) -> Self {
               Self(self.0 / rhs)
            }
        }

        impl std::ops::Div<#name> for #name {
            type Output = NativeType;
            fn div(self, rhs: #name) -> NativeType {
               self.0 / rhs.0
            }
        }

        impl std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
               *self = Self(self.0 + rhs.0);
            }
        }

        impl std::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
               Self(self.0 + rhs.0)
            }
        }

        impl std::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
               *self = Self(self.0 + rhs.0);
            }
        }
    };
    gen.into()
}

fn get_parameters(ast: &DeriveInput, expect: &'static str) -> Parameters {
    let attribute = ast
        .attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "parameters")
        .nth(0)
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
    let gen = quote::quote! {
        impl std::ops::Mul<#rhs> for #lhs {
            type Output = #name;
            fn mul(self, rhs: #rhs) -> #name {
               #name(self.0 * rhs.0)
            }
        }

        impl std::ops::Mul<#lhs> for #rhs {
            type Output = #name;
            fn mul(self, rhs: #lhs) -> #name {
               #name(self.0 * rhs.0)
            }
        }


        impl std::ops::Div<#rhs> for #name {
            type Output = #lhs;
            fn div(self, rhs: #rhs) -> #lhs {
               #lhs(self.0 / rhs.0)
            }
        }

        impl std::ops::Div<#lhs> for #name {
            type Output = #rhs;
            fn div(self, rhs: #lhs) -> #rhs {
               #rhs(self.0 / rhs.0)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SiSquare, attributes(parameters))]
pub fn square_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiSquare");
    let name = &ast.ident;
    let square = parameters.get_token("square");
    let gen = quote::quote! {
        impl std::ops::Mul<#square> for #square {
            type Output = #name;
            fn mul(self, square: #square) -> #name {
               #name(self.0 * square.0)
            }
        }

        impl std::ops::Div<#square> for #name {
            type Output = #square;
            fn div(self, square: #square) -> #square {
               #square(self.0 / square.0)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SiInvert, attributes(parameters))]
pub fn invert_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let parameters = get_parameters(&ast, "parameters required for deriving SiSquare");
    let name = &ast.ident;
    let inv = parameters.get_token("inv");
    let gen = quote::quote! {
        impl std::ops::Mul<#inv> for #name {
            type Output = NativeType;
            fn mul(self, inv: #inv) -> NativeType {
               self.0 * inv.0
            }
        }

        impl std::ops::Mul<#name> for #inv {
            type Output = NativeType;
            fn mul(self, name: #name) -> NativeType {
               self.0 * name.0
            }
        }


        impl std::ops::Div<#inv> for NativeType {
            type Output = #name;
            fn div(self, inv: #inv) -> #name {
               #name(self / inv.0)
            }
        }


        impl std::ops::Div<#name> for NativeType {
            type Output = #inv;
            fn div(self, name: #name) -> #inv {
               #inv(self / name.0)
            }
        }

    };
    gen.into()
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
    let gen = quote::quote! {
        impl std::ops::Div<#rhs> for #lhs {
            type Output = #name;
            fn div(self, rhs: #rhs) -> #name {
               #name(self.0 / rhs.0)
            }
        }

        impl std::ops::Div<#name> for #lhs {
            type Output = #rhs;
            fn div(self, rhs: #name) -> #rhs {
               #rhs(self.0 / rhs.0)
            }
        }

        impl std::ops::Mul<#rhs> for #name {
            type Output = #lhs;
            fn mul(self, rhs: #rhs) -> #lhs {
               #lhs(self.0 * rhs.0)
            }
        }

        impl std::ops::Mul<#name> for #rhs {
            type Output = #lhs;
            fn mul(self, rhs: #name) -> #lhs {
               #lhs(self.0 * rhs.0)
            }
        }
    };
    gen.into()
}

// actual procedural macro
#[proc_macro]
pub fn si(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Expr);
    LiteralReplacer.visit_expr_mut(&mut input);
    input.into_token_stream().into()
}

struct UnitType {
    literal_suffix: &'static str,
    suffix: &'static str,
    name: &'static str,
    label: &'static str,
}

struct ModifierType {
    prefix: &'static str,
    multiplier: f64,
}

fn process_unit(suffix: &str, unit_value: &syn::LitInt, unit: &UnitType) -> Option<Expr> {
    let modifiers = [
        ModifierType {
            prefix: "f",
            multiplier: 0.000_000_000_000_001,
        },
        ModifierType {
            prefix: "p",
            multiplier: 0.000_000_000_001,
        },
        ModifierType {
            prefix: "n",
            multiplier: 0.000_000_001,
        },
        ModifierType {
            prefix: "u",
            multiplier: 0.000_001,
        },
        ModifierType {
            prefix: "m",
            multiplier: 0.001,
        },
        ModifierType {
            prefix: "k",
            multiplier: 1000.0,
        },
        ModifierType {
            prefix: "M",
            multiplier: 1000_000.0,
        },
        ModifierType {
            prefix: "G",
            multiplier: 1000_000_000.0,
        },
        ModifierType {
            prefix: "T",
            multiplier: 1000_000_000_000.0,
        },
    ];

    if unit.literal_suffix == "" {
        return None;
    }

    let name: syn::Expr = syn::parse_str(unit.name).expect("failed ot parse");
    let adjust_kilo_default = if unit.name == "Mass" { 0.001 } else { 1.0 };

    for modifier in modifiers {
        let multiplier = modifier.multiplier;
        if suffix == format!("{}{}", modifier.prefix, unit.literal_suffix) {
            return Some(
                parse_quote! { #name( #unit_value as NativeType * #multiplier as NativeType * #adjust_kilo_default as NativeType) },
            );
        }
    }

    if suffix == unit.literal_suffix {
        return Some(
            parse_quote! { #name( #unit_value as NativeType * #adjust_kilo_default as NativeType) },
        );
    }

    return None;
}

const UNITS: &[UnitType] = &[
    UnitType {
        literal_suffix: "g",
        suffix: "kg",
        name: "Mass",
        label: "kilograms",
    },
    UnitType {
        literal_suffix: "s",
        suffix: "s",
        name: "Time",
        label: "seconds",
    },
    UnitType {
        literal_suffix: "A",
        suffix: "A",
        name: "ElectricCurrent",
        label: "amps",
    },
    UnitType {
        literal_suffix: "K",
        suffix: "K",
        name: "ThermodynamicTemperature",
        label: "kelvin",
    },
    UnitType {
        literal_suffix: "mol",
        suffix: "mol",
        name: "AmountOfSubstance",
        label: "moles",
    },
    UnitType {
        literal_suffix: "m",
        suffix: "m",
        name: "Length",
        label: "meters",
    },
    UnitType {
        literal_suffix: "",
        suffix: "m",
        name: "OrthogonalLength",
        label: "meters",
    },
    UnitType {
        literal_suffix: "cd",
        suffix: "cd",
        name: "LuminousIntensity",
        label: "candelas",
    },
    UnitType {
        literal_suffix: "rad",
        suffix: "rad",
        name: "PlaneAngle",
        label: "radians",
    },
    UnitType {
        literal_suffix: "sr",
        suffix: "sr",
        name: "SolidAngle",
        label: "steradians",
    },
    UnitType {
        literal_suffix: "Hz",
        suffix: "Hz",
        name: "Frequency",
        label: "hertz",
    },
    UnitType {
        literal_suffix: "m2",
        suffix: "m^2",
        name: "Area",
        label: "meters squared",
    },
    UnitType {
        literal_suffix: "m3",
        suffix: "m^3",
        name: "Volume",
        label: "meters cubed",
    },
    UnitType {
        literal_suffix: "mps",
        suffix: "m/s",
        name: "Velocity",
        label: "meters per second",
    },
    UnitType {
        literal_suffix: "mps2",
        suffix: "m/s^2",
        name: "Acceleration",
        label: "meters per second squared",
    },
    UnitType {
        literal_suffix: "N",
        suffix: "N",
        name: "Force",
        label: "newtons",
    },
    UnitType {
        literal_suffix: "Pa",
        suffix: "Pa",
        name: "Pressure",
        label: "pascals",
    },
    UnitType {
        literal_suffix: "J",
        suffix: "J",
        name: "Energy",
        label: "joules",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/Hz",
        name: "EnergyPerFrequency",
        label: "joules/hertz",
    },
    UnitType {
        literal_suffix: "W",
        suffix: "W",
        name: "Power",
        label: "watts",
    },
    UnitType {
        literal_suffix: "C",
        suffix: "C",
        name: "ElectricCharge",
        label: "coulombs",
    },
    UnitType {
        literal_suffix: "V",
        suffix: "V",
        name: "ElectricPotential",
        label: "volts",
    },
    UnitType {
        literal_suffix: "F",
        suffix: "F",
        name: "Capacitance",
        label: "farads",
    },
    UnitType {
        literal_suffix: "ohms",
        suffix: "ohms",
        name: "ElectricResistance",
        label: "ohms",
    },
    UnitType {
        literal_suffix: "S",
        suffix: "S",
        name: "ElectricConductance",
        label: "siemens",
    },
    UnitType {
        literal_suffix: "Wb",
        suffix: "Wb",
        name: "MagneticFlux",
        label: "webers",
    },
    UnitType {
        literal_suffix: "T",
        suffix: "T",
        name: "MagneticFluxDensity",
        label: "teslas",
    },
    UnitType {
        literal_suffix: "H",
        suffix: "H",
        name: "Inductance",
        label: "henry",
    },
    UnitType {
        literal_suffix: "degreeC",
        suffix: "C",
        name: "Temperature",
        label: "degrees Celcuis",
    },
    UnitType {
        literal_suffix: "lm",
        suffix: "lm",
        name: "LuminousFlux",
        label: "lumens",
    },
    UnitType {
        literal_suffix: "lx",
        suffix: "lx",
        name: "Illuminance",
        label: "lux",
    },
    UnitType {
        literal_suffix: "Pas",
        suffix: "Pas",
        name: "DynamicViscosity",
        label: "pascal seconds",
    },
    UnitType {
        literal_suffix: "Nm",
        suffix: "Nm",
        name: "MomentOfForce",
        label: "newton meters",
    },
    UnitType {
        literal_suffix: "",
        suffix: "rad/s",
        name: "AngularVelocity",
        label: "radians per second",
    },
    UnitType {
        literal_suffix: "",
        suffix: "rad/s^2",
        name: "AngularAcceleration",
        label: "radians per second squared",
    },
    UnitType {
        literal_suffix: "",
        suffix: "N/m",
        name: "SurfaceTension",
        label: "newtons per meter",
    },
    UnitType {
        literal_suffix: "",
        suffix: "W/m^2",
        name: "HeatFluxDensity",
        label: "watts per meter squared",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/K",
        name: "HeatCapacity",
        label: "joules per kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/(kg*K)",
        name: "SpecificHeatCapacity",
        label: "joules per kilogram kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/kg",
        name: "SpecificEnergy",
        label: "joules per kilogram",
    },
    UnitType {
        literal_suffix: "",
        suffix: "W/(m*K)",
        name: "ThermalConductivity",
        label: "watts per meter kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/m^3",
        name: "EnergyDensity",
        label: "joules per meter cubed",
    },
    UnitType {
        literal_suffix: "",
        suffix: "V/m",
        name: "ElectricFieldStrength",
        label: "volts per meter",
    },
    UnitType {
        literal_suffix: "",
        suffix: "C/m^3",
        name: "ElectricChargeDensity",
        label: "coulombs per meter cubed",
    },
    UnitType {
        literal_suffix: "",
        suffix: "C/m^2",
        name: "ElectricFluxDensity",
        label: "coulombs per meter squared",
    },
    UnitType {
        literal_suffix: "",
        suffix: "F/m",
        name: "Permittivity",
        label: "farads per meter",
    },
    UnitType {
        literal_suffix: "",
        suffix: "H/m",
        name: "Permeability",
        label: "henry per meter",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/mol",
        name: "MolarEnergy",
        label: "joules per mole",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/(mol*K)",
        name: "MolarHeatCapacity",
        label: "joules per mole kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "W/(m^2*sr)",
        name: "Radiance",
        label: "watts per meter squared steradian",
    },
    UnitType {
        literal_suffix: "",
        suffix: "kg*K",
        name: "MassThermodynamicTemperature",
        label: "kilograms kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "m*K",
        name: "LengthThermodynamicTemperature",
        label: "meters kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "mol*K",
        name: "AmountOfSubstanceThermodynamicTemperature",
        label: "moles kelvin",
    },
    UnitType {
        literal_suffix: "",
        suffix: "m^2*sr",
        name: "AreaSolidAngle",
        label: "meters squared steradian",
    },
    UnitType {
        literal_suffix: "",
        suffix: "1/mol",
        name: "PerAmountOfSubstance",
        label: "per mole",
    },
    UnitType {
        literal_suffix: "",
        suffix: "J/Hz",
        name: "EnergyPerFrequency",
        label: "joules per hertz",
    },
    UnitType {
        literal_suffix: "",
        suffix: "kg/m^3",
        name: "MassDensity",
        label: "kilograms per meter cubed",
    },
];

fn find_unit(name: String) -> &'static UnitType {
    for unit in UNITS {
        if name == unit.name {
            return &unit;
        }
    }
    panic!("not unit named {} was found", name);
}

fn impl_display_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_string = name.to_string();
    let current_unit = find_unit(name_string);
    let label: &'static str = current_unit.label;
    let suffix: &'static str = current_unit.suffix;
    let literal_suffix = current_unit.literal_suffix;
    let gen = quote::quote! {
      impl fmt::Display for #name {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{} {}", self.0, #label)
        }
      }

      impl std::fmt::Debug for #name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
          f.debug_struct(stringify!(#name))
              .field("value", &self.to_string())
              .field("label", &#label)
              .field("suffix", &#suffix)
              .field("literal_suffix", &#literal_suffix)
              .finish()
        }
      }
    };
    gen.into()
}

// "visitor" that visits every node in the syntax tree
// we add our own behavior to replace custom literals with proper Rust code
struct LiteralReplacer;

impl VisitMut for LiteralReplacer {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        if let Expr::Lit(ExprLit { lit, .. }) = i {
            match lit {
                Lit::Int(lit) => {
                    // get literal suffix
                    // get literal without suffix
                    let lit_no_suffix = LitInt::new(lit.base10_digits(), lit.span());
                    for unit in UNITS {
                        let value = process_unit(lit.suffix(), &lit_no_suffix, &unit);
                        match value {
                            Some(matched_value) => {
                                *i = matched_value;
                                break;
                            }
                            None => (),
                        }
                    }
                }
                _ => (), // other literal type we won't modify
            }
        } else {
            // not a literal, use default visitor method
            visit_mut::visit_expr_mut(self, i)
        }
    }
}
