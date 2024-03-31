references must never outlive their referents.

Shared references are Copy. BUT Mutable references are not Copy.

Rust requires types that contain references to take explicit lifetime parameters

