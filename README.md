# Concatenates identifiers. 

Yet again, I am making a crate with the amount of code that is *actually necessary*:
<50 SLOC, 0 dependencies.

```
use ident_concat::ident;
let ident!(a b) = 4;
assert_eq!(ab, ident!(a b));
```
