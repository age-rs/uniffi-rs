/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::CodeType;
use crate::{
    bail,
    interface::Literal,
    interface::{ComponentInterface, Radix, Type},
    Result,
};

fn render_literal(literal: &Literal, _ci: &ComponentInterface) -> Result<String> {
    fn typed_number(type_: &Type, num_str: String) -> Result<String> {
        let unwrapped_type = match type_ {
            Type::Optional { inner_type } => inner_type,
            t => t,
        };
        Ok(match unwrapped_type {
            // Bytes, Shorts and Ints can all be inferred from the type.
            Type::Int8 | Type::Int16 | Type::Int32 => num_str,
            Type::Int64 => format!("{num_str}L"),

            Type::UInt8 | Type::UInt16 | Type::UInt32 => format!("{num_str}u"),
            Type::UInt64 => format!("{num_str}uL"),

            Type::Float32 => format!("{num_str}f"),
            Type::Float64 => num_str,
            _ => bail!("Unexpected literal: {num_str} for type: {type_:?}"),
        })
    }

    match literal {
        Literal::Boolean(v) => Ok(format!("{v}")),
        Literal::String(s) => Ok(format!("\"{s}\"")),
        Literal::Int(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("{i:#x}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::UInt(i, radix, type_) => typed_number(
            type_,
            match radix {
                Radix::Octal => format!("{i:#x}"),
                Radix::Decimal => format!("{i}"),
                Radix::Hexadecimal => format!("{i:#x}"),
            },
        ),
        Literal::Float(string, type_) => typed_number(type_, string.clone()),

        _ => bail!("Invalid literal {literal:?}"),
    }
}

macro_rules! impl_code_type_for_primitive {
    ($T:ident, $class_name:literal) => {
        #[derive(Debug)]
        pub struct $T;

        impl CodeType for $T {
            fn type_label(&self, _ci: &ComponentInterface) -> String {
                format!("kotlin.{}", $class_name)
            }

            fn canonical_name(&self) -> String {
                $class_name.into()
            }

            fn literal(&self, literal: &Literal, ci: &ComponentInterface) -> Result<String> {
                render_literal(&literal, ci)
            }
        }
    };
}

impl_code_type_for_primitive!(BooleanCodeType, "Boolean");
impl_code_type_for_primitive!(StringCodeType, "String");
impl_code_type_for_primitive!(BytesCodeType, "ByteArray");
impl_code_type_for_primitive!(Int8CodeType, "Byte");
impl_code_type_for_primitive!(Int16CodeType, "Short");
impl_code_type_for_primitive!(Int32CodeType, "Int");
impl_code_type_for_primitive!(Int64CodeType, "Long");
impl_code_type_for_primitive!(UInt8CodeType, "UByte");
impl_code_type_for_primitive!(UInt16CodeType, "UShort");
impl_code_type_for_primitive!(UInt32CodeType, "UInt");
impl_code_type_for_primitive!(UInt64CodeType, "ULong");
impl_code_type_for_primitive!(Float32CodeType, "Float");
impl_code_type_for_primitive!(Float64CodeType, "Double");
