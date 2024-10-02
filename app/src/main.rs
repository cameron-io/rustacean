use rustacean_lib::actor::start;
use rustacean_lib::actor::Worker;
use rustacean_lib::actor::ActorMessage;

fn main() -> Result<(), std::io::Error> {
    // Initialize with subscribed children
    let subscribed_actor = Worker {
        is_parent: false,
        name: "Subordinate 1",
        recipients: vec![],
        free: vec![1],
        reserved: vec![]
    };

    let res = start(vec![subscribed_actor]).unwrap();

    res.system.block_on(async {
        // start sending messages
        res.worker.do_send(ActorMessage {
            id: 1,
            action: "store this value in state"
        });
        res.worker.do_send(ActorMessage {
            id: 1,
            action: "return state"
        });
        res.worker.do_send(ActorMessage {
            id: 2,
            action: "exit"
        })
    });

    res.system.run()
}