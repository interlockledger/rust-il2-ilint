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
#[cfg(test)]

use super::*;

#[test]
fn test_constants() {
    assert_eq!(ILINT_BASE, 0xF8);
}

#[test]
fn test_encoded_size() {
    assert_eq!(encoded_size(0), 1);
    assert_eq!(encoded_size(ILINT_BASE_U64 - 1), 1);
    assert_eq!(encoded_size(ILINT_BASE_U64), 2);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFF), 2);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100), 3);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFF), 3);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x10000), 4);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFF), 4);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1000000), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFF), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100000000), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFF), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x10000000000), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFFFF), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1000000000000), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFFFFFF), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100000000000000), 9);
    assert_eq!(encoded_size(0xFFFFFFFFFFFFFFFF), 9);
}

#[test]
fn test_encode() -> Result<(), ()> {

    for i in 0..0xF8 {
        let mut buff: [u8; 1] = [0];

        match encode(i as u64, &mut buff) {
            Ok(v) => assert_eq!(v, 1),
            Err(()) => return Err(()),
        }
        assert_eq!(buff[0], i as u8);
        assert!(encode(i as u64, &mut buff[0..0]).is_err());
    }

    // TODO Complete this test!!!
    let mut buff: [u8; 2] = [0; 2];
    let exp: [u8; 2] = [0xF8, 0x00];
    match encode(0xF8, &mut buff) {
        Ok(v) => assert_eq!(v, 2),
        Err(()) => return Err(()),
    }
    assert_eq!(buff, exp);

    Ok(())
}

#[test]
fn test_decoded_size() {

    for i in 0..0xF8 {
        assert_eq!(decoded_size(i), 1);
    }
    assert_eq!(decoded_size(0xF8), 2);
    assert_eq!(decoded_size(0xF9), 3);
    assert_eq!(decoded_size(0xFA), 4);
    assert_eq!(decoded_size(0xFB), 5);
    assert_eq!(decoded_size(0xFC), 6);
    assert_eq!(decoded_size(0xFD), 7);
    assert_eq!(decoded_size(0xFE), 8);
    assert_eq!(decoded_size(0xFF), 9);
}

#[test]
fn test_decode() -> Result<(),()> {
    // TODO This test is missing...
    Err(())
}
