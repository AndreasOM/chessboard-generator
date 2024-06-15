use color_eyre::Result;
use std::path::Path;

use chessboard_generator::layers::LayerWood;
use chessboard_generator::ChessboardGenerator;

fn main() -> Result<()> {
    let mut cg = ChessboardGenerator::default();

    cg.set_size(584);
    cg.render(); // .await
    cg.save_as_png(Path::new("chessboard.png"));

    let mut cg = ChessboardGenerator::new(&[[234, 233, 213, 255], [73, 116, 150, 255]]);
    cg.set_size(584);
    // overlay another image on top
    let legend_buffer = include_bytes!("legend.png");
    let legend = image::load_from_memory(legend_buffer).expect("Can load legend from memory");
    cg.add_overlay_image(legend)?;
    cg.render(); // .await
    cg.save_as_png(Path::new("chessboard_blue.png"));

    let mut cg = ChessboardGenerator::new(&[[133, 87, 35, 255], [70, 40, 16, 255]]);
    cg.set_size(584);
    cg.add_layer(Box::new(LayerWood::default()));
    cg.render(); // .await
    cg.save_as_png(Path::new("chessboard_wood.png"));

    Ok(())
}
