use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

use super::node::*;
use super::pin::*;
use crate::core::node;
use crate::types::data_type::DataValue;

pub struct Graph {
    // field id : InputField
    input_fields: HashMap<usize, InputField>,
    // pin id : OutputPin
    output_pins: HashMap<usize, OutputPin>,

    nodes: Vec<Box<dyn Node>>,
    // node id : Vec Index
    nodes_map: HashMap<usize, usize>,
    // Whether nodes need resorting before evaluation
    dirty: bool,
}

impl Graph {
    // Pins
    pub fn input_fields(&self) -> &HashMap<usize, InputField> {
        &self.input_fields
    }

    pub fn output_pins(&self) -> &HashMap<usize, OutputPin> {
        &self.output_pins
    }

    pub fn get_input_field(&self, id: &usize) -> &InputField {
        self.input_fields().get(id).unwrap()
    }

    pub fn get_output_pin(&self, id: &usize) -> &OutputPin {
        self.output_pins().get(id).unwrap()
    }

    pub fn add_input_field(&mut self, field: InputField) {
        self.input_fields.insert(field.id(), field);
    }

    pub fn add_output_pin(&mut self, pin: OutputPin) {
        self.output_pins.insert(pin.id(), pin);
    }

    // Nodes
    pub fn nodes(&self) -> &Vec<Box<dyn Node>> {
        &self.nodes
    }

    pub fn nodes_map(&self) -> &HashMap<usize, usize> {
        &self.nodes_map
    }

    pub fn get_node_id(&self, node: &Box<dyn Node>) -> usize {
        self.nodes().iter().position(|x| x.eq(node)).unwrap()
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes_map
            .insert(node.properties().id, self.get_node_id(&node));
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, id: &usize) {
        // Disconnect nodes connections before removal
        let node = &self.nodes[*self.nodes_map.get(id).unwrap()];

        // Disconnect all inputs
        for input_field_id in node.input_fields() {
            let input_field = self.get_input_field(&input_field_id);
            input_field.disconnect(self);
        }

        // Disconnect all outputs
        for output_pin_id in node.output_pins() {
            let output_pin = self.get_output_pin(&output_pin_id);
            output_pin.disconnect_all();
        }

        // Remove from map and vector
        let index = self.nodes_map.remove(id).unwrap();
        self.nodes.remove(index);

        self.dirty = true
    }

    pub fn connect(
        &mut self,
        src_node_id: i32,
        src_pin_id: i32,
        dst_node_id: i32,
        dst_field_id: i32,
    ) {
        let source_node = &self.nodes[*self.nodes_map.get(&src_node_id).unwrap()];
        let dst_node = &self.nodes[*self.nodes_map.get(&dst_node_id).unwrap()];

        let output_pin = source_node.get_output_pin(src_pin_id);
        let input_field = dst_node.get_input_field(dst_field_id);

        output_pin.connect(input_field);

        self.dirty = true
    }

    pub fn disconnect(&mut self, dst_node_id: i32, dst_field_id: i32) {
        let dst_node = &self.nodes[*self.nodes_map.get(&dst_node_id).unwrap()];
        let input_field = dst_node.get_input_field(dst_field_id);

        input_field.disconnect();

        self.dirty = true
    }

    pub fn evaluate(&mut self) {
        if self.dirty {
            self.sort();
        }

        for node in self.nodes() {
            node.process()
        }
    }

    pub fn sort(&mut self) {
        let nodes = self.get_topologically_sorted_nodes();
        self.nodes.clear();

        for node in nodes {
            self.nodes[node.properties().id] = node
        }
    }

    fn get_topologically_sorted_nodes(&self) -> Vec<&Box<dyn Node>> {
        let mut in_degrees: HashMap<i32, i32> = HashMap::new();
        let mut queue: VecDeque<&Box<dyn Node>> = VecDeque::new();
        let mut sorted_result: Vec<&Box<dyn Node>> = Vec::new();

        for node in self.nodes() {
            let in_degree = node.get_in_degree();

            // in_degrees[&node.properties().id] = node.get_in_degree();
            in_degrees.insert(node.properties().id, node.get_in_degree());

            if in_degree == 0 {
                queue.push_front(node);
            }
        }

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            sorted_result.push(node);

            // Add connections to queue
            for output_pin in node.output_pins() {
                for input_field in output_pin.get_connected_inputs() {
                    let node = &self.nodes[self.nodes_map[&input_field.get_parent_id()]];
                    // subtract 1 from in degreees
                    in_degrees
                        .entry(node.properties().id)
                        .and_modify(|x| *x -= 1);

                    if in_degrees[&node.properties().id] == 0 {
                        queue.push_back(node);
                    }
                } // for field
            } // for pin
        } // while queue

        // Check for loops
        if sorted_result.len() != self.nodes().len() {
            panic!()
        }

        return sorted_result;
    }
}
