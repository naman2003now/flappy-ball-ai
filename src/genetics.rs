#[derive(Clone)]
pub struct Brain {
    pub hidden_weights: [f32; 8 * 4],
    pub output_weights: [f32; 2 * 8],
}

impl Brain {
    pub fn new() -> Self {
        let mut hidden_weights = [0.0; 4 * 8];
        for i in 0..4 * 8 {
            hidden_weights[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        let mut output_weights = [0.0; 2 * 8];
        for i in 0..2 * 8 {
            output_weights[i] = rand::random::<f32>() * 2.0 - 1.0;
        }
        Self {
            hidden_weights,
            output_weights,
        }
    }

    pub fn generate_n_child(&self, n: usize) -> Vec<Self> {
        let mut children = Vec::new();
        for _ in 0..n {
            let mut hidden_weights = [0.0; 4 * 8];
            for i in 0..4 * 8 {
                hidden_weights[i] = self.hidden_weights[i] + rand::random::<f32>() * 0.3 - 0.15;
            }
            let mut output_weights = [0.0; 2 * 8];
            for i in 0..2 * 8 {
                output_weights[i] = self.output_weights[i] + rand::random::<f32>() * 0.3 - 0.15;
            }
            children.push(Self {
                hidden_weights,
                output_weights,
            });
        }
        children
    }

    pub fn think(&self, inputs: [f32; 4]) -> [f32; 2] {
        let mut hidden = [0.0; 8];
        for i in 0..8 {
            hidden[i] = 0.0;
            for j in 0..4 {
                hidden[i] += inputs[j] * self.hidden_weights[i * 4 + j];
            }
            hidden[i] = hidden[i].tanh();
        }
        let mut output = [0.0; 2];
        for i in 0..2 {
            output[i] = 0.0;
            for j in 0..8 {
                output[i] += hidden[j] * self.output_weights[i * 8 + j];
            }
            output[i] = output[i].tanh();
        }
        output
    }
}
