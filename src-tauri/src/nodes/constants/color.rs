use crate::core::node::*;
use crate::types::color::{Color, ColorValue};
use crate::types::data_type::*;
use std::sync::LazyLock;

fn constant_color_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_color = match inputs[0] {
        DataValue::Color(color) => color,
        _ => panic!("Expected Color"),
    };

    if let DataValue::Color(ref mut n) = outputs[0] {
        *n = input_color;
    };
}

pub static CONSTANT_COLOR_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Constant Color",
    inputs: &[InputDesc {
        name: "Color",
        data_type: DataType::Color,
        default: DataValue::Color(Color::default()),
    }],
    outputs: &[OutputDesc {
        name: "Color",
        data_type: DataType::Color,
    }],
    process: constant_color_process,
};
