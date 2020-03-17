#![allow(dead_code)]

use super::Gene;

pub const SENSOR: u32 = std::u32::MIN;
pub const OUTPUT: u32 = std::u32::MAX;

pub struct NeuronGene {
    innovation_number: u32,
    class: u32,
}

impl NeuronGene {
    pub fn new(innovation_number: u32, class: u32) -> NeuronGene {
        NeuronGene {
            innovation_number,
            class,
        }
    }

    pub fn get_class(&self) -> u32 {
        self.class
    }
}

impl Gene for NeuronGene {
    fn get_innovation_number(&self) -> u32 {
        self.innovation_number
    }
}