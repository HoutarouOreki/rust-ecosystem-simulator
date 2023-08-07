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
    message_sender: Sender<SimulationThreadMessage>,
}

impl SimulationThread {
    pub fn new(
        initial_time_step: Duration,
        generation_configuration: GenerationConfiguration,
    ) -> Self {
        let (organism_info_sender, organism_info_receiver) = mpsc::channel();
        let (message_sender, message_receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut simulation = Simulation::new(&generation_configuration);
            let mut time_step = initial_time_step;
            let mut target_time = Duration::ZERO;

            loop {
                if simulation.time < target_time {
                    let simulation_data = simulation.run(time_step);
                    organism_info_sender.send(simulation_data).unwrap();
                }

                while let Ok(message) = message_receiver.try_recv() {
                    match message {
                        SimulationThreadMessage::AdvanceTo(new_target_time) => {
                            target_time = new_target_time;
                        }
                        SimulationThreadMessage::ChangeTimeStep(new_time_step) => {
                            time_step = new_time_step;
                        }
                        SimulationThreadMessage::Restart(new_generation_configuration) => {
                            simulation = Simulation::new(&new_generation_configuration);
                            target_time = Duration::ZERO;
                        }
                    }
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
            message_sender,
        }
    }

    pub fn advance(&self, target_time: Duration) {
        self.message_sender
            .send(SimulationThreadMessage::AdvanceTo(target_time))
            .unwrap();
    }

    pub fn probe(&mut self) {
        while let Ok(data) = self.organism_info_receiver.try_recv() {
            self.last_data = data;
        }
    }

    pub fn change_time_step(&self, time_step: Duration) {
        self.message_sender
            .send(SimulationThreadMessage::ChangeTimeStep(time_step))
            .unwrap();
    }

    pub fn restart(&self, species_gen_config: GenerationConfiguration) {
        self.message_sender
            .send(SimulationThreadMessage::Restart(species_gen_config))
            .unwrap();
    }
}

#[derive(Clone)]
pub struct SimulationData {
    pub organisms: Vec<OrganismInfo>,
    pub organism_counter: HashMap<String, u32>,
    pub time: Duration,
    pub step: u64,
}

enum SimulationThreadMessage {
    AdvanceTo(Duration),
    ChangeTimeStep(Duration),
    Restart(GenerationConfiguration),
}
