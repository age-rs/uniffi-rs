/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::{AsCodeType, CodeType};
use crate::{
    bail,
    interface::{DefaultValue, Literal, Type},
    ComponentInterface, Result,
};

#[derive(Debug)]
pub struct OptionalCodeType {
    inner: Type,
}

impl OptionalCodeType {
    pub fn new(inner: Type) -> Self {
        Self { inner }
    }
    fn inner(&self) -> &Type {
        &self.inner
    }
}

impl CodeType for OptionalCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        format!(
            "{}?",
            super::KotlinCodeOracle.find(self.inner()).type_label(ci)
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Optional{}",
            super::KotlinCodeOracle.find(self.inner()).canonical_name()
        )
    }

    fn default(&self, default: &DefaultValue, ci: &ComponentInterface) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::None) => Ok("null".into()),
            DefaultValue::Literal(Literal::Some { inner }) => {
                super::KotlinCodeOracle.find(&self.inner).default(inner, ci)
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
    fn inner(&self) -> &Type {
        &self.inner
    }
}

impl CodeType for SequenceCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        format!(
            "List<{}>",
            super::KotlinCodeOracle.find(self.inner()).type_label(ci)
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Sequence{}",
            super::KotlinCodeOracle.find(self.inner()).canonical_name()
        )
    }

    fn default(&self, default: &DefaultValue, _ci: &ComponentInterface) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::EmptySequence) => {
                Ok("listOf()".into())
            }
            _ => bail!("Invalid literal for List type: {default:?}"),
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

    fn key(&self) -> &Type {
        &self.key
    }

    fn value(&self) -> &Type {
        &self.value
    }
}

impl CodeType for MapCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        format!(
            "Map<{}, {}>",
            super::KotlinCodeOracle.find(self.key()).type_label(ci),
            super::KotlinCodeOracle.find(self.value()).type_label(ci),
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Map{}{}",
            self.key().as_codetype().canonical_name(),
            self.value().as_codetype().canonical_name(),
        )
    }

    fn default(&self, default: &DefaultValue, _ci: &ComponentInterface) -> Result<String> {
        match default {
            DefaultValue::Default | DefaultValue::Literal(Literal::EmptyMap) => {
                Ok("mapOf()".into())
            }
            _ => bail!("Invalid literal for Map type: {default:?}"),
        }
    }
}
