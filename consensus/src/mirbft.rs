use crate::state_machine::StateMachine;
use crate::timer::BatchTimer;
use crate::NodeID;
use config::MirConfig;
use crossbeam::crossbeam_channel::{unbounded, Receiver, RecvError};
use logger::prelude::*;
use network::GrpcClient;
use proto::proto::mirbft::{Forward, Message, Message_oneof_Type};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct MirBft {
    node_id: NodeID,
    msg_receiver: Receiver<(NodeID, Message)>,
    state_machine: Arc<Mutex<StateMachine>>,
    clients: HashMap<NodeID, Arc<Mutex<GrpcClient>>>,
}

impl MirBft {
    pub fn new(r: Receiver<(NodeID, Message)>, config: MirConfig) -> Self {
        info!("BFT State Machine Launched.");
        let mut clients = HashMap::new();
        for (key, val) in config.consensus_config.peers.iter() {
            let key = key.clone().parse::<u64>().unwrap();
            let client = Arc::new(Mutex::new(GrpcClient::new(
                key,
                val.address.clone(),
                val.port,
            )));
            clients.insert(key, client);
        }
        MirBft {
            node_id: config.node_config.service.peer_id,
            msg_receiver: r,
            state_machine: Arc::new(Mutex::new(StateMachine::new(config))),
            clients,
        }
    }

    pub fn start(r: Receiver<(NodeID, Message)>, config: MirConfig) {
        let (time_sender, time_receiver) = unbounded();
        BatchTimer::start(
            time_sender,
            config.consensus_config.consensus.batch_timeout_ms,
        );

        let engine = Self::new(r, config);

        let _main_thread = thread::Builder::new()
            .name("consensus".to_string())
            .spawn(move || loop {
                let mut get_msg = Err(RecvError);
                let mut timeout_msg = Err(RecvError);
                select! {
                    recv(engine.msg_receiver) -> msg => get_msg = msg,
                    recv(time_receiver) -> msg => timeout_msg = msg,
                }

                if let Ok(msg) = get_msg {
                    engine.process(msg.0, msg.1);
                }
                if let Ok(_msg) = timeout_msg {
                    let batch = engine.state_machine.clone().lock().unwrap().handle_batch();
                    if batch.is_some() {
                        let batch_msg = batch.unwrap();
                        engine.broadcast(&batch_msg);
                        engine.process(engine.node_id, batch_msg);
                    }
                }
            })
            .unwrap();
    }

    fn broadcast(&self, msg: &Message) {
        for (id, client) in self.clients.iter() {
            if *id == self.node_id {
                continue;
            }
            client
                .lock()
                .unwrap()
                .broadcast(self.node_id.to_string().as_str(), &msg);
        }
    }

    fn unicast(&self, node_id: NodeID, msg: &Message) {
        for (id, client) in self.clients.iter() {
            if node_id == *id {
                client
                    .lock()
                    .unwrap()
                    .broadcast(self.node_id.to_string().as_str(), &msg);
                return;
            }
        }
    }

    fn process(&self, source: NodeID, msg: Message) {
        info!("handle msg: {:?} from node: {}", msg, source);
        let msg_type = msg.Type.unwrap();
        match msg_type {
            Message_oneof_Type::proposal(proposal) => {
                let machine = self.state_machine.clone();
                let (is_ok, node_id, bucket_id) =
                    machine.lock().unwrap().propose(proposal.payload.clone());
                if !is_ok {
                    let mut new_msg = Message::new();
                    let mut forward = Forward::new();
                    forward.set_epoch(machine.lock().unwrap().current_epoch.number);
                    forward.set_bucket(bucket_id);
                    forward.set_payload(proposal.payload);
                    new_msg.set_forward(forward);
                    self.unicast(node_id, &new_msg);
                }
            }
            Message_oneof_Type::forward(forward) => {
                self.state_machine
                    .clone()
                    .lock()
                    .unwrap()
                    .propose(forward.payload);
            }
            Message_oneof_Type::preprepare(preprepare) => {
                let action = self
                    .state_machine
                    .clone()
                    .lock()
                    .unwrap()
                    .preprepare(preprepare);
                if action.is_some() {
                    let action_msg = action.unwrap();
                    self.process(self.node_id, action_msg.clone());
                    self.broadcast(&action_msg);
                }
            }
            Message_oneof_Type::prepare(prepare) => {
                self.state_machine
                    .clone()
                    .lock()
                    .unwrap()
                    .prepare(source, prepare)
                    .map(|commit| {
                        self.broadcast(&commit);
                        self.process(self.node_id, commit);
                    });
            }
            Message_oneof_Type::commit(commit) => {
                self.state_machine
                    .clone()
                    .lock()
                    .unwrap()
                    .commit(source, commit);
            }
            _ => error!("Invalid Message!"),
        }
    }
}
