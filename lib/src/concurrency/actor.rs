use actix::prelude::Actor;
use actix::prelude::Message;
use actix::prelude::Recipient;
use actix::prelude::Context;
use actix::prelude::Handler;
use actix::prelude::System;
use actix::prelude::Addr;
use actix::prelude::AsyncContext;
use actix::SystemRunner;
use std::io::Error;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ActorMessage {
    pub id: usize,
    pub action: &'static str
}

// Actor definition
pub struct Worker {
    // base state
    pub name: String,
    pub is_parent: bool,
    pub recipients: Vec<Recipient<ActorMessage>>,
    // store
    pub free: Vec<usize>,
    pub reserved: Vec<HashMap<usize, Addr<Worker>>>
}
impl Actor for Worker {
    type Context = Context<Worker>;
}

// simple message handler for message
impl Handler<ActorMessage> for Worker {
    type Result = ();

    fn handle(&mut self, msg: ActorMessage, _ctx: &mut Context<Self>) {
        println!("[{0}] ActorMessage received {1}, {2}", self.name, msg.id, msg.action);
        println!("State\n  Free: {0},\n  Reserved: {1}", self.free.len(), self.reserved.len());
        // notify subscribed child actors:
        if self.is_parent && self.recipients.len() > 0 {
            println!("notifying subscribed subordinates...");
            for r in self.recipients.clone() {
                r.do_send(ActorMessage {
                    id: msg.id + 1,
                    action: msg.action
                });
            }
        }
        if msg.action == "exit" {
            println!("exiting worker: {0}...", self.name);
            System::current().stop();
        }
    }
}

pub struct NewSystem {
    pub worker: Addr<Worker>,
    pub system: SystemRunner
}

pub fn start(subscribed_actors: Vec<Worker>) -> Result<NewSystem, Error> {
    // create new distributed async runtime
    let system = System::new();

    // block for actor creation
    let addr = system.block_on(async {
        Worker::create(|ctx| {
            // get parent actor address
            let _addr = ctx.address();

            // create the subscribed child actors
            let mut children = Vec::new();
            for w in subscribed_actors {
                children.push(w.start().into());
            }

            // finalise the parent actor
            Worker {
                is_parent: true,
                name: String::from("Worker 1"),
                recipients: children,
                free: vec![1,2,3],
                reserved: Vec::new()
            }
        })
    });
    Ok(NewSystem {
        worker: addr,
        system: system
    }, )
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::concurrency::actor::start;
    use crate::concurrency::actor::ActorMessage;

    use super::Worker;

    #[test]
    fn actor_with_children_test() -> Result<(), std::io::Error> {
        // Initialize with subscribed children
        let subscribed_actor = Worker {
            is_parent: false,
            name: String::from("Subordinate 1"),
            recipients: vec![],
            free: vec![1],
            reserved: vec![]
        };

        let res = start(vec![subscribed_actor]).unwrap();

        res.system.block_on(async {
            // start sending messages
            res.worker.do_send(ActorMessage {
                id: 1,
                action: "some action"
            });
            res.worker.do_send(ActorMessage {
                id: 2,
                action: "exit"
            })
        });

        res.system.run()
    }
}
