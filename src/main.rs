use node::Pos;
use trans_map::TransMap;

mod node;
mod trans_map;

fn main() -> Result<(), std::io::Error> {
    let trans_map = TransMap::load("trans_map.txt")?;

    let item = trans_map.get(Pos::new(7, 3));
    match item {
        node::NodeType::Node(value) => println!("{}", value),
        node::NodeType::Wall => println!("Wall duh"),
    }

    Ok(())
}
