# How is the backend structured

**There are four main structures in the backend of wovmoH:**

## Node

- **\<Node\>**:
  - **id**<sub>\<NodeId\></sub> *- Node Id*
  - **kind**<sub>\<NodeKind\></sub> *- Node Kind*
  - **inputs_offset**<sub>\<usize\></sub> *- Index of the start of the nodes inputs in graph.inputs*
  - **outputs_offset**<sub>\<usize\></sub> *- Index of the start of the nodes outputs in graph.inputs*
---
- ***Related Types***:
  - **\<NodeKind\> Enum**:
    - Members are node types, e.g:
      - ADD
      - SUBTRACT
      - CONSTANT_NUMBER
    - **fn descriptor() -> \<&'static NodeDescriptor\>** *- Get the descriptor; Matches enum*
  - **\<NodeDescriptor\>**:
    - **name**<sub>\<&'static str\></sub> *- Name of the node type*
    - **inputs**<sub>\<&'static [InputDesc]\></sub>  *- Array of **InputDesc**
    - **outputs**<sub>\<&'static [OutputDesc]\></sub> *- Array of **OutputDesc**
    - **process**<sub>\<fn(&[&DataValue], &[&mut DataValue]\></sub> *- Function which take inputs and sets outputs*
  - **\<InputDesc\>**:
    - **name**<sub>\<&'static str\></sub> *- Name of the Input Field*
    - **data_type**<sub>\<DataType\></sub> *- Data type of the Input Field*
    - **default**<sub>\<DataValue\></sub> *- Default value of the Input Field*
  - **\<OutputDesc\>**:
    - **name**<sub>\<&'static str\></sub> *- Name of the Output Pin*
    - **data_type**<sub>\<DataType\></sub> *- Data type of the Output Pin*
  - **\<NodeId\>** : **\<u32\>**

## Fields and Pins
- **\<InputField\>**:
  - An input field stores a reference to its parent node, its index in **node.descriptor.inputs**, and the connected output.
  ---
  - **parent**<sub>\<NodeId\></sub> *- Id of the parent node*
  - **index**<sub>\<u8\></sub> *- Index of this field in **node.descriptor.inputs***
  - **value**<sub>\<DataValue\></sub> *- Value of the pin (Only used if not connected)*
  - **connected_output**<sub>\<Option\<OutputId\>\></sub> *- Optional connected output pin id*
- **\<OutputPin\>**:
  - An **OutputPin** is the same as an **InputField** except it has a vector of connections.
  ---
  - **parent**<sub>\<NodeId\></sub> *- Id of the parent node*
  - **index**<sub>\<u8\></sub> *- Index of this pin in **node.descriptor.outputs***
  - **value**<sub>\<DataValue\></sub> *- Value of the pin*
  - **connections**<sub>\<Vec\<InputId\>\></sub> *- Vector of connected InputField ids*
---
- ***Related Types***:
  - **\<InputId\>** : **\<u32\>**
  - **\<OutputId\>** : **\<u32\>**

## Graphs
- **\<Graph\>**:
  - **nodes**<sub>\<Vec\<Node\>\></sub> *- Vector owning nodes (node id is index)*
  - **inputs**<sub>\<Vec\<InputField\>\></sub> *- Vector owning input fields (inputfield id is index)*
  - **inputs**<sub>\<Vec\<OutputPin\>\></sub> *- Vector owning output pins (outputpin id is index)*

  - **nodes_map**<sub>\<HashMap\<NodeId, usize\>\></sub> *- Hashmap mapping NodeId -> **graph.nodes** index*
  - **execution_order**<sub>\<Vec\<NodeId\>\></sub> *- Ordered Vec of NodeId's, order nodes are processed*

  - **order_dirty**<sub>\<bool\></sub> *- Anytime the graph is invalidated (e.g, connection made, node removed that was connected to things, etc...) this becomes true. If true, on next process the graph will be sorted.*

## Graph Manager
- **\<GraphManager\>**:
  - **graphs**<sub>\<Vec\<Graph\>\></sub> *- Vector owning graphs (graph id is index)*