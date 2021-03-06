extern crate rand;

use rand::prelude::*;

pub struct GenomeConfig {
    n_sensor: u32,
    n_output: u32,
    is_connected: bool,
    default_weight: f64,
    weight_is_random: bool,
    weight_deviation: f64,
    mutate_create_connection: f64,
    mutate_create_neuron: f64,
    mutate_set_weight: f64,
    mutate_update_weight: f64,
    mutate_toggle_connection: f64,
}

impl GenomeConfig {
    pub fn new(n_sensor: u32, n_output: u32) -> GenomeConfig {
        GenomeConfig {
            n_sensor,
            n_output,
            is_connected: false,
            default_weight: 0.0,
            weight_is_random: true,
            weight_deviation: 3.0,
            mutate_create_connection: 0.05,
            mutate_create_neuron: 0.03,
            mutate_set_weight: 0.15,
            mutate_update_weight: 0.2,
            mutate_toggle_connection: 0.1,
        }
    }

    pub fn set_mutate_toggle_connection(&mut self, value: f64) {
        self.mutate_toggle_connection = value;
    }

    pub fn set_mutate_update_weight(&mut self, value: f64) {
        self.mutate_update_weight = value;
    }

    pub fn set_mutate_set_weight(&mut self, value: f64) {
        self.mutate_set_weight = value;
    }

    pub fn set_mutate_create_neuron(&mut self, value: f64) {
        self.mutate_create_neuron = value;
    }

    pub fn set_mutate_create_connection(&mut self, value: f64) {
        self.mutate_create_connection = value;
    }

    pub fn get_mutate_toggle_connection(&self) -> f64 {
        self.mutate_toggle_connection
    }

    pub fn get_mutate_update_weight(&self) -> f64 {
        self.mutate_update_weight
    }

    pub fn get_mutate_set_weight(&self) -> f64 {
        self.mutate_set_weight
    }

    pub fn get_mutate_create_neuron(&self) -> f64 {
        self.mutate_create_connection
    }

    pub fn get_mutate_create_connection(&self) -> f64 {
        self.mutate_create_connection
    }

    pub fn set_is_connected(&mut self, is_connected: bool) {
        self.is_connected = is_connected;
    }

    pub fn set_default_weight(&mut self, default_weight: f64) {
        self.default_weight = default_weight;
    }

    pub fn set_weight_is_random(&mut self, weight_is_random: bool) {
        self.weight_is_random = weight_is_random;
    }

    pub fn set_weight_deviation(&mut self, weight_deviation: f64) {
        self.weight_deviation = weight_deviation;
    }

    pub fn get_n_sensor(&self) -> u32 {
        self.n_sensor
    }

    pub fn get_n_output(&self) -> u32 {
        self.n_output
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn is_weight_random(&self) -> bool {
        self.weight_is_random
    }

    pub fn get_weight_deviation(&self) -> f64 {
        self.weight_deviation
    }

    pub fn get_random_weight(&self) -> f64 {
        self.default_weight + thread_rng().gen::<f64>() * 2.0 * self.weight_deviation - self.weight_deviation
    }

    pub fn get_weight(&self) -> f64 {
        if self.weight_is_random {
            self.get_random_weight()
        } else {
            self.default_weight
        }
    }
}
