Unlike C++, in Rust the protected data is stored `inside` the Mutex

```

use std::sync::Arc;
let app = Arc::new(FernEmpireApp {
...
waiting_list: Mutex::new(vec![]),
...
});

```

`Arc` is handy for sharing things across threads, and `Mutex` is handy for
mutable data that’s shared across threads.


The **only way** to get at the data is to call the `.lock()` method:

`let mut guard = self.waiting_list.lock().unwrap();`


Rust’s type system is telling us what Mutex does. It dynamically enforces exclusive
access, something that’s usually done statically, at compile time, by the Rust compiler.
