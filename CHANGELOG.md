# v0.2.0

Includes significant improvements to usability, specifically in defining custom integer and decimal literals.

## Renamed the modules that we expect at `crate::custom_literal` to be more descriptive

|old|new|
|---|---|
|`int`|`integer`|
|`float`|`decimal`|
|`char`|`character`|
|`byte_char`|`byte_character`|
|`byte_str`|`byte_string`|
|`str`|`string`|
|`c_str`|`c_string`|

We renamed from Float because float is too-specific to the format but decimal is a more general name for what we actually give you

## The signature of custom integer literal has changed.

- No more base, we handle that for you.
- No more strings, you get the actual number.

```rs
10km // crate::custom_literal::integer::km!(10)
0b10km // crate::custom_literal::integer::km!(3)
-10km // -crate::custom_literal::integer::km!(10)
```

Limitation: The absolute value of the custom literal may not exceed `340_282_366_920_938_463_463_374_607_431_768_211_455`

## The signature of custom decimal literal has changed

- No more strings. Fractional, integral and the exponent parts are now numbers.
Exponent also contains the `-` sign

```rs
10.0km // crate::custom_literal::decimal::km!(10 0 1)
10e7km // crate::custom_literal::decimal::km!(10 0 7)
10e-7km // crate::custom_literal::decimal::km!(10 0 -7)
-10e-7km // -crate::custom_literal::decimal::km!(10 0 -7)
-10.4e7km // -crate::custom_literal::decimal::km!(10 4 7)
```

Limitation: Each of these may not exceed `340_282_366_920_938_463_463_374_607_431_768_211_455`:

- Integral part (part before the decimal point)
- Fractional part (part after the decimal point, before the exponent)
- Exponent

# v0.1.0

Initial release
