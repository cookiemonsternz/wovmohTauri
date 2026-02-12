use std::collections::{HashMap, VecDeque};

use crate::{
    core::{input_field::*, node::*, output_pin::*},
    types::data_type::DataValue,
};

pub type GraphId = usize;

pub struct Graph {
    nodes: Vec<Node>,
    inputs: Vec<InputField>,
    outputs: Vec<OutputPin>,

    nodes_map: HashMap<NodeId, usize>,
    execution_order: Vec<NodeId>,

    order_dirty: bool,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            nodes_map: HashMap::new(),
            execution_order: Vec::new(),
            order_dirty: false,
        }
    }

    pub fn add_node(&mut self, kind: NodeKind) {
        let desc = kind.descriptor();

        let mut inputs = Vec::new();
        // Create Input Fields
        for (i, input_desc) in desc.inputs.iter().enumerate() {
            inputs.push(self.inputs.len());
            self.inputs.push(InputField {
                parent: self.nodes.len(),
                index: i as u8,
                value: input_desc.default,
                connected_output: None,
            });
        }

        let mut outputs = Vec::new();
        // Create Output Pins
        for (i, output_desc) in desc.outputs.iter().enumerate() {
            outputs.push(self.outputs.len());
            self.outputs.push(OutputPin {
                parent: self.nodes.len(),
                index: i as u8,
                value: DataValue::default(output_desc.data_type.clone()),
                connections: Vec::new(),
            });
        }

        // Create Node
        let node = Node {
            id: self.nodes.len(),
            kind,
            inputs,
            outputs,
        };
        self.nodes.push(node);

        self.order_dirty = true;
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        // Swap and remove, then update node id.

        let node = self.nodes[node_id].clone();

        // Swap and remove all inputs
        for index in &node.inputs {
            // Get the parent of the input that will be swapped and change the parents reference to the input to
            // the current index
            let swapped_input_id = self.inputs.len() - 1;
            let swapped_input_parent = &mut self.nodes[self.inputs.last().unwrap().parent];
            let swapped_input_index = swapped_input_parent
                .inputs
                .iter()
                .find(|&x| *x == swapped_input_id)
                .unwrap()
                .clone();

            // Set the input id at the input to be swapped to the id to be swapped to
            swapped_input_parent.inputs[swapped_input_index] = *index;

            // Now swap
            self.inputs.swap_remove(*index);
        }

        // Swap and remove all outputs
        for index in node.outputs {
            // Get parent of output that will be swapped, and change the parents ref to the output to the current index.
            let swapped_output_id = self.outputs.len() - 1;
            let swapped_output_parent = &mut self.nodes[self.outputs.last().unwrap().parent];
            let swapped_output_index = swapped_output_parent
                .inputs
                .iter()
                .find(|&x| *x == swapped_output_id)
                .unwrap()
                .clone();

            swapped_output_parent.outputs[swapped_output_index] = index;

            // swap
            self.outputs.swap_remove(index);
        }

        // Swap and remove node
        self.nodes.swap_remove(node_id);

        // Update node at node_id
        let node = &mut self.nodes[node_id];
        node.id = node_id;

        // Update all fields and pins parent ids
        for input_id in &node.inputs {
            let input_field = &mut self.inputs[*input_id];
            input_field.parent = node_id;
        }

        // Update all outputs
        for output_id in &node.outputs {
            let output_pin = &mut self.outputs[*output_id];
            output_pin.parent = node_id;
        }

        self.order_dirty = true
    }

    pub fn connect(&mut self, from: OutputId, to: InputId) {
        let input_field = &mut self.inputs[to];
        input_field.connected_output = Some(from);

        let output_pin = &mut self.outputs[from];
        output_pin.connections.push(to);

        self.order_dirty = true;
    }

    pub fn disconnect(&mut self, input_field_id: InputId) {
        let input_field = &mut self.inputs[input_field_id];

        match input_field.connected_output {
            Some(id) => id,
            None => panic!("Cannot disconnect, not connected!"),
        };
        input_field.connected_output = None;

        self.order_dirty = true;
    }

    fn inputs_for(&self, node_id: NodeId) -> Vec<&InputField> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.inputs.iter().map(|&id| &self.inputs[id]).collect()
    }

    fn outputs_for(&self, node_id: NodeId) -> Vec<&OutputPin> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.outputs.iter().map(|&id| &self.outputs[id]).collect()
    }

    fn outputs_for_mut(&mut self, node_id: NodeId) -> Vec<&mut OutputPin> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        // node.outputs
        //     .iter()
        //     .map(|&id| &mut self.outputs[id])
        //     .collect()
        // Hacky (still safe but in future use Vec.get_many_mut())
        let mut result = Vec::with_capacity(node.outputs.len());
        let outputs = self.outputs.as_mut_ptr();

        for &id in &node.outputs {
            unsafe {
                // Should be safe...
                result.push(&mut *outputs.add(id));
            }
        }

        result
    }

    fn input_values_for(&self, node_id: NodeId) -> Vec<DataValue> {
        self.inputs_for(node_id)
            .iter()
            .map(|input| match input.connected_output {
                Some(output_id) => self.outputs[output_id].value.clone(),
                None => input.value.clone(),
            })
            .collect()
    }

    fn output_values_for(&mut self, node_id: NodeId) -> Vec<&mut DataValue> {
        self.outputs_for_mut(node_id)
            .into_iter()
            .map(|output| &mut output.value)
            .collect()
    }

    fn calculate_indegrees(&self) -> HashMap<NodeId, u32> {
        let mut indegrees = HashMap::with_capacity(self.nodes.len());

        for input in &self.inputs {
            if input.connected_output.is_some() {
                let indegrees_val = indegrees.get_mut(&input.parent).unwrap();
                *indegrees_val -= 1;
            }
        }

        indegrees
    }

    fn sort_nodes(&mut self) {
        let mut in_degrees = self.calculate_indegrees();
        let mut queue: VecDeque<NodeId> = VecDeque::new();

        for (id, in_degree) in in_degrees.iter() {
            if *in_degree == 0 {
                queue.push_back(*id);
            }
        }

        self.execution_order.clear();
        self.execution_order.resize(self.nodes.len(), 0);

        while !queue.is_empty() {
            let node_id = queue.pop_front().unwrap();

            self.execution_order.push(node_id);

            // Add connections to queue
            for output in self.outputs_for(node_id) {
                for input_id in &output.connections {
                    let input_field = &self.inputs[*input_id];
                    // let node = self.nodes[input_field.parent];

                    in_degrees.entry(input_field.parent).and_modify(|x| *x -= 1);

                    if in_degrees[&input_field.parent] == 0 {
                        queue.push_back(input_field.parent);
                    }
                }
            }
        }

        // Check for loops
        if self.execution_order.len() != self.nodes.len() {
            panic!("Loop in graph");
        }

        self.order_dirty = false;
    }

    pub fn process(&mut self) {
        if self.order_dirty {
            self.sort_nodes()
        };

        for node_id in &self.execution_order.clone() {
            let node = &self.nodes[*node_id].clone();

            let inputs = self.input_values_for(*node_id);
            let mut outputs = self.output_values_for(*node_id);

            (node.kind.descriptor().process)(inputs, outputs.as_mut());
        }
    }
}

// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::hash::Hash;

// use super::node::*;
// use super::pin::*;
// use crate::core::node;
// use crate::types::data_type::DataValue;

// pub struct Graph {
//     // field id : InputField
//     input_fields: HashMap<usize, InputField>,
//     // pin id : OutputPin
//     output_pins: HashMap<usize, OutputPin>,

//     nodes: Vec<Box<dyn Node>>,
//     // node id : Vec Index
//     nodes_map: HashMap<usize, usize>,
//     // Whether nodes need resorting before evaluation
//     dirty: bool,
// }

// impl Graph {
//     // Pins
//     pub fn input_fields(&self) -> &HashMap<usize, InputField> {
//         &self.input_fields
//     }

//     pub fn input_fields_mut(&mut self) -> &mut HashMap<usize, InputField> {
//         &mut self.input_fields
//     }

//     pub fn output_pins(&self) -> &HashMap<usize, OutputPin> {
//         &self.output_pins
//     }

//     pub fn output_pins_mut(&mut self) -> &mut HashMap<usize, OutputPin> {
//         &mut self.output_pins
//     }

//     pub fn get_input_field(&self, id: &usize) -> &InputField {
//         self.input_fields().get(id).unwrap()
//     }

//     pub fn get_input_field_mut(&mut self, id: &usize) -> &mut InputField {
//         self.input_fields_mut().get_mut(id).unwrap()
//     }

//     pub fn get_output_pin(&self, id: &usize) -> &OutputPin {
//         self.output_pins().get(id).unwrap()
//     }

//     pub fn get_output_pin_mut(&mut self, id: &usize) -> &mut OutputPin {
//         self.output_pins_mut().get_mut(id).unwrap()
//     }

//     pub fn add_input_field(&mut self, field: InputField) {
//         self.input_fields_mut().insert(field.id(), field);
//     }

//     pub fn add_output_pin(&mut self, pin: OutputPin) {
//         self.output_pins_mut().insert(pin.id(), pin);
//     }

//     pub fn set_input_field_value(&mut self, id: &usize, value: DataValue) {
//         let input_field = self.get_input_field_mut(id);
//         input_field.value = value;
//     }

//     pub fn get_input_field_value(&self, id: &usize) -> &DataValue {
//         let input_field = self.get_input_field(id);

//         match input_field.get_connected_output() {
//             Some(output_pin_id) => {
//                 let output_pin = self.get_output_pin(&output_pin_id);
//                 output_pin.value()
//             }
//             None => input_field.value(),
//         }
//     }

//     pub fn get_output_pin_value(&self, id: &usize) -> &DataValue {
//         self.get_output_pin(id).value()
//     }

//     fn disconnect_input_field(&mut self, input_field_id: &usize) {
//         let input_field = self.get_input_field_mut(input_field_id);
//         let connected_output_id = input_field.get_connected_output().unwrap();
//         // Clear ref to pin from field
//         input_field.clear_connected_output();
//         // Drop so that we can do another mut borrow
//         drop(input_field);

//         // clear ref to field from pin
//         let output_field = self.get_output_pin_mut(&connected_output_id);
//         output_field.remove_connected_field(*input_field_id);

//         self.dirty = true;
//     }

//     // pub fn disconnect_input_field(&mut self, input_field_id: &usize) {
//     //     let input_field = self.get_input_field_mut(input_field_id);

//     //     input_field.disconnect();
//     // }

//     // pub fn disconnect_output_pin(&mut self, output_pin_id: &usize, input_field_id: usize) {
//     //     // Can just disconnect the input field, as only one output can be connected to an input
//     //     self.disconnect_input_field(&input_field_id);
//     // }

//     pub fn disconnect_output_pin_all(&mut self, output_pin_id: &usize) {
//         let output_pin = self.get_output_pin_mut(output_pin_id);

//         let input_field_ids: Vec<_> = output_pin.get_connected_inputs().clone();

//         for input_field_id in input_field_ids {
//             self.disconnect_input_field(&input_field_id);
//         }

//         self.dirty = true;
//     }

//     // pub fn disconnect_pin_pair(&mut self, output_pin_id: &usize, input_field_id: &usize) {
//     //     let output_pin = self.get_output_pin_mut(output_pin_id);
//     //     output_pin.disconnect(*input_field_id);

//     //     self.get_input_field_mut(input_field_id).disconnect();
//     // }

//     // Nodes
//     pub fn nodes(&self) -> &Vec<Box<dyn Node>> {
//         &self.nodes
//     }

//     pub fn nodes_map(&self) -> &HashMap<usize, usize> {
//         &self.nodes_map
//     }

//     pub fn get_node_id(&self, node: &Box<dyn Node>) -> usize {
//         self.nodes().iter().position(|x| x.eq(node)).unwrap()
//     }

//     pub fn add_node(&mut self, node: Box<dyn Node>) {
//         self.nodes_map
//             .insert(node.properties().id, self.get_node_id(&node));
//         self.nodes.push(node);
//     }

//     pub fn remove_node(&mut self, id: &usize) {
//         // Disconnect nodes connections before removal
//         let node = &self.nodes[*self.nodes_map.get(id).unwrap()];
//         let input_field_ids = node.input_fields();
//         let output_pin_ids = node.output_pins();

//         // Disconnect all inputs
//         for input_field_id in input_field_ids {
//             self.disconnect_input_field(&input_field_id);
//         }

//         // Disconnect all outputs
//         for output_pin_id in output_pin_ids {
//             self.disconnect_output_pin_all(&output_pin_id);
//         }

//         // Remove from map and vector
//         let index = self.nodes_map.remove(id).unwrap();
//         self.nodes.remove(index);

//         self.dirty = true
//     }

//     pub fn connect(&mut self, src_pin_id: usize, dst_field_id: usize) {
//         let output_pin = self.get_output_pin_mut(&src_pin_id);
//         output_pin.add_connected_field(dst_field_id);

//         let input_field = self.get_input_field_mut(&dst_field_id);
//         input_field.set_connected_output(&src_pin_id);

//         self.dirty = true
//     }

//     // pub fn disconnect(&mut self, dst_node_id: usize, dst_field_id: usize) {
//     //     self.disconnect_input_field(&dst_field_id);

//     //     self.dirty = true
//     // }

//     pub fn evaluate(&mut self) {
//         if self.dirty {
//             self.sort();
//         }

//         for node in self.nodes() {
//             node.process()
//         }
//     }

//     pub fn sort(&mut self) {
//         let nodes = self.get_topologically_sorted_nodes();
//     }

//     fn get_topologically_sorted_nodes(&self) -> Vec<Box<dyn Node>> {
//         let mut in_degrees: HashMap<usize, usize> = HashMap::new();
//         let mut queue: VecDeque<Box<dyn Node>> = VecDeque::new();
//         let mut sorted_result: Vec<Box<dyn Node>> = Vec::new();

//         for node in self.nodes().clone() {
//             let in_degree = node.get_in_degree(self);

//             // in_degrees[&node.properties().id] = node.get_in_degree();
//             in_degrees.insert(node.properties().id, node.get_in_degree(self));

//             if in_degree == 0 {
//                 queue.push_front(node);
//             }
//         }

//         while !queue.is_empty() {
//             let node = queue.pop_front().unwrap();

//             sorted_result.push(node);

//             // Add connections to queue
//             for output_pin_id in node.output_pins() {
//                 let output_pin = self.get_output_pin(&output_pin_id);
//                 for input_field_id in output_pin.get_connected_inputs() {
//                     let input_field = self.get_input_field(&input_field_id);
//                     let node = &self.nodes[self.nodes_map[&input_field.get_parent_id()]];
//                     // subtract 1 from in degreees
//                     in_degrees
//                         .entry(node.properties().id)
//                         .and_modify(|x| *x -= 1);

//                     if in_degrees[&node.properties().id] == 0 {
//                         queue.push_back(node);
//                     }
//                 } // for field
//             } // for pin
//         } // while queue

//         // Check for loops
//         if sorted_result.len() != self.nodes().len() {
//             panic!()
//         }

//         return sorted_result;
//     }
// }
