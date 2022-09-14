use std::borrow::Cow;
use std::str;

pub struct StrStrU8Codec;

impl<'a> heed::BytesDecode<'a> for StrStrU8Codec {
    type DItem = (&'a str, &'a str, u8);

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        let (n, bytes) = bytes.split_first()?;
        let s1_end = bytes.iter().position(|b| *b == 0)?;
        let (s1_bytes, rest) = bytes.split_at(s1_end);
        let s2_bytes = &rest[1..];
        let s1 = str::from_utf8(s1_bytes).ok()?;
        let s2 = str::from_utf8(s2_bytes).ok()?;
        Some((s1, s2, *n))
    }
}

impl<'a> heed::BytesEncode<'a> for StrStrU8Codec {
    type EItem = (&'a str, &'a str, u8);

    fn bytes_encode((s1, s2, n): &Self::EItem) -> Option<Cow<[u8]>> {
        let mut bytes = Vec::with_capacity(s1.len() + s2.len() + 1);
        bytes.push(*n);
        bytes.extend_from_slice(s1.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(s2.as_bytes());
        Some(Cow::Owned(bytes))
    }
}
pub struct UncheckedStrStrU8Codec;

impl<'a> heed::BytesDecode<'a> for UncheckedStrStrU8Codec {
    type DItem = (&'a [u8], &'a [u8], u8);

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        let (n, bytes) = bytes.split_first()?;
        let s1_end = bytes.iter().position(|b| *b == 0)?;
        let (s1_bytes, rest) = bytes.split_at(s1_end);
        let s2_bytes = &rest[1..];
        Some((s1_bytes, s2_bytes, *n))
    }
}

impl<'a> heed::BytesEncode<'a> for UncheckedStrStrU8Codec {
    type EItem = (&'a [u8], &'a [u8], u8);

    fn bytes_encode((s1, s2, n): &Self::EItem) -> Option<Cow<[u8]>> {
        let mut bytes = Vec::with_capacity(s1.len() + s2.len() + 1);
        bytes.push(*n);
        bytes.extend_from_slice(s1);
        bytes.push(0);
        bytes.extend_from_slice(s2);
        Some(Cow::Owned(bytes))
    }
}
