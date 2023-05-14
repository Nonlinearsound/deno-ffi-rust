# Build instructions

To build the Rust code, call:

```
cargo build
```

To run the Deno part, call:

```
deno run --allow-ffi --unstable main.ts
```

# Remarks

For typed arrays, Deno now uses ["buffer"] instead of ["pointer"] in their function definition inside of Deno.dlopen().

Everything else in the client code stays the same. You will use something like an Int32Array or alike and pass it to the function if you have a parameter like int* on the C++ side.

Same goes for char* types as function arguments of return types.