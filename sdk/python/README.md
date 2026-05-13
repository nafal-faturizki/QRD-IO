# QRD-SDK Python

Privacy-native columnar binary container format - Python bindings.

## Installation

```bash
pip install qrd-sdk
```

## Usage

```python
from qrd_sdk import Schema, ColumnType, Reader, Writer

# Create schema
schema = Schema([
    ("id", ColumnType.INT64),
    ("name", ColumnType.STRING),
])

# Write data
writer = Writer()
writer.write_column([1, 2, 3])
data = writer.finish()

# Read data
reader = Reader(data)
column = reader.read_column(0)
```

## Development

See [../../CONTRIBUTING.md](../../CONTRIBUTING.md)
