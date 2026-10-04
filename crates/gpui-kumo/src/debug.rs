// Keep safe fields explicit; omitted values, callbacks and Base internals stay opaque.
macro_rules! debug_struct {
    ($name:ident { $($fields:tt)* }) => {
        debug_struct!(@impl [impl] $name { $($fields)* });
    };
    ([$($bounds:tt)*] $name:ident<$param:ident> { $($fields:tt)* }) => {
        debug_struct!(@impl [impl<$($bounds)*>] $name<$param> { $($fields)* });
    };
    (@impl [$($implementation:tt)*] $name:ident $(<$param:ident>)? {
        $($field:ident $(: |$this:ident| $value:expr)?),* $(,)?
    }) => {
        $($implementation)* std::fmt::Debug for $name $(<$param>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug = f.debug_struct(stringify!($name));
                $(debug_struct!(@field debug, self, $field $(: |$this| $value)?);)*
                debug.finish_non_exhaustive()
            }
        }
    };
    (@field $debug:ident, $self:ident, $field:ident) => {
        $debug.field(stringify!($field), &$self.$field);
    };
    (@field $debug:ident, $self:ident, $field:ident: |$this:ident| $value:expr) => {{
        let $this = $self;
        $debug.field(stringify!($field), &$value);
    }};
}
