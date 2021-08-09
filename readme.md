# `hushed_panic`

A crate for hushing panics for a specific thread. 

This is especially useful for when you want to hush tests which are intended to panic, but do not want to see the long output.

Usage in a test:

```rs
fn my_test() {
    let _x = hushed_panic::hush_this_test();
    panic!(); // Won't print anything!
    drop(_x);
    panic!(); // Would print normally!
}
```
