Rust uses u8 type for byte values

since the ASCII code for A is 65, the literals b'A' and
65u8 are exactly equivalent. Only ASCII characters may appear in byte literals.

Rust uses the char type for single characters in isolation, but uses the UTF-8 encod‐
ing for strings and streams of text. So, a String represents its text as a sequence of
UTF-8 bytes, not as an array of characters.

1 mutable read-write reference OR multiple immutable readonly refs.  not both at the same time