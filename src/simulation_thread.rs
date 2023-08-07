use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::{
    organisms::{organism::Organism, organism_info::OrganismInfo},
    simulation::Simulation,
};

pub struct SimulationThread {
    pub last_data: SimulationData,
    organism_info_receiver: Receiver<SimulationData>,
    simulation_request_sender: Sender<Duration>,
}

impl SimulationThread {
    pub fn new(organisms: Vec<Organism>, organism_counter: HashMap<String, u32>) -> Self {
        let organism_infos = OrganismInfo::from_organisms(&organisms);

        let (organism_info_sender, organism_info_receiver) = mpsc::channel();
        let (simulation_request_sender, simulation_request_receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut simulation = Simulation::new(organisms, organism_counter);
            while let Ok(time_step) = simulation_request_receiver.recv() {
                let simulation_data = simulation.run(time_step);
                organism_info_sender.send(simulation_data).unwrap();
            }
        });

        SimulationThread {
            last_data: SimulationData {
                organisms: organism_infos,
                organism_counter: HashMap::new(),
                time: Duration::ZERO,
                step: 0,
            },
            organism_info_receiver,
            simulation_request_sender,
        }
    }

    pub fn advance(&self, delta: Duration) {
        if delta != Duration::ZERO {
            self.simulation_request_sender.send(delta).unwrap();
        }
    }

    pub fn probe(&mut self) {
        while let Ok(data) = self.organism_info_receiver.try_recv() {
            self.last_data = data;
        }
    }
}

#[derive(Clone)]
pub struct SimulationData {
    pub organisms: Vec<OrganismInfo>,
    pub organism_counter: HashMap<String, u32>,
    pub time: Duration,
    pub step: u64,
}
