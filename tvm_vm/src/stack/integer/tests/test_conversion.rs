// Copyright (C) 2019-2024 TON. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_block::types::ExceptionCode;

use crate::stack::integer::IntegerData;

#[test]
fn test_into() {
    let one = IntegerData::one();
    let nan = IntegerData::nan();

    assert_eq!(IntegerData::into(&one, 0..=1).unwrap(), 1i8);
    assert_eq!(
        crate::error::tvm_exception_code(&IntegerData::into(&one, 0..=0).unwrap_err()),
        Some(ExceptionCode::RangeCheckError)
    );
    assert_eq!(
        crate::error::tvm_exception_code(&IntegerData::into(&one, 2..=2).unwrap_err()),
        Some(ExceptionCode::RangeCheckError)
    );
    assert_eq!(
        crate::error::tvm_exception_code(&IntegerData::into(&nan, 0..=0).unwrap_err()),
        Some(ExceptionCode::RangeCheckError)
    );
}
