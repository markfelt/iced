/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::*;
use core::u32;

const BITNESS: u32 = 32;
const ORIG_RIP: u64 = 0x8000;
const NEW_RIP: u64 = 0x8000_0000;

#[test]
fn xbegin_fwd_rel16() {
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let original_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0x66, 0xC7, 0xF8, 0x08, 0x00,// xbegin 0000800Fh
		/*0007*/ 0xB0, 0x01,// mov al,1
		/*0009*/ 0xC7, 0xF8, 0x02, 0x00, 0x00, 0x00,// xbegin 00008011h
		/*000F*/ 0xB0, 0x02,// mov al,2
		/*0011*/ 0xB0, 0x03,// mov al,3
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let new_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0x66, 0xC7, 0xF8, 0x07, 0x00,// xbegin 8000000Eh
		/*0007*/ 0xB0, 0x01,// mov al,1
		/*0009*/ 0x66, 0xC7, 0xF8, 0x02, 0x00,// xbegin 80000010h
		/*000E*/ 0xB0, 0x02,// mov al,2
		/*0010*/ 0xB0, 0x03,// mov al,3
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let expected_instruction_offsets = [
		0x0000,
		0x0002,
		0x0007,
		0x0009,
		0x000E,
		0x0010,
	];
	let expected_reloc_infos = [];
	const OPTIONS: u32 = BlockEncoderOptions::NONE;
	encode_test(
		BITNESS,
		ORIG_RIP,
		&original_data,
		NEW_RIP,
		&new_data,
		OPTIONS,
		DECODER_OPTIONS,
		&expected_instruction_offsets,
		&expected_reloc_infos,
	);
}

#[test]
fn xbegin_bwd_rel16() {
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let original_data = [
		/*0000*/ 0xB0, 0x02,// mov al,2
		/*0002*/ 0xB0, 0x03,// mov al,3
		/*0004*/ 0xB0, 0x00,// mov al,0
		/*0006*/ 0x66, 0xC7, 0xF8, 0xF5, 0xFF,// xbegin 00008000h
		/*000B*/ 0xB0, 0x01,// mov al,1
		/*000D*/ 0xC7, 0xF8, 0xEF, 0xFF, 0xFF, 0xFF,// xbegin 00008002h
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let new_data = [
		/*0000*/ 0xB0, 0x02,// mov al,2
		/*0002*/ 0xB0, 0x03,// mov al,3
		/*0004*/ 0xB0, 0x00,// mov al,0
		/*0006*/ 0x66, 0xC7, 0xF8, 0xF5, 0xFF,// xbegin 80000000h
		/*000B*/ 0xB0, 0x01,// mov al,1
		/*000D*/ 0x66, 0xC7, 0xF8, 0xF0, 0xFF,// xbegin 80000002h
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let expected_instruction_offsets = [
		0x0000,
		0x0002,
		0x0004,
		0x0006,
		0x000B,
		0x000D,
	];
	let expected_reloc_infos = [];
	const OPTIONS: u32 = BlockEncoderOptions::NONE;
	encode_test(
		BITNESS,
		ORIG_RIP,
		&original_data,
		NEW_RIP,
		&new_data,
		OPTIONS,
		DECODER_OPTIONS,
		&expected_instruction_offsets,
		&expected_reloc_infos,
	);
}

#[test]
fn xbegin_fwd_rel32() {
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let original_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0x66, 0xC7, 0xF8, 0x11, 0x00,// xbegin 80000018h
		/*0007*/ 0xB0, 0x01,// mov al,1
		/*0009*/ 0xC7, 0xF8, 0x09, 0x00, 0x00, 0x00,// xbegin 80000018h
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let new_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0xC7, 0xF8, 0x10, 0x00, 0x01, 0x00,// xbegin 80000018h
		/*0008*/ 0xB0, 0x01,// mov al,1
		/*000A*/ 0xC7, 0xF8, 0x08, 0x00, 0x01, 0x00,// xbegin 80000018h
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let expected_instruction_offsets = [
		0x0000,
		0x0002,
		0x0008,
		0x000A,
	];
	let expected_reloc_infos = [];
	const OPTIONS: u32 = BlockEncoderOptions::NONE;
	const ORIG_RIP: u64 = 0x8000_0000;
	encode_test(
		BITNESS,
		ORIG_RIP,
		&original_data,
		ORIG_RIP - 0x0001_0000,
		&new_data,
		OPTIONS,
		DECODER_OPTIONS,
		&expected_instruction_offsets,
		&expected_reloc_infos,
	);
}

#[test]
fn xbegin_bwd_rel32() {
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let original_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0x66, 0xC7, 0xF8, 0xF8, 0xFF,// xbegin 7FFFFFFFh
		/*0007*/ 0xB0, 0x01,// mov al,1
		/*0009*/ 0xC7, 0xF8, 0xF0, 0xFF, 0xFF, 0xFF,// xbegin 7FFFFFFFh
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let new_data = [
		/*0000*/ 0xB0, 0x00,// mov al,0
		/*0002*/ 0xC7, 0xF8, 0xF7, 0xFF, 0xFE, 0xFF,// xbegin 7FFFFFFFh
		/*0008*/ 0xB0, 0x01,// mov al,1
		/*000A*/ 0xC7, 0xF8, 0xEF, 0xFF, 0xFE, 0xFF,// xbegin 7FFFFFFFh
	];
	#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
	let expected_instruction_offsets = [
		0x0000,
		0x0002,
		0x0008,
		0x000A,
	];
	let expected_reloc_infos = [];
	const OPTIONS: u32 = BlockEncoderOptions::NONE;
	const ORIG_RIP: u64 = 0x8000_0000;
	encode_test(
		BITNESS,
		ORIG_RIP,
		&original_data,
		ORIG_RIP + 0x0001_0000,
		&new_data,
		OPTIONS,
		DECODER_OPTIONS,
		&expected_instruction_offsets,
		&expected_reloc_infos,
	);
}
