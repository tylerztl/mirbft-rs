use crate::*;
use logger::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};
use crossbeam::crossbeam_channel::{unbounded, Receiver, RecvError, Sender};
use proto::proto::mirbft::{
    Message,
    Message_oneof_Type,
};
use crate::state_machine::StateMachine;

pub struct MirBft {
    msg_sender: Sender<Message>,
    msg_receiver: Receiver<Message>,
    state_machine: Arc<Mutex<StateMachine>>,
}

impl MirBft {
    pub fn new(s: Sender<Message>, r: Receiver<Message>, node_id: usize) -> Self {
        info!("BFT State Machine Launched.");
        MirBft {
            msg_sender: s,
            msg_receiver: r,
            state_machine: Arc::new(Mutex::new(StateMachine::new(node_id))),
        }
    }

    pub fn start(s: Sender<Message>, r: Receiver<Message>, node_id: usize) {
        let mut engine = Self::new(s, r, node_id);
        let _main_thread = thread::Builder::new()
            .name("consensus".to_string())
            .spawn(move || {
                loop {
                    let mut get_msg = Err(RecvError);
                    select! {
                        recv(engine.msg_receiver) -> msg => get_msg = msg,
                    }

                    if let Ok(msg) = get_msg {
//                        engine.msg_sender.send(msg.clone()).unwrap();
                        engine.process(msg);
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
}
