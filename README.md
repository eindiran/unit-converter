# unit-converter
Simple Rust program for converting between metric and imperial units on the commandline.

This tool is something that I threw together quickly to allow converting between imperial and metric units of length. Other unit types (eg volume, pressure, temperature) are TODO. The bulk of the work is handled by the [measurements crate](https://crates.io/crates/measurements).

Usage:
```
# Convert inches to millimeters:
./unit-converter 6.5 inches mm
# Convert millimeters to hectometers
./unit-converter 100023 mm hectometers
# Convert furlongs to feet
./unit-converter 3.777 fur feet
```
