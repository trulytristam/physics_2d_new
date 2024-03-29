mod engine;
mod test;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut engine = engine::Engine::new();

    engine.start().await;
}
