// QRD-SDK Node.js Bindings
// FFI wrapper around qrd-core

export interface Schema {
  columns: ColumnDef[];
}

export interface ColumnDef {
  name: string;
  type: ColumnType;
  nullable: boolean;
}

export enum ColumnType {
  Null = "Null",
  Boolean = "Boolean",
  Int8 = "Int8",
  Int16 = "Int16",
  Int32 = "Int32",
  Int64 = "Int64",
  UInt32 = "UInt32",
  UInt64 = "UInt64",
  Float32 = "Float32",
  Float64 = "Float64",
  String = "String",
  Binary = "Binary",
  Date32 = "Date32",
  Timestamp = "Timestamp",
}

export class Reader {
  constructor(data: Buffer) {}
  readColumn(index: number): Buffer {
    throw new Error("Not implemented");
  }
}

export class Writer {
  writeColumn(data: Buffer): void {
    throw new Error("Not implemented");
  }
  finish(): Buffer {
    throw new Error("Not implemented");
  }
}

export const version = "1.0.0";
