use crate::core::graph::Graph;

use super::super::types::data_type::DataValue;
use super::pin::*;

pub struct NodeProperties {
    pub id: usize,
}

pub trait Node {
    fn properties(&self) -> &NodeProperties;
    fn input_fields(&self) -> Vec<usize>;
    fn output_pins(&self) -> Vec<usize>;

    fn eq(&self, other: &Box<dyn Node>) -> bool {
        self.properties().id == other.properties().id
    }

    // fn get_input_field(&self, id: usize) -> &InputField {
    //     self.input_fields()
    //         .into_iter()
    //         .find(|x| x.id() == id)
    //         .unwrap()
    // }

    // fn get_output_pin(&self, id: usize) -> &OutputPin {
    //     self.output_pins()
    //         .into_iter()
    //         .find(|x| x.id() == id)
    //         .unwrap()
    // }

    // fn add_input_field(&mut self, id: usize, value: DataValue) {
    //     let input_field = InputField::new(id, self.properties().id, value);

    //     self.input_fields().push(input_field);
    // }

    // fn add_output_pin(&mut self, id: usize) {
    //     let output_pin = OutputPin::new(id, self.properties().id);

    //     self.output_pins().push(output_pin);
    // }

    fn get_in_degree(&self, graph: &Graph) -> usize {
        let mut in_degree = 0;

        for input_field_id in self.input_fields() {
            let input_field = graph.get_input_field(&input_field_id);
            if input_field.is_connected() {
                in_degree += 1;
            }
        }

        in_degree
    }

    // Updates output pin values based on input field values
    fn process(&self);
}
