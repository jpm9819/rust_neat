mod connection_gene;
mod neuron_gene;
pub mod genome_config;

use std::slice::Iter;
use genome_config::GenomeConfig;
use std::ops::Deref;
use std::cmp::Ordering;
use connection_gene::ConnectionGene;
use neuron_gene::NeuronGene;
use crate::utils::HashVec;
use super::InnovationCounter;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Genome {
    counter: Rc<RefCell<InnovationCounter>>,
    connections: HashVec<u32, ComparableGeneInterface<ConnectionGene>>,
    neurons: HashVec<u32, ComparableGeneInterface<NeuronGene>>,
    config: GenomeConfig
}

impl Genome {
    pub fn new(counter: Rc<RefCell<InnovationCounter>>, config: GenomeConfig) -> Genome {
        let mut neurons: HashVec<u32, ComparableGeneInterface<NeuronGene>> = HashVec::new();
        let mut connections: HashVec<u32, ComparableGeneInterface<ConnectionGene>> = HashVec::new();
        
        let mut i = 0;

        while i < config.get_n_sensor() {
            let neuron = NeuronGene::new(i, neuron_gene::SENSOR);
            neurons.insert_ordered(i, ComparableGeneInterface(neuron));
            i += 1;
        }
        while i < config.get_n_output() + config.get_n_sensor() {
            let neuron = NeuronGene::new(i, neuron_gene::OUTPUT);
            neurons.insert_ordered(i, ComparableGeneInterface(neuron));
            i += 1
        }


        if config.is_connected() {
            for i in 0..config.get_n_sensor() {
                for k in config.get_n_sensor()..(config.get_n_output() + config.get_n_sensor()) {
                    let mut counter_mut = counter.borrow_mut();
                    let innovation = counter_mut.get_connection_innovation(i, k);
                    let connection = ConnectionGene::new(innovation, i, k, config.get_weight());

                    connections.insert_ordered(innovation, ComparableGeneInterface(connection));
                }
            }
        }

        Genome {
            counter,
            connections,
            neurons,
            config,
        }
    }

    pub fn iter_connections(&self) -> Iter<ComparableGeneInterface<ConnectionGene>> {
        self.connections.iter()
    }

    pub fn iter_neurons(&self) -> Iter<ComparableGeneInterface<NeuronGene>> {
        self.neurons.iter()
    }
}

pub trait Gene {
    fn get_innovation_number(&self) -> u32;
}

pub struct ComparableGeneInterface<T>(T)
    where T: Gene;

impl<T: Gene> Deref for ComparableGeneInterface<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Gene> PartialEq for ComparableGeneInterface<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_innovation_number() == other.0.get_innovation_number()
    }
}

impl<T: Gene> Eq for ComparableGeneInterface<T> {}

impl<T: Gene> PartialOrd for ComparableGeneInterface<T> {
    fn partial_cmp(&self, other: &ComparableGeneInterface<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Gene> Ord for ComparableGeneInterface<T> {
    fn cmp(&self, other: &ComparableGeneInterface<T>) -> Ordering {
        self.0.get_innovation_number().cmp(&other.0.get_innovation_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Dummy(u32);

    impl Gene for Dummy {
        fn get_innovation_number(&self) -> u32 {
            self.0
        }
    }

    #[test]
    fn test_comparable_gene_interface() {
        let a = Dummy(2);
        let b = Dummy(3);

        assert!(ComparableGeneInterface(a) < ComparableGeneInterface(b));
        assert!(ComparableGeneInterface(a) == ComparableGeneInterface(a));
        assert!(ComparableGeneInterface(a) <= ComparableGeneInterface(a));
        assert!(ComparableGeneInterface(a) <= ComparableGeneInterface(b));
    }

    #[test]
    fn test_genome_new() {
        let config = GenomeConfig::new(2, 2);
        let counter = Rc::new(RefCell::new(InnovationCounter::new(4)));
        let genome = Genome::new(Rc::clone(&counter), config);

        let control = &[0, 1, 2, 3];

        for (neuron, neuron_control) in genome.iter_neurons().zip(control.iter()) {
            assert!(neuron.get_innovation_number().eq(neuron_control))
        }

        assert_eq!(genome.iter_neurons().count(), 4);

        let control = &[4, 5, 6, 7];

        for(connection, connection_control) in genome.iter_connections().zip(control.iter()) {
            assert!(connection.get_innovation_number().eq(connection_control));
        }

        assert_eq!(genome.iter_connections().count(), 0);

        let mut config = GenomeConfig::new(2, 2);
        config.set_is_connected(true);
        let genome = Genome::new(Rc::clone(&counter), config);

        assert_eq!(genome.iter_connections().count(), 4);
    }
}