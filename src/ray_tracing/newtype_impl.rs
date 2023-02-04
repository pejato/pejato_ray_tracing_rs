// TODO
macro_rules! _newtype_impl {
    (impl $__trait: ident for $__type: ident { fn $__method: ident -> $__out: ty }) => {
        impl $__trait for $__type {
            fn $__method(self) -> $__out {
                let $__type(a) = self;
                a.$__method()
            }
        }
    };
}
