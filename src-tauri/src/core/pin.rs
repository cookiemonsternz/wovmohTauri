use crate::{core::node::Node, types::data_type::DataValue};

pub struct PinProperties {
    pub id: i32,
    pub parent_id: i32,
}

pub struct InputField {
    pub properties: PinProperties,
    pub value: DataValue,
    connected_output: Option<&OutputPin> = None
}

impl InputField {
    pub fn new(id: i32, parent_id: i32, value: DataValue) -> InputField {
        Self {
            properties: PinProperties { id, parent_id },
            value,
            connected_output: None,
        }
    }

    pub fn id(&self) -> i32 {
        self.properties.id
    }

    pub fn get_parent(&self) -> &Node {
        self.properties.parent
    }

    pub fn value(&self) -> &DataValue {
        match self.connected_output {
            Some(pin) => pin.value(),
            None => &self.value,
        }
    }

    pub fn set_value(&self, value: DataValue) {
        // Only if disconnected
        self.value = value
    }

    // Connections
    pub fn is_connected(&self) -> bool {
        match self.connected_output {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_connected_output(&self) -> Option<i32> {
        match self.connected_output {
            Some(pin) => Some(pin.properties.id),
            None => None,
        }
    }

    pub fn connect(&self, pin: &OutputPin) {
        self.connected_output = Some(pin)
    }

    pub fn disconnect(&self) {
        match self.connected_output {
            Some(output_pin) => {
                output_pin.disconnect(self);

                self.connected_output = None;
            }
            None => {
                panic!()
            }
        }
    }
}

pub struct OutputPin {
    properties: PinProperties,
    value: DataValue,
    connected_fields: Vec<&InputField>,
}

impl OutputPin {
    pub fn new(id: i32, parent_id: i32) -> OutputPin {
        Self {
            properties: PinProperties { id, parent_id },
            value: DataValue,
            connected_fields: Vec::new(),
        }
    }

    pub fn id(&self) -> i32 {
        self.properties.id
    }

    pub fn get_parent(&self) -> &Node {
        self.properties.parent
    }

    pub fn value(&self) -> &DataValue {
        &self.value
    }

    pub fn set_value(&self, value: DataValue) {
        self.value = value
    }

    pub fn is_connected(&self) -> bool {
        if (connected_fields.len() > 0) {
            true
        } else {
            false
        }
    }

    pub fn get_connected_inputs(&self) -> Vec<&InputField> {
        self.connected_fields
    }

    pub fn connect(&self, field: &InputField) {
        field.connect(self);
        self.connected_fields.push(field);
    }

    pub fn disconnect(&self, field: &InputField) {
        // Only remove from self, don't call disconnect on field as this function is called by the field
        // In effect, we just don't want to call OutputPin.disconnect() in almost every case - instead
        // use InputField.disconnect()
        let index = self.connected_fields.iter().position(|&x| x == field);
        match index {
            Some(field_index) => {
                self.connected_fields.remove(field_index);
            }
            None => {
                panic!();
            }
        }
    }

    pub fn disconnect_all(&self) {
        for input_field in self.connected_fields {
            input_field.disconnect();
        }
        // Should be unnecessary as input_field.disconnect() calls disconnect() on the output pin already
        // self.connected_fields.clear();
    }
}
