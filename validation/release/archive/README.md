# Historical cache intervention

`cache_lifetime.py` is the unchanged private guard intervention used for frozen
a35104a and reviewed in decision 0109. It requires the pre-promotion source and
is retained only to reproduce that historical experiment. Its original reports
and raw evidence remain unchanged.

Production now registers the same local-cache owner in `predictor_ffi.rs`.
The current `c_concurrency.py` tests the installed production library and an
explicit caller-cleanup control; it neither injects a second owner nor requires
a leaking production run.
