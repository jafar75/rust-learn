If a type implements `Drop`, it cannot implement the `Copy` trait. If a type is Copy, that
means that simple byte-for-byte duplication is sufficient to produce an independent
copy of the value. But it is typically a mistake to call the same drop method more than
once on the same data.

Drop, Sized, Clone, Copy, Deref, DerefMut, Borrow, BorrowMut, From, Into, TryFrom, TryInto, ToOwned