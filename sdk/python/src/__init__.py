# -*- coding: utf-8 -*-
"""QRD-SDK Python bindings"""

from enum import Enum
from typing import List, Tuple, Optional

__version__ = "1.0.0"


class ColumnType(Enum):
    """Supported column types"""
    NULL = "Null"
    BOOLEAN = "Boolean"
    INT8 = "Int8"
    INT16 = "Int16"
    INT32 = "Int32"
    INT64 = "Int64"
    UINT32 = "UInt32"
    UINT64 = "UInt64"
    FLOAT32 = "Float32"
    FLOAT64 = "Float64"
    STRING = "String"
    BINARY = "Binary"
    DATE32 = "Date32"
    TIMESTAMP = "Timestamp"


class Schema:
    """Schema definition for QRD files"""

    def __init__(self, columns: List[Tuple[str, ColumnType]]):
        self.columns = columns

    def validate(self) -> None:
        """Validate schema"""
        if not self.columns:
            raise ValueError("Schema cannot be empty")


class Reader:
    """Reader for QRD files"""

    def __init__(self, data: bytes):
        self.data = data

    def read_column(self, index: int) -> bytes:
        """Read column data"""
        raise NotImplementedError()


class Writer:
    """Writer for QRD files"""

    def __init__(self):
        self.columns = []

    def write_column(self, data: bytes) -> None:
        """Write column data"""
        raise NotImplementedError()

    def finish(self) -> bytes:
        """Finish writing and return bytes"""
        raise NotImplementedError()
