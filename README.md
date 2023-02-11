## About

Slender Math is a lightweight, easy to use math library targeted at game development.  
It provides 2, 3 and 4 component vectors, 2x3 and 4x4 matrices as well as quaternions.  
All structs are thin wrappers around Rusts portable SIMD types with an intuitive API.

The main design goal of Slender Math is to be as easy to use as possible.  
As such it is only optimized for speed as far as doesn't conflict with that goal.  
Nevertheless it will be faster than any math library not using SIMD.

### Stability

As long as Rusts portable SIMD is not stable, neither will Slender Math be.  
Until the API is stabilized, new Rust versions may break Slender Math temporarily.
