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

using System;

namespace Generator.Enums.Encoder {
	[Enum("EncFlags3", Flags = true, NoInitialize = true)]
	enum EncFlags3 : uint {
		None					= 0,
		/// <summary><see cref="EncodingKind"/></summary>
		EncodingShift			= 0,
		EncodingMask			= 7,
		/// <summary><see cref="CodeSize"/></summary>
		OperandSizeShift		= 3,
		OperandSizeMask			= 3,
		/// <summary><see cref="CodeSize"/></summary>
		AddressSizeShift		= 5,
		AddressSizeMask			= 3,
		/// <summary><see cref="TupleType"/></summary>
		TupleTypeShift			= 7,
		TupleTypeMask			= 0xF,

		// FREE FREE FREE FREE
		// [11] = free
		// BITS BITS BITS BITS

		DefaultOpSize64			= 0x00001000,
		ForceOpSize64			= 0x00002000,
		IntelForceOpSize64		= 0x00004000,
		Fwait					= 0x00008000,
		Bit16or32				= 0x00010000,
		Bit64					= 0x00020000,
		Lock					= 0x00040000,
		Xacquire				= 0x00080000,
		Xrelease				= 0x00100000,
		Rep						= 0x00200000,
		Repne					= 0x00400000,
		Bnd						= 0x00800000,
		HintTaken				= 0x01000000,
		Notrack					= 0x02000000,
		Broadcast				= 0x04000000,
		RoundingControl			= 0x08000000,
		SuppressAllExceptions	= 0x10000000,
		OpMaskRegister			= 0x20000000,
		ZeroingMasking			= 0x40000000,
		RequireOpMaskRegister	= 0x80000000,
	}
}
