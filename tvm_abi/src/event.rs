// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_types::Result;
use tvm_types::SliceData;

use crate::Function;
use crate::Param;
use crate::Token;
use crate::TokenValue;
use crate::contract::AbiVersion;
use crate::contract::SerdeEvent;
use crate::error::AbiError;

/// Contract event specification.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// ABI version
    pub abi_version: AbiVersion,
    /// Event name.
    pub name: String,
    /// Event input.
    pub inputs: Vec<Param>,
    /// Event ID
    pub id: u32,
}

impl Event {
    /// Creates `Function` struct from parsed JSON struct `SerdeFunction`
    pub(crate) fn from_serde(abi_version: AbiVersion, serde_event: SerdeEvent) -> Self {
        let mut event =
            Event { abi_version, name: serde_event.name, inputs: serde_event.inputs, id: 0 };
        event.id =
            if let Some(id) = serde_event.id { id } else { event.get_function_id() & 0x7FFFFFFF };
        event
    }

    /// Returns all input params of given function.
    pub fn input_params(&self) -> Vec<Param> {
        self.inputs.to_vec()
    }

    /// Returns true if function has input parameters, false in not
    pub fn has_input(&self) -> bool {
        !self.inputs.is_empty()
    }

    /// Retruns ABI function signature
    pub fn get_function_signature(&self) -> String {
        let input_types = self
            .inputs
            .iter()
            .map(|param| param.kind.type_signature())
            .collect::<Vec<String>>()
            .join(",");

        format!("{}({})v{}", self.name, input_types, self.abi_version.major)
    }

    /// Computes function ID for contract function
    pub fn get_function_id(&self) -> u32 {
        let signature = self.get_function_signature();

        Function::calc_function_id(&signature)
    }

    /// Returns ID for event emitting message
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Parses the ABI function call to list of tokens.
    pub fn decode_input(&self, mut data: SliceData, allow_partial: bool) -> Result<Vec<Token>> {
        let id = data.get_next_u32()?;

        if id != self.get_id() {
            Err(AbiError::WrongId { id })?
        }

        TokenValue::decode_params(&self.input_params(), data, &self.abi_version, allow_partial)
    }

    /// Decodes function id from contract answer
    pub fn decode_id(mut data: SliceData) -> Result<u32> {
        data.get_next_u32()
    }

    /// Check if message body is related to this event
    pub fn is_my_message(&self, data: SliceData, _internal: bool) -> Result<bool> {
        let decoded_id = Self::decode_id(data)?;
        Ok(self.get_id() == decoded_id)
    }
}

#[cfg(test)]
mod tests {
    use tvm_types::BuilderData;
    use tvm_types::IBitstring;
    use tvm_types::SliceData;

    use super::Event;
    use crate::Param;
    use crate::ParamType;
    use crate::contract::ABI_VERSION_2_4;
    use crate::contract::SerdeEvent;

    fn slice_with_u32(value: u32) -> SliceData {
        let mut builder = BuilderData::new();
        builder.append_u32(value).unwrap();
        SliceData::load_cell(builder.into_cell().unwrap()).unwrap()
    }

    #[test]
    fn from_serde_populates_helpers_and_computed_id() {
        let inputs = vec![Param::new("value", ParamType::Uint(32))];
        let event = Event::from_serde(
            ABI_VERSION_2_4,
            SerdeEvent { name: "ValueChanged".into(), inputs: inputs.clone(), id: None },
        );

        assert!(event.has_input());
        assert_eq!(event.input_params(), inputs);
        assert_eq!(event.get_function_signature(), "ValueChanged(uint32)v2");
        assert_eq!(event.get_id(), event.get_function_id() & 0x7FFFFFFF);
    }

    #[test]
    fn decode_input_and_message_matching_cover_id_paths() {
        let event = Event {
            abi_version: ABI_VERSION_2_4,
            name: "Ping".into(),
            inputs: vec![],
            id: 0x1020_3040,
        };

        let decoded = event.decode_input(slice_with_u32(event.id), false).unwrap();
        assert!(decoded.is_empty());
        assert_eq!(Event::decode_id(slice_with_u32(event.id)).unwrap(), event.id);
        assert!(event.is_my_message(slice_with_u32(event.id), false).unwrap());
        assert!(!event.is_my_message(slice_with_u32(event.id + 1), false).unwrap());
    }

    #[test]
    fn decode_input_rejects_wrong_id() {
        let event =
            Event { abi_version: ABI_VERSION_2_4, name: "Ping".into(), inputs: vec![], id: 7 };
        let err = event.decode_input(slice_with_u32(8), false).unwrap_err();
        assert!(err.to_string().contains("Wrong function ID"));
    }
}
