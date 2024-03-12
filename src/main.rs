use node::Pos;
use trans_map::TransMap;
use ucs::UCS;

mod node;
mod trans_map;
mod ucs;

fn main() -> Result<(), String> {
    let trans_map = TransMap::load("trans_map.txt")
        .map_err(|_| "reading input".to_owned())?;
    let mut ucs = UCS::new(trans_map, Pos::new(3, 6), Pos::new(6, 3))?;
    let res = ucs.search()?;
    res.print_res();

    Ok(())
}
