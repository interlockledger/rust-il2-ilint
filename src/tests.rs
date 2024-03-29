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
use super::*;

pub struct SampleILInt {
    pub value: u64,
    pub encoded_size: usize,
    pub encoded: [u8; 10],
}

const FILLER: u8 = 0xA5;

const SAMPLE_VALUES: [SampleILInt; 10] = [
    SampleILInt {
        value: 0xF7,
        encoded_size: 1,
        encoded: [
            0xF7, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0xF8,
        encoded_size: 2,
        encoded: [
            0xF8, 0x00, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x021B,
        encoded_size: 3,
        encoded: [
            0xF9, 0x01, 0x23, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x01_243D,
        encoded_size: 4,
        encoded: [
            0xFA, 0x01, 0x23, 0x45, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x0123_465F,
        encoded_size: 5,
        encoded: [
            0xFB, 0x01, 0x23, 0x45, 0x67, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x01_2345_6881,
        encoded_size: 6,
        encoded: [
            0xFC, 0x01, 0x23, 0x45, 0x67, 0x89, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x0123_4567_8AA3,
        encoded_size: 7,
        encoded: [
            0xFD, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x1_2345_6789_ACC5,
        encoded_size: 8,
        encoded: [
            0xFE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x123_4567_89AB_CEE7,
        encoded_size: 9,
        encoded: [0xFF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, FILLER],
    },
    SampleILInt {
        value: 0xFFFF_FFFF_FFFF_FFFF,
        encoded_size: 9,
        encoded: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07, FILLER],
    },
];

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
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFF_FFFF), 4);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100_0000), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFF_FFFF), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1_0000_0000), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFF_FFFF_FFFF), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100_0000_0000), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFF_FFFF_FFFF), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1_0000_0000_0000), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFF_FFFF_FFFF_FFFF), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100_0000_0000_0000), 9);
    assert_eq!(encoded_size(0xFFFF_FFFF_FFFF_FFFF), 9);
}

#[test]
fn test_encode() {
    for i in 0..0xF8 {
        let mut buff: [u8; 1] = [0];

        match encode(i as u64, &mut buff) {
            Ok(v) => assert_eq!(v, 1),
            _ => panic!("Success expected!"),
        }
        assert_eq!(buff[0], i as u8);
        match encode(i as u64, &mut buff[0..0]) {
            Err(ErrorKind::InsufficientBuffer) => (),
            _ => panic!("Expected Err(ErrorKind::InsufficientBuffer)"),
        }
    }

    for sample in &SAMPLE_VALUES {
        let mut buff: [u8; 10] = [FILLER; 10];
        let enc_size = sample.encoded_size;
        match encode(sample.value, &mut buff[0..enc_size]) {
            Ok(v) => assert_eq!(v, enc_size),
            _ => panic!("Success expected!"),
        }
        assert_eq!(buff, sample.encoded);
    }
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
fn test_decode() {
    // All with 1 byte
    for i in 0..0xF8 {
        let mut buff: [u8; 1] = [0];
        buff[0] = i as u8;

        match decode(&buff) {
            Ok((v, s)) => {
                assert_eq!(v, i as u64);
                assert_eq!(s, 1);
            }
            _ => panic!(),
        }
    }

    // From samples
    for sample in &SAMPLE_VALUES {
        let enc_size = sample.encoded_size;
        match decode(&sample.encoded[0..enc_size]) {
            Ok((v, s)) => {
                assert_eq!(v, sample.value);
                assert_eq!(s, sample.encoded_size);
            }
            _ => panic!(),
        }
    }

    // Corrupted due to size
    let mut encoded: [u8; 9] = [0; 9];
    for size in 2..10 {
        encoded[0] = ILINT_BASE + (size - 2) as u8;
        for bad_size in 0..size {
            match decode(&encoded[0..bad_size]) {
                Err(ErrorKind::InsufficientBuffer) => {}
                _ => panic!("Corrupted data expected."),
            }
        }
    }

    // Overflow!
    let mut encoded: [u8; 9] = [0xFF; 9];
    for last in 0x08..0x100 {
        encoded[8] = last as u8;
        match decode(&encoded[0..9]) {
            Err(ErrorKind::Overflow) => {}
            _ => panic!("Overflow expected."),
        }
    }
}

#[test]
fn test_encode_sign() {
    // Unsigned
    assert_eq!(encode_sign(0), 0);
    assert_eq!(encode_sign(1), 2);
    assert_eq!(encode_sign(0x7FFF_FFFF_FFFF_FFFF), 0xFFFF_FFFF_FFFF_FFFE);
    // Signed
    assert_eq!(encode_sign(-1), 1);
    assert_eq!(encode_sign(-2), 3);
    assert_eq!(
        encode_sign(-9_223_372_036_854_775_808),
        0xFFFF_FFFF_FFFF_FFFF
    );
}

#[test]
fn test_decode_sign() {
    // Unsigned
    assert_eq!(decode_sign(0), 0);
    assert_eq!(decode_sign(2), 1);
    assert_eq!(decode_sign(0xFFFF_FFFF_FFFF_FFFE), 0x7FFF_FFFF_FFFF_FFFF);
    // Signed
    assert_eq!(decode_sign(1), -1);
    assert_eq!(decode_sign(3), -2);
    assert_eq!(
        decode_sign(0xFFFF_FFFF_FFFF_FFFF),
        -9_223_372_036_854_775_808
    );
}
