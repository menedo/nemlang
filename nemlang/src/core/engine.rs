use nemlc::engine::engine::Engine;
use poirot::raster::ComCanvas;

#[derive(Debug)]
pub struct NemlEngine {
    pub eng: Engine,
    pub config: NemlConfig,
}

#[derive(Debug)]
pub struct NemlConfig {
    pub source: String,
    pub destination: Option<String>,
    pub output_sz: Option<(i32, i32)>,
}

impl NemlEngine {
    pub fn new(nc: NemlConfig) -> Self {
        NemlEngine {
            eng: Engine::init(),
            config: nc,
        }
    }

    pub fn build(&mut self) -> bool {
        let config = Engine::init_config(self.config.source.clone());
        let data_arch = self.eng.compile(config);

        match data_arch {
            Ok(arch) => {
                let mut root = arch.create_object_tree();

                let mut output_name = "./nemlang.output.img".to_string();
                match &self.config.destination {
                    Some(d) => {
                        output_name = d.clone();
                    }
                    None => {
                        return true;
                    }
                }

                let mut img_sz = (1600, 1600);
                match self.config.output_sz {
                    Some(sz) => {
                        img_sz = sz;
                    }
                    None => {
                        return true;
                    }
                }

                let mut cc = ComCanvas::new(output_name, img_sz, None);
                let h = root.calc_box_height();

                let w0 = 100;
                let h0 = 100 + h / 2;
                root.draw_start(&mut cc, w0, h0);

                cc.draw_save();
            }
            Err(_) => {
                return false;
            }
        }

        return true;
    }
}
