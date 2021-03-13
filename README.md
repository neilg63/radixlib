#RADIX Conversion Library

This RUST library facilitates accurate conversion of different number systems, including fractions expressed either via place-value notation or as rational fractions. It also exposes a method to approximate any decimal fraction to the nearest rational fraction.

Javascript does have good support for the conversion of integers in bases as high as 36, which can be represented by the digits 0-9 and letters a to z, via the Number.toString(baseNumber) and parseInt(stringValue, baseNumber) methods. However, supporting fractions with a high degree of accuracy is problematic.

I wanted to compare all systems with place value notation from 2 to 60. Bases greater than 36 are represented by colon-separated pairs of decimal digits.
