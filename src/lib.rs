/*
 * BSD 3-Clause License
 *
 * Copyright (c) 2020, InterlockLedger Network
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright notice, this
 *   list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 *
 * * Neither the name of the copyright holder nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
//! This crate provides a **no_std** implementation of the **InterlockLedger ILInt**
//! format. This format allows the encoding of 64 bit integer values in a compact
//! format that uses 1 to 9 bytes of space depending on the actual value. See
//! [ILInt Specification](https://github.com/interlockledger/specification/tree/master/ILInt)
//! for further details about the format.
#![no_std]

#[cfg(test)]
mod tests;

/// LInt base value. All values smaller than this value are encoded as
/// a single byte.
pub const ILINT_BASE: u8 = 0xF8;

/// Value of ILINT_BASE as U64.
pub const ILINT_BASE_U64: u64 = ILINT_BASE as u64;

/// Types of errors generated by this library.
pub enum ErrorKind {
    /// The provided buffer is too small.
    InsufficientBuffer,
    /// The encoded value is larger than 26^4 - 1 (18446744073709551615).
    Overflow,
}

/// Alias to the Result used by this library.
pub type Result<T> = core::result::Result<T, ErrorKind>;

/// Returns the size of the given value encoded as an ILInt.
///
/// Arguments:
///
/// * `value` : The value to be encoded;
///
/// Returns:
///
/// * The number of bytes required to encode the value;
///
pub fn encoded_size(value: u64) -> usize {
    match value {
        value if value < ILINT_BASE_U64 => 1,
        value if value <= (0xFF + ILINT_BASE_U64) => 2,
        value if value <= (0xFFFF + ILINT_BASE_U64) => 3,
        value if value <= (0x00FF_FFFF + ILINT_BASE_U64) => 4,
        value if value <= (0xFFFF_FFFF + ILINT_BASE_U64) => 5,
        value if value <= (0x00FF_FFFF_FFFF + ILINT_BASE_U64) => 6,
        value if value <= (0xFFFF_FFFF_FFFF + ILINT_BASE_U64) => 7,
        value if value <= (0x00FF_FFFF_FFFF_FFFF + ILINT_BASE_U64) => 8,
        _ => 9,
    }
}

/// Encodes the given value into a ILInt value.
///
/// Arguments:
///
/// * `value`: The value to be encoded;
/// * `enc`: The slice that will receive the encoded value.
/// It must have at least encoded_size(value) bytes;
///
/// Returns:
///
/// * `Ok(size)`: The number of bytes used.
/// * `Err(ErrorKind::InsufficientBuffer)`: If the buffer is too small
/// to hold the encoded value.
///
pub fn encode(value: u64, enc: &mut [u8]) -> Result<usize> {
    let size = encoded_size(value);
    if size > enc.len() {
        Err(ErrorKind::InsufficientBuffer)
    } else {
        if size == 1 {
            enc[0] = value as u8
        } else {
            enc[0] = (ILINT_BASE + ((size - 2) as u8)) as u8;
            let v = value - ILINT_BASE_U64;
            let mut shift = 8 * (size - 1);
            for i in enc.iter_mut().take(size).skip(1) {
                shift -= 8;
                *i = ((v >> shift) & 0xFF) as u8;
            }
        }
        Ok(size)
    }
}

/// Determines the size of the ILInt based on its header (the
/// first byte of the encoded value).
///
/// Arguments:
///
/// * `header`: The header of the ILInt. It is always the first byte of
///   the ILInt value;
///
/// Returns:
///
/// * The size of the ILInt in bytes, including the header.
///
pub fn decoded_size(header: u8) -> usize {
    if header < ILINT_BASE {
        1
    } else {
        (header - ILINT_BASE + 2) as usize
    }
}

/// Decodes an **ILInt** value.
///
/// Arguments:
///
/// * `value`: The **ILInt** value;
///
/// Returns:
///
/// * Ok(value,size): The decoded value and the number of bytes used.
/// * `Err(ErrorKind::InsufficientBuffer)`: If the buffer is too small
/// to hold the encoded value.
/// * `Err(ErrorKind::Overflow)`: If the encoded value is larger than
/// the maximum allowed value.
///
pub fn decode(value: &[u8]) -> Result<(u64, usize)> {
    if value.is_empty() {
        return Err(ErrorKind::InsufficientBuffer);
    }

    let size = decoded_size(value[0]);
    if size > value.len() {
        return Err(ErrorKind::InsufficientBuffer);
    }
    if size == 1 {
        Ok((value[0] as u64, 1))
    } else {
        let mut v: u64 = 0;
        for i in value.iter().take(size).skip(1) {
            v = (v << 8) + (*i as u64);
        }
        if v > 0xFFFF_FFFF_FFFF_FF07 {
            Err(ErrorKind::Overflow)
        } else {
            Ok((v + ILINT_BASE_U64, size))
        }
    }
}

/// Encodes a signed value into an unsiged value suitable
/// to be encoded as **ILInt**.
///
/// Arguments:
/// - `v`: The value to be encoded;
///
/// Returns the signed value ready to be encoded as an **ILInt**.
///
/// New since 2021-08-11.
pub fn encode_sign(v: i64) -> u64 {
    let tmp = v as u64;

    if tmp & 0x8000_0000_0000_0000 == 0 {
        tmp << 1
    } else {
        !(tmp << 1)
    }
}

/// Decodes an unsigned value into a siged value.
///
/// Arguments:
/// - `v`: The value to be decoded;
///
/// Returns the decoded signed value.
///
/// New since 2021-08-11.
pub fn decode_sign(v: u64) -> i64 {
    if v & 0x1 == 0 {
        (v >> 1) as i64
    } else {
        (!(v >> 1)) as i64
    }
}
