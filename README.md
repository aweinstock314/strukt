# strukt

`strukt` is a library for ad-hoc [de]serialization inspired by Python's [struct](https://docs.python.org/2/library/struct.html) module. 

## Examples

```rust
pack!(">H")(1337) == [5, 57]
pack!("<Q")(0x41424344) == b"DCBA\0\0\0\0"
pack!("<hhI")(-1, 2, 42) == b"\xff\xff\x02\x00\x2a\x00\x00\x00"
```
