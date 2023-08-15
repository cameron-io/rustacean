mod actors;

use actors::game;

fn main() {
    game::run().unwrap();
}