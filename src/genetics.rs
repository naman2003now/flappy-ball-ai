#[derive(Clone)]
pub struct Brain {
    pub hidden_weights: [f32; 8 * 4],
    pub output_weights: [f32; 2 * 8],
    pub hidden_bias: [f32; 8],
    pub output_bias: [f32; 2],
}

impl Brain {
    // Initilize the brain with the following values:
    // ---- Brain ----
    // Hidden weights:
    // -0.8487895 0.49137527 0.39987323 0.364316
    // 0.23451778 -0.103829905 -0.19837642 0.99858856
    // 0.62801474 -0.30081648 -1.1717306 0.7693195
    // -0.39168888 0.52182764 -0.13809662 0.67412513
    // -0.34392887 0.9412651 -0.13442256 -0.27424228
    // -0.7642416 0.74272853 1.330818 0.05658693
    // -0.49279448 0.224115 1.0401345 1.6181815
    // 0.3687311 0.3473385 0.35138813 -0.18115227
    // Output weights:
    // 0.3215181 0.71852684 0.5317758 0.67969036 -0.28395957 -0.812366 0.84331447 -0.5995793
    // -0.2996683 -0.9115532 0.23906352 -0.9690799 0.7915144 0.63064504 0.72257954 0.19844499
    // Hidden bias:
    // 0.936299 -0.35624337 0.005529036 0.79126817 0.5500305 0.4516027 -0.03398283 -0.91007674
    // Output bias:
    // -1.1206839 0.11029067
    pub fn new() -> Self {
        Self {
            hidden_weights: [
                -0.8487895,
                0.49137527,
                0.39987323,
                0.364316,
                0.23451778,
                -0.103829905,
                -0.19837642,
                0.99858856,
                0.62801474,
                -0.30081648,
                -1.1717306,
                0.7693195,
                -0.39168888,
                0.52182764,
                -0.13809662,
                0.67412513,
                -0.34392887,
                0.9412651,
                -0.13442256,
                -0.27424228,
                -0.7642416,
                0.74272853,
                1.330818,
                0.05658693,
                -0.49279448,
                0.224115,
                1.0401345,
                1.6181815,
                0.3687311,
                0.3473385,
                0.35138813,
                -0.18115227,
            ],
            output_weights: [
                0.3215181,
                0.71852684,
                0.5317758,
                0.67969036,
                -0.28395957,
                -0.812366,
                0.84331447,
                -0.5995793,
                -0.2996683,
                -0.9115532,
                0.23906352,
                -0.9690799,
                0.7915144,
                0.63064504,
                0.72257954,
                0.19844499,
            ],
            hidden_bias: [
                0.936299,
                -0.35624337,
                0.005529036,
                0.79126817,
                0.5500305,
                0.4516027,
                -0.03398283,
                -0.91007674,
            ],
            output_bias: [-1.1206839, 0.11029067],
        }
    }

    pub fn random() -> Self {
        let mut hidden_weights = [0.0; 4 * 8];
        for i in 0..4 * 8 {
            hidden_weights[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        let mut output_weights = [0.0; 2 * 8];
        for i in 0..2 * 8 {
            output_weights[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        let mut hidden_bias = [0.0; 8];
        for i in 0..8 {
            hidden_bias[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        let mut output_bias = [0.0; 2];
        for i in 0..2 {
            output_bias[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        Self {
            hidden_weights,
            output_weights,
            hidden_bias,
            output_bias,
        }
    }

    pub fn print(&self) {
        println!("\n\n---- Brain ----");
        println!("Hidden weights:");
        for i in 0..8 {
            for j in 0..4 {
                print!("{} ", self.hidden_weights[i * 4 + j]);
            }
            println!();
        }
        println!("Output weights:");
        for i in 0..2 {
            for j in 0..8 {
                print!("{} ", self.output_weights[i * 8 + j]);
            }
            println!();
        }
        println!("Hidden bias:");
        for i in 0..8 {
            print!("{} ", self.hidden_bias[i]);
        }
        println!();
        println!("Output bias:");
        for i in 0..2 {
            print!("{} ", self.output_bias[i]);
        }
    }

    pub fn generate_n_child(&self, n: usize) -> Vec<Self> {
        let mut children = Vec::new();
        for _ in 0..n {
            let mut hidden_weights = [0.0; 4 * 8];
            for i in 0..4 * 8 {
                hidden_weights[i] = self.hidden_weights[i] + rand::random::<f32>() * 0.01 - 0.005;
            }
            let mut output_weights = [0.0; 2 * 8];
            for i in 0..2 * 8 {
                output_weights[i] = self.output_weights[i] + rand::random::<f32>() * 0.01 - 0.005;
            }
            let mut hidden_bias = [0.0; 8];
            for i in 0..8 {
                hidden_bias[i] = self.hidden_bias[i] + rand::random::<f32>() * 0.01 - 0.005;
            }
            let mut output_bias = [0.0; 2];
            for i in 0..2 {
                output_bias[i] = self.output_bias[i] + rand::random::<f32>() * 0.01 - 0.005;
            }
            children.push(Self {
                hidden_weights,
                output_weights,
                hidden_bias,
                output_bias,
            });
        }
        children
    }

    pub fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn think(&self, inputs: [f32; 4]) -> [f32; 2] {
        let mut hidden_output = [0.0; 8];
        for i in 0..8 {
            hidden_output[i] = self.hidden_bias[i];
            for j in 0..4 {
                hidden_output[i] += inputs[j] * self.hidden_weights[i * 4 + j];
            }
            hidden_output[i] = Self::sigmoid(hidden_output[i]);
        }
        let mut output = [0.0; 2];
        for i in 0..2 {
            output[i] = self.output_bias[i];
            for j in 0..8 {
                output[i] += hidden_output[j] * self.output_weights[i * 8 + j];
            }
            output[i] = Self::sigmoid(output[i]);
        }
        output
    }
}
