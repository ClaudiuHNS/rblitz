macro_rules! make_bitfield {
    (
        $(#[$outer:meta])*
        pub struct $bitfield:ident = $var:ident:$t:ty {
            $(
                $field:ident: $fieldtype:ty = $value:expr,
            )+
        }
    ) => {
        $(#[$outer])*
        pub struct $bitfield {
            $(
                pub $field: $fieldtype,
            )+
        }

        impl<'de> serde::Deserialize<'de> for $bitfield {
            fn deserialize<D>(d: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                let $var: $t = serde::Deserialize::deserialize(d)?;
                Ok($bitfield {
                    $(
                        $field: $value,
                    )+
                })
            }
        }
    };
}

macro_rules! make_fixed_string {
    ($ident:ident $e:expr) => {
        pub(in crate) mod $ident {
            use serde::de::{SeqAccess, Visitor};
            pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
                where
                    D: serde::Deserializer<'de>,
            {
                struct FixedStringVisitor;

                impl<'de> Visitor<'de> for FixedStringVisitor {
                    type Value = String;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("seq")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                        where
                            A: SeqAccess<'de>,
                    {
                        let string: String = crate::util::seq_next_elem(&mut seq)?;
                        for _ in string.len()..$e {
                            crate::util::seq_next_elem::<_, u8>(&mut seq)?;
                        }
                        Ok(string)
                    }
                }

                d.deserialize_seq(FixedStringVisitor)
            }
            pub fn serialize<S>(string: &str, s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
            {
                let mut bytes: [u8; $e] = [0; $e];
                let len = string.len().min($e - 1);
                bytes[..len].copy_from_slice(&string.as_bytes()[..len]);
                s.serialize_bytes(&bytes)
            }
        }
    }
}

macro_rules! make_sized_vec {
    ($ident:ident $e:ty) => {
        pub(in crate) mod $ident {
            pub fn deserialize<'de, D, T: 'de>(d: D) -> Result<Vec<T>, D::Error>
                where
                    D: serde::Deserializer<'de>,
                    T: serde::Deserialize<'de>,
            {
                use core::marker::PhantomData;
                use serde::de::{SeqAccess, Visitor};

                struct SizedVecVisitor<'de, T: serde::Deserialize<'de>>(PhantomData<&'de T>);

                impl<'de, T: serde::Deserialize<'de>> Visitor<'de> for SizedVecVisitor<'de, T> {
                    type Value = Vec<T>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("seq")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                        where
                            A: SeqAccess<'de>,
                    {
                        let len: $e = crate::util::seq_next_elem(&mut seq)?;
                        let mut buf: Vec<T> = Vec::with_capacity(len as usize);
                        for _ in 0..len {
                            buf.push(
                                crate::util::seq_next_elem(&mut seq)?);
                        }
                        Ok(buf)
                    }
                }

                d.deserialize_seq(SizedVecVisitor(PhantomData))
            }
            pub fn serialize<S, T>(buf: &[T], s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                    T: serde::Serialize,
            {
                use serde::{ser::Error, ser::SerializeTuple};
                const MAX: $e = !0;
                let mut s = s.serialize_tuple(buf.len())?;
                if buf.len() > MAX as usize {
                    Err(Error::custom(crate::Error::TooMuchData(
                        buf.len(),
                        MAX as usize,
                    )))
                } else {
                    s.serialize_element(&(buf.len() as $e))?;
                    s.serialize_element(buf)?;
                    s.end()
                }
            }
        }
    }
}
