### Background
- I want to produce a `.dll` (or `.so`, `.dylib`, ...) in project `a`.
- In project `b` I want to test `a.dll` actually works.


### Problem

Compiling this on Windows produces

```
  = note: LINK : fatal error LNK1181: cannot open input file 'a_library.lib'
``` 

Which is sort of correct, because Rust only produces

```
06/28/2019  08:42           131,584 a_library.dll
06/28/2019  08:42             1,912 a_library.dll.lib
```