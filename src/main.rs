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
        // Miles
        "mile"        => Some(Length::from_miles(input_length)),
        "miles"       => Some(Length::from_miles(input_length)),
        "mi"          => Some(Length::from_miles(input_length)),
        // Furlongs
        "furlong"     => Some(Length::from_furlongs(input_length)),
        "furlongs"    => Some(Length::from_furlongs(input_length)),
        "f"           => Some(Length::from_furlongs(input_length)),
        "fur"         => Some(Length::from_furlongs(input_length)),
        // Yards
        "yard"        => Some(Length::from_yards(input_length)),
        "yards"       => Some(Length::from_yards(input_length)),
        "y"           => Some(Length::from_yards(input_length)),
        "yd"          => Some(Length::from_yards(input_length)),
        "yds"         => Some(Length::from_yards(input_length)),
        // Feet
        "feet"        => Some(Length::from_feet(input_length)),
        "foot"        => Some(Length::from_feet(input_length)),
        "ft"          => Some(Length::from_feet(input_length)),
        "'"           => Some(Length::from_feet(input_length)),
        // Inches
        "inch"        => Some(Length::from_inches(input_length)),
        "inches"      => Some(Length::from_inches(input_length)),
        "i"           => Some(Length::from_inches(input_length)),
        "in"          => Some(Length::from_inches(input_length)),
        "\""          => Some(Length::from_inches(input_length)),
        // Thou
        "thou"        => Some(Length::from_inches(input_length / 1000f64)),
        "th"          => Some(Length::from_inches(input_length / 1000f64)),
        // Kilometers
        "kilometer"   => Some(Length::from_kilometers(input_length)),
        "kilometers"  => Some(Length::from_kilometers(input_length)),
        "km"          => Some(Length::from_kilometers(input_length)),
        // Hectometers
        "hectometer"  => Some(Length::from_hectometers(input_length)),
        "hectometers" => Some(Length::from_hectometers(input_length)),
        "hm"          => Some(Length::from_hectometers(input_length)),
        // Meters
        "meter"       => Some(Length::from_meters(input_length)),
        "meters"      => Some(Length::from_meters(input_length)),
        "metre"       => Some(Length::from_meters(input_length)),
        "metres"      => Some(Length::from_meters(input_length)),
        "m"           => Some(Length::from_meters(input_length)),
        // Decimeters
        "decimeter"   => Some(Length::from_decimeters(input_length)),
        "decimeters"  => Some(Length::from_decimeters(input_length)),
        "decimetre"   => Some(Length::from_decimeters(input_length)),
        "decimetres"  => Some(Length::from_decimeters(input_length)),
        "dm"          => Some(Length::from_decimeters(input_length)),
        // Centimeters
        "centimeter"  => Some(Length::from_centimeters(input_length)),
        "centimeters" => Some(Length::from_centimeters(input_length)),
        "centimetre"  => Some(Length::from_centimeters(input_length)),
        "centimetres" => Some(Length::from_centimeters(input_length)),
        "cm"          => Some(Length::from_centimeters(input_length)),
        // Millimeters
        "millimeter"  => Some(Length::from_millimeters(input_length)),
        "millimeters" => Some(Length::from_millimeters(input_length)),
        "millimetre"  => Some(Length::from_millimeters(input_length)),
        "millimetres" => Some(Length::from_millimeters(input_length)),
        "mm"          => Some(Length::from_millimeters(input_length)),
        // Nanometers
        "nanometer"   => Some(Length::from_nanometers(input_length)),
        "nanometers"  => Some(Length::from_nanometers(input_length)),
        "nanometre"   => Some(Length::from_nanometers(input_length)),
        "nanometres"  => Some(Length::from_nanometers(input_length)),
        "nm"          => Some(Length::from_nanometers(input_length)),
        _           => { eprintln!("Unknown unit of input length: {}", length_unit); None }
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
                // Miles
                "mile"        => Some(l.as_miles()),
                "miles"       => Some(l.as_miles()),
                "mi"          => Some(l.as_miles()),
                // Furlongs
                "furlong"     => Some(l.as_furlongs()),
                "furlongs"    => Some(l.as_furlongs()),
                "f"           => Some(l.as_furlongs()),
                "fur"         => Some(l.as_furlongs()),
                // Yards
                "yard"        => Some(l.as_yards()),
                "yards"       => Some(l.as_yards()),
                "y"           => Some(l.as_yards()),
                "yd"          => Some(l.as_yards()),
                "yds"         => Some(l.as_yards()),
                // Feet
                "feet"        => Some(l.as_feet()),
                "foot"        => Some(l.as_feet()),
                "ft"          => Some(l.as_feet()),
                "'"           => Some(l.as_feet()),
                // Inches
                "inch"        => Some(l.as_inches()),
                "inches"      => Some(l.as_inches()),
                "i"           => Some(l.as_inches()),
                "in"          => Some(l.as_inches()),
                "\""          => Some(l.as_inches()),
                // Thou
                "thou"        => Some(l.as_inches() / 1000f64),
                "th"          => Some(l.as_inches() / 1000f64),
                // Kilometers
                "kilometer"   => Some(l.as_kilometers()),
                "kilometers"  => Some(l.as_kilometers()),
                "km"          => Some(l.as_kilometers()),
                // Hectometers
                "hectometer"  => Some(l.as_hectometers()),
                "hectometers" => Some(l.as_hectometers()),
                "hm"          => Some(l.as_hectometers()),
                // Meters
                "meter"       => Some(l.as_meters()),
                "meters"      => Some(l.as_meters()),
                "metre"       => Some(l.as_meters()),
                "metres"      => Some(l.as_meters()),
                "m"           => Some(l.as_meters()),
                // Decimeters
                "decimeter"   => Some(l.as_decimeters()),
                "decimeters"  => Some(l.as_decimeters()),
                "decimetre"   => Some(l.as_decimeters()),
                "decimetres"  => Some(l.as_decimeters()),
                "dm"          => Some(l.as_decimeters()),
                // Centimeters
                "centimeter"  => Some(l.as_centimeters()),
                "centimeters" => Some(l.as_centimeters()),
                "centimetre"  => Some(l.as_centimeters()),
                "centimetres" => Some(l.as_centimeters()),
                "cm"          => Some(l.as_centimeters()),
                // Millimeters
                "millimeter"  => Some(l.as_millimeters()),
                "millimeters" => Some(l.as_millimeters()),
                "millimetre"  => Some(l.as_millimeters()),
                "millimetres" => Some(l.as_millimeters()),
                "mm"          => Some(l.as_millimeters()),
                // Nanometers
                "nanometer"   => Some(l.as_nanometers()),
                "nanometers"  => Some(l.as_nanometers()),
                "nanometre"   => Some(l.as_nanometers()),
                "nanometres"  => Some(l.as_nanometers()),
                "nm"          => Some(l.as_nanometers()),
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
        Some(f) => println!("{} {}", f, output_units)
    }
}
