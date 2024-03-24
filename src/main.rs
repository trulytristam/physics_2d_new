mod engine;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut engine = engine::Engine::new();
    engine.start().await;
}
