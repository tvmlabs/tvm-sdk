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

use crate::stack::integer::behavior::OperationBehavior;
use crate::stack::integer::utils::binary_op;
use crate::stack::integer::utils::construct_single_nan;
use crate::stack::integer::utils::process_single_result;
use crate::stack::integer::utils::unary_op;
use crate::stack::integer::IntegerData;

impl IntegerData {
    pub fn and<T>(&self, other: &IntegerData) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        binary_op::<T, _, _, _, _, _>(
            self,
            other,
            |x, y| x & y,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }

    pub fn or<T>(&self, other: &IntegerData) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        binary_op::<T, _, _, _, _, _>(
            self,
            other,
            |x, y| x | y,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }

    pub fn xor<T>(&self, other: &IntegerData) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        binary_op::<T, _, _, _, _, _>(
            self,
            other,
            |x, y| x ^ y,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }

    pub fn not<T>(&self) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        unary_op::<T, _, _, _, _, _>(
            self,
            |x| !x,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }

    pub fn shl<T>(&self, shift: usize) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        unary_op::<T, _, _, _, _, _>(
            self,
            |x| x << shift,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }

    pub fn shr<T>(&self, shift: usize) -> Result<IntegerData>
    where
        T: OperationBehavior,
    {
        unary_op::<T, _, _, _, _, _>(
            self,
            |x| x >> shift,
            construct_single_nan,
            process_single_result::<T, _>,
        )
    }
}
