use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::{
    configurations::generation_configuration::GenerationConfiguration,
    organisms::organism_info::OrganismInfo, simulation::Simulation,
};

pub struct SimulationThread {
    pub last_data: SimulationData,
    organism_info_receiver: Receiver<SimulationData>,
    requested_time_sender: Sender<Duration>,
    time_step_sender: Sender<Duration>,
}

impl SimulationThread {
    pub fn new(
        initial_time_step: Duration,
        generation_configuration: GenerationConfiguration,
    ) -> Self {
        let (organism_info_sender, organism_info_receiver) = mpsc::channel();
        let (requested_time_sender, requested_time_receiver) = mpsc::channel();
        let (time_step_sender, time_step_receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut simulation = Simulation::new(&generation_configuration);
            let mut time_step = initial_time_step;
            while let Ok(requested_time) = requested_time_receiver.recv() {
                while simulation.time < requested_time {
                    while let Ok(changed_time_step) = time_step_receiver.try_recv() {
                        time_step = changed_time_step;
                    }
                    let simulation_data = simulation.run(time_step);
                    organism_info_sender.send(simulation_data).unwrap();
                }
            }
        });

        SimulationThread {
            last_data: SimulationData {
                organisms: Vec::new(),
                organism_counter: HashMap::new(),
                time: Duration::ZERO,
                step: 0,
            },
            organism_info_receiver,
            requested_time_sender,
            time_step_sender,
        }
    }

    pub fn advance(&self, target_time: Duration) {
        self.requested_time_sender.send(target_time).unwrap();
    }

    pub fn probe(&mut self) {
        while let Ok(data) = self.organism_info_receiver.try_recv() {
            self.last_data = data;
        }
    }

    pub fn change_time_step(&self, time_step: Duration) {
        self.time_step_sender.send(time_step).unwrap();
    }
}

#[derive(Clone)]
pub struct SimulationData {
    pub organisms: Vec<OrganismInfo>,
    pub organism_counter: HashMap<String, u32>,
    pub time: Duration,
    pub step: u64,
}
