use super::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub enum DataValue {
    Number(f64),
    Boolean(bool),
    Color(color::Color),
    Vector3(vector::Vec3),
    Point3(vector::Point3),
}

impl DataValue {
    // Used for errors
    pub fn type_name(&self) -> &'static str {
        match self {
            DataValue::Number(_) => "Number",
            DataValue::Boolean(_) => "Boolean",
            DataValue::Color(_) => "Color",
            DataValue::Vector3(_) => "Vector3",
            DataValue::Point3(_) => "Point3",
        }
    }
}

// Error type for DataValue ops
pub enum EvalError {
    TypeError {
        op: &'static str,
        lhs: &'static str,
        rhs: &'static str,
    },
    UnsupportedOp {
        op: &'static str,
        lhs: &'static str,
        rhs: &'static str,
    },
}

impl Add for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn add(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a + b)),

            (Color(a), Color(b)) => Ok(Color(a + b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a + b)),
            (Point3(a), Point3(b)) => Ok(Point3(a + b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Neg for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn neg(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a - b)),

            (Color(a), Color(b)) => Ok(Color(a - b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a - b)),
            (Point3(a), Point3(b)) => Ok(Point3(a - b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Mul for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn mul(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a * b)),

            (Color(a), Color(b)) => Ok(Color(a * b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a * b)),
            (Point3(a), Point3(b)) => Ok(Point3(a * b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Div for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn div(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a / b)),

            (Color(a), Color(b)) => Ok(Color(a / b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a / b)),
            (Point3(a), Point3(b)) => Ok(Point3(a / b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}
