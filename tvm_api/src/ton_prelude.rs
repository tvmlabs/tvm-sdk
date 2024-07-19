// Copyright (C) 2019-2023 EverX. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

#![allow(non_camel_case_types)]

use std::any::type_name;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Read;
use std::io::Write;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use extfmt::Hexlify;
use ordered_float::OrderedFloat;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tvm_block::error;

use crate::ton::Bool;
use crate::AnyBoxedSerialize;
use crate::BareDeserialize;
use crate::BareSerialize;
use crate::BoxedDeserialize;
use crate::BoxedSerialize;
use crate::ConstructorNumber;
use crate::Deserializer;
use crate::Result;
use crate::Serializer;

const MAX_BYTES_DEBUG_LEN: usize = 4;

macro_rules! impl_byteslike {
    (@common $ty:ident) => {

        impl fmt::Debug for $ty {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.len() <= MAX_BYTES_DEBUG_LEN {
                    write!(f, "<{}>", Hexlify(&self.0))
                } else {
                    write!(f, "<{}... {} bytes>", Hexlify(&self.0[..MAX_BYTES_DEBUG_LEN]), self.0.len())
                }
            }
        }

        impl ::std::ops::Deref for $ty {
            type Target = [u8];
            fn deref(&self) -> &[u8] { &self.0 }
        }

        impl ::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut [u8] { &mut self.0 }
        }

    };

    (@arraylike $ty:ident) => {

        impl_byteslike!(@common $ty);

        impl BareDeserialize for $ty {
            fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
                let mut ret = Self::default();
                de.read_exact(&mut ret.0)?;
                Ok(ret)
            }
        }

        impl BareSerialize for $ty {
            fn constructor(&self) -> crate::ConstructorNumber { unreachable!() }
            fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
                ser.write_all(&self.0)?;
                Ok(())
            }
        }

    };
}

/// Represents 128-bit unsigned integer.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct int128(pub [u8; 16]);

/// Represents 256-bit unsigned integer.
pub(crate) type int256 = tvm_block::UInt256;

/// Represents 512-bit unsigned integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct int512(pub [u8; 64]);

impl Default for int512 {
    fn default() -> Self {
        int512([0; 64])
    }
}

impl_byteslike!(@arraylike int128);
impl_byteslike!(@arraylike int512);

/// Represents base TL-object type.
pub struct TLObject(Box<dyn AnyBoxedSerialize>);

impl TLObject {
    pub fn new<I: AnyBoxedSerialize>(inner: I) -> Self {
        TLObject(Box::new(inner))
    }

    pub fn is<I: AnyBoxedSerialize>(&self) -> bool {
        self.0.as_any().is::<I>()
    }

    pub fn downcast<I: AnyBoxedSerialize>(self) -> ::std::result::Result<I, Self> {
        if self.is::<I>() {
            Ok(*self.0.into_boxed_any().downcast::<I>().unwrap())
        } else {
            Err(self)
        }
    }
}

impl Clone for TLObject {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

impl Default for TLObject {
    fn default() -> Self {
        unimplemented!()
    }
}

impl PartialEq for TLObject {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

impl Hash for TLObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        unimplemented!()
    }
}

impl fmt::Debug for TLObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (type_id, _) = self.0.serialize_boxed();
        write!(f, "(TLObject tl_id:{:?})", type_id)
    }
}

impl BoxedDeserialize for TLObject {
    fn possible_constructors() -> Vec<ConstructorNumber> {
        crate::ton::dynamic::BY_NUMBER.keys().cloned().collect()
    }

    fn deserialize_boxed(id: ConstructorNumber, de: &mut Deserializer) -> Result<Self> {
        match crate::ton::dynamic::BY_NUMBER.get(&id) {
            Some(dynamic) => (dynamic.ton)(id, de),
            None => _invalid_id!(id),
        }
    }
}

impl BoxedSerialize for TLObject {
    fn serialize_boxed(&self) -> (ConstructorNumber, &dyn BareSerialize) {
        self.0.serialize_boxed()
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LengthPrefixed<T>(pub T);

impl<T> From<T> for LengthPrefixed<T> {
    fn from(x: T) -> Self {
        LengthPrefixed(x)
    }
}

impl<T> BareDeserialize for LengthPrefixed<T>
where
    T: BoxedDeserialize,
{
    fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
        let len = de.read_i32::<LittleEndian>()? as usize;
        let mut buf = vec![0u8; len];
        de.read_exact(&mut buf)?;
        Ok(LengthPrefixed(T::boxed_deserialized_from_bytes(&buf)?))
    }
}

impl<T> BareSerialize for LengthPrefixed<T>
where
    T: BoxedSerialize,
{
    fn constructor(&self) -> crate::ConstructorNumber {
        let inner = self.0.boxed_serialized_bytes().unwrap_or_default();
        crate::ConstructorNumber(inner.len() as u32)
    }

    fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
        let inner = self.0.boxed_serialized_bytes()?;
        ser.write_i32::<LittleEndian>(inner.len() as i32)?;
        ser.write_all(&inner)?;
        Ok(())
    }
}

impl BareSerialize for () {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, _ser: &mut Serializer) -> Result<()> {
        Ok(())
    }
}

impl From<bool> for &'static Bool {
    fn from(b: bool) -> Self {
        if b { &Bool::BoolTrue } else { &Bool::BoolFalse }
    }
}

impl From<bool> for Bool {
    fn from(b: bool) -> Self {
        let b: &'static Bool = b.into();
        b.clone()
    }
}

impl From<Bool> for bool {
    fn from(val: Bool) -> Self {
        match val {
            Bool::BoolTrue => true,
            Bool::BoolFalse => false,
        }
    }
}

impl BoxedDeserialize for bool {
    fn possible_constructors() -> Vec<ConstructorNumber> {
        Bool::possible_constructors()
    }

    fn deserialize_boxed(id: ConstructorNumber, de: &mut Deserializer) -> Result<Self> {
        Ok(Bool::deserialize_boxed(id, de)?.into())
    }
}

impl BoxedSerialize for bool {
    fn serialize_boxed(&self) -> (ConstructorNumber, &dyn BareSerialize) {
        let b: &'static Bool = (*self).into();
        Bool::serialize_boxed(b)
    }
}

impl BareDeserialize for String {
    fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
        let vec = de.read_bare::<Vec<u8>>()?;
        Ok(String::from_utf8(vec)?)
    }
}

impl BareSerialize for String {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
        ser.write_bare::<[u8]>(self.as_bytes())?;
        Ok(())
    }
}

impl<T> BoxedDeserialize for Box<T>
where
    T: BoxedDeserialize,
{
    fn possible_constructors() -> Vec<ConstructorNumber> {
        T::possible_constructors()
    }

    fn deserialize_boxed(id: ConstructorNumber, de: &mut Deserializer) -> Result<Self> {
        Ok(Box::new(T::deserialize_boxed(id, de)?))
    }
}

impl<T> BoxedSerialize for Box<T>
where
    T: BoxedSerialize,
{
    fn serialize_boxed(&self) -> (ConstructorNumber, &dyn BareSerialize) {
        T::serialize_boxed(self)
    }
}

/// Vector serialization
pub trait Vectored<T> {
    fn deserialize_with(
        de: &mut Deserializer,
        op: fn(&mut Deserializer) -> Result<T>,
    ) -> Result<Vec<T>>
    where
        Self: Sized,
    {
        let count = de.read_i32::<LittleEndian>()?;
        let mut ret = Vec::new();
        ret.try_reserve_exact(count as usize)
            .map_err(|e| error!("count {} is too big for {}: {}", count, type_name::<Self>(), e))?;
        for _ in 0..count {
            ret.push(op(de)?)
        }
        Ok(ret)
    }

    fn serialize_with(
        &self,
        ser: &mut Serializer,
        op: fn(&mut Serializer, &T) -> Result<()>,
    ) -> Result<()> {
        let vec = self.vector();
        ser.write_i32::<LittleEndian>(vec.len() as i32)?;
        for item in vec.iter() {
            op(ser, item)?
        }
        Ok(())
    }

    fn vector(&self) -> &Vec<T>;
}

/// Vector of bare objects
pub trait VectoredBare<T: BareDeserialize + BareSerialize> {
    fn deserialize(de: &mut Deserializer) -> Result<Vec<T>>
    where
        Self: Sized;
    fn serialize(&self, ser: &mut Serializer) -> Result<()>;
}

/// Vector of boxed objects
pub trait VectoredBoxed<T: BoxedDeserialize + BoxedSerialize> {
    fn deserialize(de: &mut Deserializer) -> Result<Vec<T>>
    where
        Self: Sized;
    fn serialize(&self, ser: &mut Serializer) -> Result<()>;
}

/// Vector of generic objects
pub type vector<T> = Vec<T>;

impl<T> Vectored<T> for Vec<T> {
    fn vector(&self) -> &Vec<T> {
        self
    }
}

impl<T: BareDeserialize + BareSerialize> VectoredBare<T> for Vec<T> {
    fn deserialize(de: &mut Deserializer) -> Result<Vec<T>> {
        <Vec<T> as Vectored<T>>::deserialize_with(de, |de| de.read_bare())
    }

    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        (self as &dyn Vectored<T>).serialize_with(ser, |ser, item| ser.write_bare(item))
    }
}

impl<T: BoxedDeserialize + BoxedSerialize> VectoredBoxed<T> for Vec<T> {
    fn deserialize(de: &mut Deserializer) -> Result<Vec<T>> {
        <Vec<T> as Vectored<T>>::deserialize_with(de, |de| de.read_boxed())
    }

    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        (self as &dyn Vectored<T>).serialize_with(ser, |ser, item| ser.write_boxed(item))
    }
}

// Seems to be unused
// const VECTOR_CONSTRUCTOR: ConstructorNumber = ConstructorNumber(0x1cb5c415);
//
// impl<T> BoxedSerialize for Vec<T>
// where Self: BareSerialize
// {
// fn serialize_boxed(&self) -> (ConstructorNumber, &dyn BareSerialize) {
// (VECTOR_CONSTRUCTOR, self)
// }
// }
//
// impl<T> BoxedDeserialize for Vec<T>
// where Self: BareDeserialize
// {
// fn possible_constructors() -> Vec<ConstructorNumber> {
// vec![VECTOR_CONSTRUCTOR]
// }
// fn deserialize_boxed(id: ConstructorNumber, de: &mut Deserializer) ->
// Result<Self> { assert_eq!(id, VECTOR_CONSTRUCTOR);
// <Vec<T> as BareDeserialize>::deserialize_bare(de)
// }
// }

/// Vector of bytes
pub type bytes = Vec<u8>;

impl BareDeserialize for bytes {
    fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
        let len = de.read_u8()?;
        let (len, mut have_read) = if len != 254 {
            (len as usize, 1)
        } else {
            (de.read_u24::<LittleEndian>()? as usize, 4)
        };

        let mut buf = Vec::new();
        buf.try_reserve_exact(len)
            .map_err(|e| error!("count {} is too big for {}: {}", len, type_name::<Self>(), e))?;
        buf.resize(len, 0);
        de.read_exact(&mut buf)?;
        have_read += len;
        let remainder = have_read % 4;
        if remainder != 0 {
            let mut buf = [0u8; 4];
            de.read_exact(&mut buf[remainder..])?;
        }
        Ok(buf)
    }
}

impl BareSerialize for bytes {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
        ser.write_bare::<[u8]>(self)
    }
}

impl BareSerialize for [u8] {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
        let len = self.len();
        let mut have_written = if len < 254 {
            ser.write_u8(len as u8)?;
            1
        } else {
            ser.write_u8(254)?;
            ser.write_u24::<LittleEndian>(len as u32)?;
            4
        };

        ser.write_all(self)?;
        have_written += len;
        let remainder = have_written % 4;
        if remainder != 0 {
            let buf = [0u8; 4];
            ser.write_all(&buf[remainder..])?;
        }
        Ok(())
    }
}

macro_rules! impl_tl_primitive {
    ($tltype:ident, $ptype:ty, $read:ident, $write:ident) => {
        pub type $tltype = $ptype;

        impl BareDeserialize for $ptype {
            fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
                Ok(de.$read::<LittleEndian>()?)
            }
        }

        impl BareSerialize for $ptype {
            fn constructor(&self) -> crate::ConstructorNumber {
                unreachable!()
            }

            fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
                ser.$write::<LittleEndian>(*self)?;
                Ok(())
            }
        }
    };
}

macro_rules! impl_tl_primitive_byte {
    ($tltype:ident, $ptype:ty, $read:ident, $write:ident) => {
        pub type $tltype = $ptype;

        impl BareDeserialize for $ptype {
            fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
                Ok(de.$read()?)
            }
        }

        impl BareSerialize for $ptype {
            fn constructor(&self) -> crate::ConstructorNumber {
                unreachable!()
            }

            fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
                ser.$write(*self)?;
                Ok(())
            }
        }
    };
}

impl_tl_primitive_byte! { byte, u8, read_u8, write_u8 }
impl_tl_primitive! { int, i32, read_i32, write_i32 }
impl_tl_primitive! { uint, u32, read_u32, write_u32 }
impl_tl_primitive! { long, i64, read_i64, write_i64 }
impl_tl_primitive! { ulong, u64, read_u64, write_u64 }

pub type double = OrderedFloat<f64>;

impl BareDeserialize for double {
    fn deserialize_bare(de: &mut Deserializer) -> Result<Self> {
        Ok(de.read_f64::<LittleEndian>()?.into())
    }
}

impl BareSerialize for double {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, ser: &mut Serializer) -> Result<()> {
        ser.write_f64::<LittleEndian>(self.0)?;
        Ok(())
    }
}

// Built-in types:
// pub type int8 = i8;
pub type int32 = i32;
pub type int53 = i64;
pub type int64 = i64;
pub type uint8 = u8;
pub type uint32 = u32;
pub type uint64 = u64;

/// Flags built-in type.
pub type Flags = u32;
pub type lengthPrefixedTypedObject = LengthPrefixed<TypedObject>;
pub type True = bool;
/// String built-in type.
pub type string = String;
/// Alias of TLObject built-in type.
pub type TypedObject = TLObject;
/// Alias of TLObject built-in type.
pub type Object = TLObject;
/// Function. Alias of TLObject built-in type.
pub type Function = TLObject;
/// Alias of SecureBytes built-in type.
pub type secureBytes = crate::secure::SecureBytes;
/// Alias of SecureString built-in type.
pub type secureString = crate::secure::SecureString;
