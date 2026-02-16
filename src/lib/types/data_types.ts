export enum DataType {
  Number,
  Boolean,
  Color,
  Vector3,
  Point3,
}

export type Color = { r: number; g: number; b: number; a: number };
export type Vec3 = { x: number; y: number; z: number };
export type Point3 = Vec3;

export type DataValue =
  | { type: DataType.Number; value: number }
  | { type: DataType.Boolean; value: boolean }
  | { type: DataType.Color; value: Color }
  | { type: DataType.Vector3; value: Vec3 }
  | { type: DataType.Point3; value: Point3 };
