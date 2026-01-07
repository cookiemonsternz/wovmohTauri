use std::collections::HashMap;

use crate::types::data_type::DataValue;
use super::node::*;
use super::pin::*;
pub struct Graph {
    nodes: Vec< &Node>,
    nodes_map: HashMap<i32, &Node>
}

impl Graph {
    pub fn nodes(&self) -> Vec<&Node> { self.nodes }
    pub fn nodes_map(&self) -> HashMap<i32, &Node> { self.nodes_map }
    
    pub fn add_node(&self, node: &Node) {
        self.nodes_map.insert(node.properties().id, node);
        self.nodes.push(node);
    }

    pub fn remove_node(&self, id: &i32) {
        let node = self.nodes_map.remove(id);
        let index = self.nodes.iter().position(|&x| x==node).unwrap();
        self.nodes.remove(index);
    }
    
    pub fn connect(&self, src_node_id: i32, src_pin_id: i32, dst_node_id: i32, dst_field_id: i32);

    pub fn disconnect(&self, dst_node_id: i32, dst_field_id: i32);

    pub fn evaluate(&self);

    pub fn sort(&self);

    fn get_topologically_sorted_nodes(&self) -> Vec<Node>;
}