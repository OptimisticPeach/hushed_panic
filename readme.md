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

# License

`hushed_panic` is distributed under the terms of either the MIT license, or the Apache License (Version
2.0)
at the user's choice.

See the files named LICENSE-MIT and LICENSE-APACHE2 relative to the root directory of this project
for more details.
