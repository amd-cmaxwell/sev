from .MOD_NAME import *  # NOQA

# Re-export internal symbols needed by dependent FFI crates.
# UniFFI's generated code references _Uniffi* helpers across packages,
# but `import *` skips underscore-prefixed names.
from . import MOD_NAME as _mod
for _k in list(vars(_mod)):
    if _k.startswith('_Uniffi'):
        globals()[_k] = getattr(_mod, _k)
del _mod, _k
