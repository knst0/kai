macro_rules! media_field_enum {
    (
        $enum_name:ident,
        common: [ $( $cvariant:ident => $cstr:literal ),+ $(,)? ],
        extra: [ $( $evariant:ident => $estr:literal ),* $(,)? ]
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $enum_name {
            $( $cvariant, )+
            $( $evariant, )*
        }

        impl $enum_name {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $( Self::$cvariant => $cstr, )+
                    $( Self::$evariant => $estr, )*
                }
            }
        }
    };
}

pub(crate) use media_field_enum;
