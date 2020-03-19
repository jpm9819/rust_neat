extern crate rand;

mod connection_gene;
mod neuron_gene;
pub mod genome_config;

use std::slice::Iter;
use genome_config::GenomeConfig;
use std::ops::{ Deref, DerefMut };
use std::cmp::Ordering;
use connection_gene::ConnectionGene;
use neuron_gene::NeuronGene;
use crate::utils::HashVec;
use super::InnovationCounter;
use std::cell::RefCell;
use std::rc::Rc;
use rand::{ thread_rng, Rng, seq::index::sample, rngs::ThreadRng };

pub struct Genome {
    counter: Rc<RefCell<InnovationCounter>>,
    connections: HashVec<u32, ComparableGeneInterface<ConnectionGene>>,
    neurons: HashVec<u32, ComparableGeneInterface<NeuronGene>>,
    config: Rc<RefCell<GenomeConfig>>
}

impl Genome {
    pub fn new(counter: Rc<RefCell<InnovationCounter>>, config_cell: Rc<RefCell<GenomeConfig>>) -> Genome {
        let mut neurons: HashVec<u32, ComparableGeneInterface<NeuronGene>> = HashVec::new();
        let mut connections: HashVec<u32, ComparableGeneInterface<ConnectionGene>> = HashVec::new();
        
        {
            let config = config_cell.borrow();

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
        }

        Genome {
            counter,
            connections,
            neurons,
            config: config_cell,
        }
    }

    pub fn crossover(_gen1: &Genome, _gen2: &Genome) -> Genome {
        unimplemented!();
    }

    pub fn distance(&self, _gen2: &Genome) -> f64 {
        unimplemented!();
    }

    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        let value: f64 = rng.gen();
        let (mcc, mcn, muw, msw, mtc) = {
            let config: &GenomeConfig = &self.config.borrow();
            (config.get_mutate_create_connection(), config.get_mutate_create_neuron(), config.get_mutate_update_weight(), config.get_mutate_set_weight(), config.get_mutate_create_connection())
        };
        
        if value > mcc {
            self.mutate_create_connection(&mut rng);
        }
        if value > mcn {
            self.mutate_create_neuron(&mut rng);
        }
        if value > muw {
            self.mutate_update_weight(&mut rng);
        }
        if value > msw {
            self.mutate_set_weight(&mut rng);
        }
        if value > mtc {
            self.mutate_toggle_connection(&mut rng);
        }
    }

    fn mutate_create_neuron(&mut self, rng: &mut ThreadRng) {
        let sample = sample(rng, self.connections.len(), 1);
        let mut iter_connections_index = sample.iter();
        let mut counter = self.counter.borrow_mut();
        
        let index_connection = match iter_connections_index.next() {
            Some(index) => index,
            None => return,
        };

        #[allow(unused_mut)]
        let mut old_connection = &mut self.connections[index_connection];
        old_connection.toggle_enabled();

        let neuron_in_innovation = old_connection.get_neuron_in();
        let neuron_out_innovation = old_connection.get_neuron_in();

        let neuron_in: &NeuronGene = self.neurons.get(neuron_in_innovation).unwrap();
        let neuron_out: &NeuronGene = self.neurons.get(neuron_out_innovation).unwrap();

        let neuron = NeuronGene::new(
            counter.get_neuron_innovation(),
            (neuron_in.get_class() + neuron_out.get_class())/2
        );
        let new_in_connection = ConnectionGene::new(
            counter.get_connection_innovation(neuron_in_innovation, neuron.get_innovation_number()),
            neuron_in_innovation,
            neuron.get_innovation_number(),
            1.0
        );
        let new_out_connection = ConnectionGene::new(
            counter.get_connection_innovation(neuron.get_innovation_number(), neuron_out_innovation),
            neuron.get_innovation_number(),
            neuron_out_innovation,
            self.connections[index_connection].get_weight()    
        );

        self.neurons.insert_ordered(
            neuron.get_innovation_number(), 
            ComparableGeneInterface(neuron)
        );
        self.connections.insert_ordered(
            new_in_connection.get_innovation_number(), 
            ComparableGeneInterface(new_in_connection)
        );
        self.connections.insert_ordered(
            new_out_connection.get_innovation_number(),
            ComparableGeneInterface(new_out_connection)
        );
    }

    fn mutate_update_weight(&mut self, rng: &mut ThreadRng) {
        let index = match sample(rng, self.connections.len(), 1).iter().next() {
            Some(index) => index,
            None => return
        };

        let new_weight = self.connections[index].get_weight()*0.8 + 0.2 * (self.connections[index].get_weight() + 0.1) * rng.gen::<f64>();
        self.connections[index].set_weight(new_weight);
    }

    fn mutate_set_weight(&mut self, rng: &mut ThreadRng) {
        let index = if let Some(index) = sample(rng, self.connections.len(), 1).iter().next() {
            index
        } else {
            return;
        };
        
        let config = self.config.borrow();

        self.connections[index].set_weight(config.get_weight());
    }

    fn mutate_toggle_connection(&mut self, rng: &mut ThreadRng) {
        let index = if let Some(index) = sample(rng, self.connections.len(), 1).iter().next() {
            index
        } else {
            return;
        };

        self.connections[index].toggle_enabled();
    }

    fn mutate_create_connection(&mut self, rng: &mut ThreadRng) {
        for _ in 0..100 {    
            let sample_rng = sample(rng, self.neurons.len(), 2);
            let mut sample_iter = sample_rng.iter();

            let neur1 = match sample_iter.next() {
                Some(index) => index,
                None => return
            };

            let connection = loop {
                let neur2 = match sample_iter.next() {
                    Some(index) => index,
                    None => return
                };
                match self.neurons[neur1].get_class().cmp(&self.neurons[neur2].get_class()) {
                    Ordering::Equal => continue,
                    Ordering::Less => {
                        let mut counter = self.counter.borrow_mut();
                        let config = self.config.borrow();
                        break ConnectionGene::new(counter.get_connection_innovation(neur1 as u32, neur2 as u32), neur1 as u32, neur2 as u32, config.get_weight());
                    },
                    Ordering::Greater => {
                        let mut counter = self.counter.borrow_mut();
                        let config = self.config.borrow();
                        break ConnectionGene::new(counter.get_connection_innovation(neur2 as u32, neur1 as u32), neur2 as u32, neur1 as u32, config.get_weight());
                    }
                };
            };
            self.connections.insert_ordered(connection.get_innovation_number(), ComparableGeneInterface(connection));
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

impl<T: Gene> DerefMut for ComparableGeneInterface<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let config = Rc::new(RefCell::new(GenomeConfig::new(2, 2)));
        let counter = Rc::new(RefCell::new(InnovationCounter::new(4)));
        let genome = Genome::new(Rc::clone(&counter), Rc::clone(&config));

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

        {
            config.borrow_mut().set_is_connected(true);
        }

        let genome = Genome::new(Rc::clone(&counter), Rc::clone(&config));

        assert_eq!(genome.iter_connections().count(), 4);
    }
}