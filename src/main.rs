/**
 * unit-converter:
 *
 * This tool allows quick conversion between units on the commandline. I built this as I wanted
 * to go between metric and imperial lengths quickly; currently it only implements lengths.
 * This code is garbage, but feel free to use it if it is useful to you. I have GPLv3 licensed it.
 * Special thanks to the author of the measurements crate which does virtually all of the work.
 *
 * Usage: ./unit-converter 6 in mm
 *        ./unit-converter 11.7 feet miles
 */
extern crate measurements;
extern crate structopt;

use measurements::Length;
use structopt::StructOpt;


/**
 * to_length:
 * Given a scalar (input_length) and a unit, create a Length
 * object. If the unit is unrecognized, return None.
 *
 * Arguments:
 *      input_length (f64)
 *      length_unit  (String)
 *
 * Returns: Option<Length>
 */
fn to_length(input_length: f64, length_unit: String) -> Option<Length> {
    match length_unit.to_lowercase().trim().as_ref() {
        /* IMPERIAL LENGTH UNITS: */
        "mile" | "miles" | "mi"                                            => Some(Length::from_miles(input_length)),            // Miles
        "furlong" | "furlongs" | "fur" | "furs"                            => Some(Length::from_furlongs(input_length)),         // Furlongs
        "yard" | "yards" | "y" | "yd" | "yds"                              => Some(Length::from_yards(input_length)),            // Yards
        "foot" | "feet" | "ft" | "'"                                       => Some(Length::from_feet(input_length)),             // Feet
        "inch" | "inches" | "i" | "in" | "\""                              => Some(Length::from_inches(input_length)),           // Inches
        "thou" | "th"                                                      => Some(Length::from_inches(input_length / 1000f64)), // Thou
        /* METRIC LENGTH UNITS: */
        "kilometer" | "kilometers" | "kilometre" | "kilometres" | "km"     => Some(Length::from_kilometers(input_length)),       // Kilometers
        "hectometer" | "hectometers" | "hectometre" | "hectometres"| "hm"  => Some(Length::from_hectometers(input_length)),      // Hectometers
        "meter" | "meters" | "metre" | "metres" | "m"                      => Some(Length::from_meters(input_length)),           // Meters
        "decimeter" | "decimeters" | "decimetre" | "decimetres"| "dm"      => Some(Length::from_decimeters(input_length)),       // Decimeters
        "centimeter" | "centimeters" | "centimetre" | "centimetres"| "cm"  => Some(Length::from_centimeters(input_length)),      // Centimeters
        "millimeter" | "millimeters" | "millimetre" | "millimetres"| "mm"  => Some(Length::from_millimeters(input_length)),      // Millimeters
        "nanometer" | "nanometers" | "nanometre" | "nanometres"| "nm"      => Some(Length::from_nanometers(input_length)),       // Nanometers
        /* OTHER: */
        _           => { eprintln!("Unknown unit of input length: {}", length_unit); None }
    }
}


/**
 * get_cannonical_unit_name:
 * Given a String containing a unit name, return the "cannonical name" for that unit.
 * This will be the fully spelled-out, plural American English variant.
 */
fn get_cannonical_unit_name(length_unit: String) -> String {
    match length_unit.to_lowercase().trim().as_ref() {
        /* IMPERIAL LENGTH UNITS: */
        "mile" | "miles" | "mi"                                            => "miles".to_string(),
        "furlong" | "furlongs" | "fur" | "furs"                            => "furlongs".to_string(),
        "yard" | "yards" | "y" | "yd" | "yds"                              => "yards".to_string(),
        "foot" | "feet" | "ft" | "'"                                       => "feet".to_string(),
        "inch" | "inches" | "i" | "in" | "\""                              => "inches".to_string(),
        "thou" | "th"                                                      => "thou".to_string(),
        /* METRIC LENGTH UNITS: */
        "kilometer" | "kilometers" | "kilometre" | "kilometres" | "km"     => "kilometers".to_string(),
        "hectometer" | "hectometers" | "hectometre" | "hectometres"| "hm"  => "hectometers".to_string(),
        "meter" | "meters" | "metre" | "metres" | "m"                      => "meters".to_string(),
        "decimeter" | "decimeters" | "decimetre" | "decimetres"| "dm"      => "decimeters".to_string(),
        "centimeter" | "centimeters" | "centimetre" | "centimetres"| "cm"  => "centimeters".to_string(),
        "millimeter" | "millimeters" | "millimetre" | "millimetres"| "mm"  => "millimeters".to_string(),
        "nanometer" | "nanometers" | "nanometre" | "nanometres"| "nm"      => "nanometers".to_string(),
        /* OTHER: */
        _                                                                  => length_unit
    }
}


/**
 * from_length:
 * Given a Length object and an output unit, return the matching scalar.
 *
 * Arguments:
 *      length (Option<Length>)  -- The Length object.
 *      output_unit  (String)      -- The length unit to output.
 *
 * Returns: Option<f64>
 */
fn from_length(length: Option<Length>, output_unit: String) -> Option<f64> {
    match length {
        None     => None,
        Some(l)  => {
            match output_unit.to_lowercase().trim().as_ref() {
                /* IMPERIAL LENGTH UNITS: */
                "mile" | "miles" | "mi"                                            => Some(l.as_miles()),            // Miles
                "furlong" | "furlongs" | "fur" | "furs"                            => Some(l.as_furlongs()),         // Furlongs
                "yard" | "yards" | "y" | "yd" | "yds"                              => Some(l.as_yards()),            // Yards
                "foot" | "feet" | "ft" | "'"                                       => Some(l.as_feet()),             // Feet
                "inch" | "inches" | "i" | "in" | "\""                              => Some(l.as_inches()),           // Inches
                "thou" | "th"                                                      => Some(l.as_inches() / 1000f64), // Thou
                /* METRIC LENGTH UNITS: */
                "kilometer" | "kilometers" | "kilometre" | "kilometres" | "km"     => Some(l.as_kilometers()),       // Kilometers
                "hectometer" | "hectometers" | "hectometre" | "hectometres"| "hm"  => Some(l.as_hectometers()),      // Hectometers
                "meter" | "meters" | "metre" | "metres" | "m"                      => Some(l.as_meters()),           // Meters
                "decimeter" | "decimeters" | "decimetre" | "decimetres"| "dm"      => Some(l.as_decimeters()),       // Decimeters
                "centimeter" | "centimeters" | "centimetre" | "centimetres"| "cm"  => Some(l.as_centimeters()),      // Centimeters
                "millimeter" | "millimeters" | "millimetre" | "millimetres"| "mm"  => Some(l.as_millimeters()),      // Millimeters
                "nanometer" | "nanometers" | "nanometre" | "nanometres"| "nm"      => Some(l.as_nanometers()),       // Nanometers
                /* OTHER: */
                _           => { eprintln!("Unknown unit of output length: {}", output_unit); None }
            }
        }
    }
}


/**
 * This struct is used by StructOpt to parse
 * the commandline arguments to this script.
 */
#[derive(StructOpt, Debug)]
struct Opt {
    input_length: f64,
    input_units: String,
    output_units: String,
}


/**
 * main:
 * Parse the commandline arguments to get the length, input units and
 * output units. Then create a length type with `to_length`, then use
 * `from_length` to get the relevant output unit scalar.
 *
 * Returns: None
 */
fn main() {
    // TODO: Add support for units other than lengths.
    let opt = Opt::from_args();
    // TODO: What is the correct way to borrow this in Rust?
    let output_units: String = opt.output_units.clone();
    match from_length(to_length(opt.input_length, opt.input_units), opt.output_units) {
        None    => println!("Could not complete conversion"),
        Some(f) => println!("{} {}", f, get_cannonical_unit_name(output_units))
    }
}
