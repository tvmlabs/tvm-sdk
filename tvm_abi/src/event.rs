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

    // Regression: an event whose last field spills into a continuation cell near
    // the 1023-bit boundary must decode. Real ext-out `OrderBook.OrderPlaced` body
    // captured from a live shellnet OrderBook (event-id 0x1b9c6957): its 9 fields
    // are 1033 (id+data) bits, so `opNonce` correctly spills to a ref
    // (cell0 = 969 bits + ref, ref = 64 bits). Before seeding `Cursor.used_bits`
    // with `slice.pos()`, `check_layout` computed 937 + 64 = 1001 <= 1023 and
    // rejected this legit spill with `WrongDataLayout` (ton-client error 304).
    #[test]
    fn decodes_multicell_event_with_last_field_spilled_to_ref() {
        use tvm_types::read_single_root_boc;

        use crate::Token;
        use crate::TokenValue;
        use crate::Uint;

        #[rustfmt::skip]
        const BODY: &[u8] = &[
            0xb5, 0xee, 0x9c, 0x72, 0x01, 0x01, 0x02, 0x01, 0x00, 0x87, 0x00, 0x01, 0xf3, 0x1b, 0x9c,
            0x69, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0xc4, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x7e, 0x11, 0xd6, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x35, 0x16, 0x00, 0xf3, 0x80, 0x00, 0x07, 0xd0, 0xe5, 0xaf, 0xc3,
            0xc4, 0x11, 0x7e, 0x4a, 0x6a, 0xdd, 0x3b, 0x9a, 0x17, 0xc8, 0xb3, 0x3e, 0x8c, 0xa1, 0x60,
            0xf7, 0x2b, 0x6c, 0x50, 0x68, 0x47, 0x96, 0x4d, 0xaa, 0x69, 0xce, 0xb0, 0x0a, 0x0b, 0xc0,
            0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];

        let cell = read_single_root_boc(BODY).unwrap();
        let slice = SliceData::load_cell(cell).unwrap();

        let event = Event {
            abi_version: ABI_VERSION_2_4,
            name: "OrderPlaced".into(),
            inputs: vec![
                Param::new("orderId", ParamType::Uint(128)),
                Param::new("outcomeId", ParamType::Uint(32)),
                Param::new("isBuy", ParamType::Bool),
                Param::new("flags", ParamType::Uint(8)),
                Param::new("price", ParamType::Uint(256)),
                Param::new("amount", ParamType::Uint(128)),
                Param::new("clientOrderId", ParamType::Uint(128)),
                Param::new("depositHash", ParamType::Uint(256)),
                Param::new("opNonce", ParamType::Uint(64)),
            ],
            id: 0x1b9c_6957,
        };

        let tokens: Vec<Token> =
            event.decode_input(slice, false).expect("multi-cell event must decode");

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].value, TokenValue::Uint(Uint::new(1, 128))); // orderId
        assert_eq!(tokens[2].value, TokenValue::Bool(false)); // isBuy
        assert_eq!(tokens[3].value, TokenValue::Uint(Uint::new(0, 8))); // flags
        // opNonce — the field that spilled into the continuation cell:
        assert_eq!(tokens[8].name, "opNonce");
        assert_eq!(tokens[8].value, TokenValue::Uint(Uint::new(1, 64)));
    }
}
