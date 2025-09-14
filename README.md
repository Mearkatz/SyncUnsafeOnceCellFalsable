Contains a single type meant to allow for multi-threading of a prime-sieve. It would also work with any other sieve operating on an array of bools.


## SyncUnsafeOnceCellFalsable

Not a very catchy name but I figured it was better than my original name for it "WeirdBool".
This is the primary type in the crate and has interior mutability given you have an immutable reference to it.

Here's its documentation:

> A `bool` that is initially always `true`. It may be set from `true` to `false`, but never the other way around. This makes it thread safe since it can only effectively be written to once.


