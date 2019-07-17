use config::node_config::load_node_config;
use node::node::Node;

fn main() {
    let node_config = load_node_config();
    let node_server = Node::new(node_config);
    node_server.run();
}
