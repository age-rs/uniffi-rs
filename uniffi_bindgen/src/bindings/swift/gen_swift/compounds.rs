/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::CodeType;
use crate::{
    bail,
    interface::{DefaultValue, Literal, Type},
    Result,
};

#[derive(Debug)]
pub struct OptionalCodeType {
    inner: Type,
}

impl OptionalCodeType {
    pub fn new(inner: Type) -> Self {
        Self { inner }
    }
}

impl CodeType for OptionalCodeType {
    fn type_label(&self) -> String {
        format!("{}?", super::SwiftCodeOracle.find(&self.inner).type_label())
    }

    fn canonical_name(&self) -> String {
        format!(
            "Option{}",
            super::SwiftCodeOracle.find(&self.inner).canonical_name()
        )
    }

    fn default(&self, default: &DefaultValue) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::None) => Ok("nil".into()),
            DefaultValue::Literal(Literal::Some { inner }) => {
                super::SwiftCodeOracle.find(&self.inner).default(inner)
            }
            _ => bail!("Invalid literal for Optional type: {default:?}"),
        }
    }
}

#[derive(Debug)]
pub struct SequenceCodeType {
    inner: Type,
}

impl SequenceCodeType {
    pub fn new(inner: Type) -> Self {
        Self { inner }
    }
}

impl CodeType for SequenceCodeType {
    fn type_label(&self) -> String {
        format!(
            "[{}]",
            super::SwiftCodeOracle.find(&self.inner).type_label()
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Sequence{}",
            super::SwiftCodeOracle.find(&self.inner).canonical_name()
        )
    }

    fn default(&self, default: &DefaultValue) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::EmptySequence) => {
                Ok("[]".into())
            }
            _ => bail!("Invalid literal for sequence type: {default:?}"),
        }
    }
}

#[derive(Debug)]
pub struct MapCodeType {
    key: Type,
    value: Type,
}

impl MapCodeType {
    pub fn new(key: Type, value: Type) -> Self {
        Self { key, value }
    }
}

impl CodeType for MapCodeType {
    fn type_label(&self) -> String {
        format!(
            "[{}: {}]",
            super::SwiftCodeOracle.find(&self.key).type_label(),
            super::SwiftCodeOracle.find(&self.value).type_label()
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Dictionary{}{}",
            super::SwiftCodeOracle.find(&self.key).canonical_name(),
            super::SwiftCodeOracle.find(&self.value).canonical_name()
        )
    }

    fn default(&self, default: &DefaultValue) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::EmptyMap) => Ok("[:]".into()),
            _ => bail!("Invalid literal for map type: {default:?}"),
        }
    }
}
