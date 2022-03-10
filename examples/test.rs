use nemlang::core::{NemlConfig, NemlEngine};

fn main() {
    let config = NemlConfig {
        source: "examples/demo.neml".to_string(),
        destination: Some("examples/demo.jpg".to_string()),
        output_sz: Some((1600, 1600)),
    };

    let mut engine = NemlEngine::new(config);
    engine.build();
}
