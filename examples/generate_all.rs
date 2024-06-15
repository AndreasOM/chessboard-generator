use std::path::Path;

use chessboard_generator::ChessboardGenerator;

fn main() {
    let mut cg = ChessboardGenerator::default();

    cg.set_size(584);
    cg.render(); // .await
    cg.save_as_png(Path::new("chessboard.png"));

    let mut cg = ChessboardGenerator::new(&[[234, 233, 213, 255], [73, 116, 150, 255]]);

    cg.set_size(584);
    	cg.render(); // .await
    cg.save_as_png(Path::new("chessboard_blue.png"));
}
