use std::collections::HashMap;
use std::collections::VecDeque;

use super::node::*;
use super::pin::*;
use crate::core::node;
use crate::types::data_type::DataValue;

pub struct Graph {
    nodes: Vec<&Node>,
    nodes_map: HashMap<i32, &Node>,
    // Whether nodes need resorting before evaluation
    dirty: bool,
}

impl Graph {
    pub fn nodes(&self) -> Vec<&Node> {
        self.nodes
    }
    pub fn nodes_map(&self) -> HashMap<i32, &Node> {
        self.nodes_map
    }

    pub fn add_node(&self, node: &Node) {
        self.nodes_map.insert(node.properties().id, node);
        self.nodes.push(node);
    }

    pub fn remove_node(&self, id: &i32) {
        // Disconnect nodes connections before removal
        let node = self.nodes_map.get(id).unwrap();

        // Disconnect all inputs
        for input_field in node.input_fields() {
            input_field.disconnect();
        }

        // Disconnect all outputs
        for output_pin in node.output_pins() {
            output_pin.disconnect_all();
        }

        // Remove from map and vector
        let node = self.nodes_map.remove(id);
        let index = self.nodes.iter().position(|&x| x == node).unwrap();
        self.nodes.remove(index);

        self.dirty = true
    }

    pub fn connect(&self, src_node_id: i32, src_pin_id: i32, dst_node_id: i32, dst_field_id: i32) {
        let source_node = self.nodes_map().get(&src_node_id).unwrap();
        let dst_node = self.nodes_map().get(&dst_node_id).unwrap();

        let output_pin = source_node.get_output_pin(src_pin_id);
        let input_field = dst_node.get_input_field(dst_field_id);

        output_pin.connect(input_field);

        self.dirty = true
    }

    pub fn disconnect(&self, dst_node_id: i32, dst_field_id: i32) {
        let dst_node = self.nodes_map().get(&dst_node_id).unwrap();
        let input_field = dst_node.get_input_field(dst_field_id);

        input_field.disconnect();

        self.dirty = true
    }

    pub fn evaluate(&self) {
        if self.dirty {
            self.sort();
        }

        for node in self.nodes() {
            node.process()
        }
    }

    pub fn sort(&self) {
        self.nodes = self.get_topologically_sorted_nodes()
    }

    fn get_topologically_sorted_nodes(&self) -> Vec<&Node> {
        let in_degrees: HashMap<&Node, int>;
        let queue: VecDeque<&Node>;
        let sorted_result: Vec<&Node>;

        for node in self.nodes() {
            let in_degree = node.get_in_degree();

            in_degree[node] = node.get_in_degree();

            if in_degree == 0 {
                queue.push(node);
            }
        }

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            sorted_result.push(node);

            // Add connections to queue
            for output_pin in node.output_pins() {
                for input_field in output_pin.get_connected_inputs() {
                    let node = input_field.get_parent();
                    in_degrees[node] -= 1;

                    if in_degrees[node] == 0 {
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
