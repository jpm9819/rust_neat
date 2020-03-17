#![allow(dead_code)]

use super::Gene;

pub struct ConnectionGene {
    innovation_number: u32,
    neuron_in: u32,
    neuron_out: u32,
    weight: f64,
    enabled: bool,
}

impl ConnectionGene {
    pub fn new(innovation_number: u32, neuron_in: u32, neuron_out: u32, weight: f64) -> ConnectionGene {
        ConnectionGene {
            innovation_number,
            neuron_in,
            neuron_out,
            weight,
            enabled: true,
        }
    }

    pub fn get_neuron_in(&self) -> u32 {
        self.neuron_in
    }

    pub fn get_neuron_out(&self) -> u32 {
        self.neuron_out
    }
 
    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }
}

impl Gene for ConnectionGene {
    fn get_innovation_number(&self) -> u32 {
        self.innovation_number
    }
}