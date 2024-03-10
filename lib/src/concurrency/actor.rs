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
use std::time::Duration;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
struct ActorMessage {
    pub id: usize,
    pub action: &'static str
}

// Actor definition
pub struct Worker {
    // base state
    name: String,
    is_parent: bool,
    recipient: Recipient<ActorMessage>,
    // store
    free: Vec<usize>,
    reserved: Vec<HashMap<usize, Addr<Worker>>>
}
impl Actor for Worker {
    type Context = Context<Worker>;
}

// simple message handler for message
impl Handler<ActorMessage> for Worker {
    type Result = ();

    fn handle(&mut self, msg: ActorMessage, ctx: &mut Context<Self>) {
        println!("[{0}] ActorMessage received {1}, {2}", self.name, msg.id, msg.action);
        println!("State\n  Free: {0},\n  Reserved: {1}", self.free.len(), self.reserved.len());

        if msg.action == "exit" {
            System::current().stop();
        }

        // notify subscribed child actors:
        if self.is_parent {
            ctx.run_later(Duration::new(0, 100), move |act, _ctx| {
                act.recipient.do_send(ActorMessage {
                    id: msg.id + 1,
                    action: msg.action
                });
            });
        }
    }
}

pub struct NewSystem {
    worker: Addr<Worker>,
    system: SystemRunner
}

pub fn start() -> Result<NewSystem, Error> {
    // create new distributed async runtime
    let system = System::new();

    // block for actor creation
    let addr = system.block_on(async {
        Worker::create(|ctx| {
            // get parent actor address
            let addr = ctx.address();

            // create the child actor
            let addr2 = Worker {
                is_parent: false,
                name: String::from("Worker 2"),
                recipient: addr.recipient(),
                free: vec![1,2,3],
                reserved: Vec::new()
            }
            .start();

            // finalise the parent actor
            Worker {
                is_parent: true,
                name: String::from("Worker 1"),
                recipient: addr2.recipient(),
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
    use crate::concurrency::actor::start;
    use crate::concurrency::actor::ActorMessage;

    #[test]
    fn actor_test() -> Result<(), std::io::Error> {
        let res = start().unwrap();

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
