Converts all borrowed values in this builder to owned values, then infers _any_
new lifetime for the builder, which may be `'static`.

This can be used to return builders from functions even when those may
otherwise hold borrowed values.

Calling this function should be cheap when there are only owned values in the
builder. If there are borrowed values, calling this function will allocate.
