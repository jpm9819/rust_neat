#![feature(is_sorted)]
#![allow(dead_code)]

mod utils;
mod genome;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use genome::Genome;

pub struct Neat {
    counter: Rc<RefCell<InnovationCounter>>,
    population_genome: Vec<Genome>
}

#[derive(Debug)]
pub struct InnovationCounter {
    counter: u32,
    connections_innovation_map: HashMap<(u32, u32), u32>
}

impl InnovationCounter {
    pub fn new(n_neurons: u32) -> InnovationCounter {
        InnovationCounter {
            counter: n_neurons,
            connections_innovation_map: HashMap::new(),
        }
    }

    pub fn get_connection_innovation(&mut self, neuron_in: u32, neuron_out: u32) -> u32 {
        match self.connections_innovation_map.get(&(neuron_in, neuron_out)) {
            Some(&innovation) => innovation,
            None => {
                let innovation = self.counter;
                self.connections_innovation_map.insert((neuron_in, neuron_out), self.counter);
                self.counter += 1;
                innovation
            }
        }
    }

    pub fn get_neuron_innovation(&mut self) -> u32 {
        let innovation = self.counter;
        self.counter += 1;
        innovation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_test() {
        let mut counter = InnovationCounter::new(0);

        let conn1 = counter.get_connection_innovation(1, 2);
        let conn2 = counter.get_connection_innovation(1, 2);
        let conn3 = counter.get_connection_innovation(2, 3);

        assert_eq!(conn1, conn2);

        assert!(conn1 < conn3);

        let neur1 = counter.get_neuron_innovation();
        let neur2 = counter.get_neuron_innovation();

        assert!(neur1 < neur2);
    }
}
