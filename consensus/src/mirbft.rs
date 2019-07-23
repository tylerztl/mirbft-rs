use logger::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};
use crossbeam::crossbeam_channel::{unbounded, Receiver, RecvError, Sender};
use proto::proto::mirbft::{
    Message,
    Message_oneof_Type,
};
use crate::state_machine::StateMachine;
use crate::timer::BatchTimer;
use config::node_config::NodeConfig;

pub struct MirBft {
    msg_sender: Sender<Message>,
    msg_receiver: Receiver<Message>,
    state_machine: Arc<Mutex<StateMachine>>,
}

impl MirBft {
    pub fn new(s: Sender<Message>, r: Receiver<Message>, peer_id: usize) -> Self {
        info!("BFT State Machine Launched.");
        MirBft {
            msg_sender: s,
            msg_receiver: r,
            state_machine: Arc::new(Mutex::new(StateMachine::new(peer_id))),
        }
    }

    pub fn start(config: &NodeConfig) {
        let (bft_sender, bft_receiver) = unbounded();
        let (time_sender, time_receiver) = unbounded();
        let mut engine = Self::new(bft_sender, bft_receiver, config.service.peer_id);
        BatchTimer::start(time_sender, config.consensus.batch_timeout_ms);

        let _main_thread = thread::Builder::new()
            .name("consensus".to_string())
            .spawn(move || {
                loop {
                    let mut get_msg = Err(RecvError);
                    let mut timeout_msg = Err(RecvError);
                    select! {
                        recv(engine.msg_receiver) -> msg => get_msg = msg,
                        recv(time_receiver) -> msg => timeout_msg = msg,
                    }

                    if let Ok(msg) = get_msg {
                        engine.process(msg);
                    }
                    if let Ok(_msg) = timeout_msg {
                        engine.propose();
                    }
                }
            })
            .unwrap();
    }

    fn process(&mut self, msg: Message) {
        let msg_type = msg.Type.unwrap();
        match msg_type {
            Message_oneof_Type::proposal(proposal) => {
                let machine = self.state_machine.clone();
                machine.lock().unwrap().propose(proposal.payload);
            }
            Message_oneof_Type::preprepare(preprepare) => {}
            Message_oneof_Type::prepare(prepare) => {}
            Message_oneof_Type::commit(commit) => {}
            _ => error!("Invalid Message!"),
        }
    }

    fn propose(&self) {

    }
}
