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

use crate::enum_utils::to_memory_size;
use crate::memory_size_info::MemorySizeInfo;
use pyo3::prelude::*;

#[pyclass(module = "_iced_x86_py")]
#[text_signature = "(/)"]
pub(crate) struct MemorySizeExt {}

/// :class:`MemorySize` enum extension methods, see also :class:`MemorySizeInfo`
#[pymethods]
impl MemorySizeExt {
	/// Gets the memory size info
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     :class:`MemorySizeInfo`: Memory size info
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     info = MemorySizeExt.info(MemorySize.PACKED256_UINT16);
	///     assert info.size == 32
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn info(memory_size: u32) -> PyResult<MemorySizeInfo> {
		Ok(MemorySizeInfo { info: to_memory_size(memory_size)?.info() })
	}

	/// Gets the size in bytes of the memory location or 0 if it's not accessed by the instruction or unknown or variable sized
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     int: (``u32``) Size in bytes of the memory location or
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert MemorySizeExt.size(MemorySize.UINT32) == 4
	///     assert MemorySizeExt.size(MemorySize.PACKED256_UINT16) == 32
	///     assert MemorySizeExt.size(MemorySize.BROADCAST512_UINT64) == 8
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn size(memory_size: u32) -> PyResult<u32> {
		Ok(to_memory_size(memory_size)?.size() as u32)
	}

	/// Gets the size in bytes of the packed element. If it's not a packed data type, it's equal to :class:`MemorySizeExt.size`.
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     int: (``u32``) Size in bytes of the packed element
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert MemorySizeExt.element_size(MemorySize.UINT32) == 4
	///     assert MemorySizeExt.element_size(MemorySize.PACKED256_UINT16) == 2
	///     assert MemorySizeExt.element_size(MemorySize.BROADCAST512_UINT64) == 8
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn element_size(memory_size: u32) -> PyResult<u32> {
		Ok(to_memory_size(memory_size)?.element_size() as u32)
	}

	/// Gets the element type if it's packed data or the input value if it's not packed data
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     :class:`MemorySize`: Element type
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert MemorySizeExt.element_type(MemorySize.UINT32) == MemorySize.UINT32
	///     assert MemorySizeExt.element_type(MemorySize.PACKED256_UINT16) == MemorySize.UINT16
	///     assert MemorySizeExt.element_type(MemorySize.BROADCAST512_UINT64) == MemorySize.UINT64
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn element_type(memory_size: u32) -> PyResult<u32> {
		Ok(to_memory_size(memory_size)?.element_type() as u32)
	}

	/// Gets the element type info if it's packed data or the input value if it's not packed data
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     :class:`MemorySizeInfo`: Element type info
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert MemorySizeExt.element_type_info(MemorySize.UINT32).memory_size == MemorySize.UINT32
	///     assert MemorySizeExt.element_type_info(MemorySize.PACKED256_UINT16).memory_size == MemorySize.UINT16
	///     assert MemorySizeExt.element_type_info(MemorySize.BROADCAST512_UINT64).memory_size == MemorySize.UINT64
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn element_type_info(memory_size: u32) -> PyResult<MemorySizeInfo> {
		Ok(MemorySizeInfo { info: to_memory_size(memory_size)?.element_type_info() })
	}

	/// ``True`` if it's signed data (signed integer or a floating point value)
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     bool: ``True`` if it's signed data
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert not MemorySizeExt.is_signed(MemorySize.UINT32)
	///     assert MemorySizeExt.is_signed(MemorySize.INT32)
	///     assert MemorySizeExt.is_signed(MemorySize.FLOAT64)
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn is_signed(memory_size: u32) -> PyResult<bool> {
		Ok(to_memory_size(memory_size)?.is_signed())
	}

	/// ``True`` if this is a packed data type, eg. :class:`MemorySize.PACKED128_FLOAT32`
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     bool: ``True`` if this is a packed data type
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert not MemorySizeExt.is_packed(MemorySize.UINT32)
	///     assert MemorySizeExt.is_packed(MemorySize.PACKED256_UINT16)
	///     assert not MemorySizeExt.is_packed(MemorySize.BROADCAST512_UINT64)
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn is_packed(memory_size: u32) -> PyResult<bool> {
		Ok(to_memory_size(memory_size)?.is_packed())
	}

	/// Gets the number of elements in the packed data type or ``1`` if it's not packed data (:class:`MemorySizeExt.is_packed`)
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     int: (``u32``) Number of elements in the packed data type
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert MemorySizeExt.element_count(MemorySize.UINT32) == 1
	///     assert MemorySizeExt.element_count(MemorySize.PACKED256_UINT16) == 16
	///     assert MemorySizeExt.element_count(MemorySize.BROADCAST512_UINT64) == 1
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn element_count(memory_size: u32) -> PyResult<u32> {
		Ok(to_memory_size(memory_size)?.element_count() as u32)
	}

	/// ``True`` if it is a broadcast memory type
	///
	/// Args:
	///     `memory_size` (:class:`MemorySize`): Enum value
	///
	/// Returns:
	///     bool: ``True`` if it is a broadcast memory type
	///
	/// Examples:
	///
	/// .. testcode::
	///
	///     from iced_x86 import *
	///
	///     assert not MemorySizeExt.is_broadcast(MemorySize.PACKED64_FLOAT16)
	///     assert MemorySizeExt.is_broadcast(MemorySize.BROADCAST512_UINT64)
	#[text_signature = "(memory_size)"]
	#[staticmethod]
	fn is_broadcast(memory_size: u32) -> PyResult<bool> {
		Ok(to_memory_size(memory_size)?.is_broadcast())
	}
}
