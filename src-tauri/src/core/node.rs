use super::super::types::data_type::DataValue;
use super::pin::*;

pub struct NodeProperties {
    pub id: i32,
}

pub trait Node {
    pub fn properties(&self) -> &NodeProperties;
    pub fn input_fields(&self) -> &Vec<InputField>;
    pub fn output_pins(&self) -> &Vec<OutputPin>;

    pub fn get_input_field(&self, id: i32) -> &InputField {
        self.input_fields()
            .into_iter()
            .find(|&x| x.id == id)
            .unwrap()
    }

    pub fn get_output_pin(&self, id: i32) -> &OutputPin {
        self.output_pins()
            .into_iter()
            .find(|&x| x.id == id)
            .unwrap()
    }

    pub fn get_in_degree(&self) -> Option<int> {
        let mut in_degree = 0;

        for input_field in self.input_fields() {
            if input_field.is_connected() {
                in_degree += 1;
            }
        }

        match in_degree {
            0 => None,
            _ => Some(in_degree),
        }
    }

    // Updates output pin values based on input field values
    pub fn process(&self);
}
