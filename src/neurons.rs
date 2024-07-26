use rand::{rngs::ThreadRng, thread_rng, Rng};


#[derive(Clone)]
enum Activation {
    Tanh,
    Step,
    Relu
}


#[derive(Clone)]
enum Node_Type {
    Normal(Neuron),
    Memory(Memory),
    Input(Input)
}

fn tanh(x:f64) -> f64 {
    x.tanh()
}
fn relu(x:f64) -> f64 {
    if x < 0.0 {
        return 0.0
    }
    x
}
fn step(x:f64) -> f64 {
    if x > 0.0 {
        return 1.0
    }
    0.0
}


#[derive(Clone)]
pub struct Neuron {
    activation: Activation,
    indices: Vec<usize>,
    weights: Vec<f64>,
    bias: f64,
    output: f64,
}

impl Neuron {
    pub fn new(input_range: usize, rng: &mut ThreadRng) -> Self {
        // Pick Activation
        let activation_rand: f64 = rng.gen();
        let mut activation: Activation = Activation::Relu;
        if activation_rand < 0.33 {
            activation = Activation::Tanh;
        } 
        else if activation_rand < 0.66 {
            activation = Activation::Step;
        }
        
        // Pick Indicies
        let input_range = 0..input_range;
        let input_size: usize = rng.gen_range(input_range.clone());
        let mut indices: Vec<usize> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();
        for _ in 0..input_size {
            indices.push(rng.gen_range(input_range.clone()));
            weights.push(rng.gen());
        }

        Self {
            activation,
            indices,
            weights,
            bias: rng.gen(),
            output: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Input {
    output: f64,
}
impl Input {
    pub fn new() -> Self {
        Self {output:0.0}
    }
    pub fn set_input(&mut self, value: f64) {
        self.output = value;
    }
}

#[derive(Clone)]
pub struct Memory {
    write: usize,
    read: usize,
    data: usize,
    output: f64,
}

#[derive(Clone)]
pub struct Network {
    input_size: usize,
    output_size: usize,
    neurones: Vec<Node_Type>,
}

impl Network {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let mut neurones: Vec<Node_Type> = Vec::new();
        for _ in 0..input_size {
            neurones.push(Node_Type::Input(Input::new()));
        }
        let mut rng = thread_rng();
        for _ in 0..output_size {
            let mut neuron = Neuron::new(neurones.len(), &mut rng);
            neuron.activation = Activation::Tanh;
            neurones.push(Node_Type::Normal(neuron))
        }
        Self {
            input_size,
            output_size,
            neurones,
        }
    }
    pub fn set_inputs(&mut self, inputs: Vec<f64>) -> bool {
        if self.input_size != inputs.len() {
            return false
        }
        for n in 0..inputs.len() {
            match self.neurones[n] {
                Node_Type::Input(ref mut x) => x.set_input(inputs[n]),
                _=>return false
            }
        }
        true
    }
    pub fn get_outputs(self) -> Vec<f64> {
        let mut q: Vec<f64> = Vec::new();
        for n in self.input_size..(self.input_size + self.output_size) {
            match &self.neurones[n] {
                Node_Type::Normal(x) => q.push(x.output),
                _=>()
            }
        }
        q
    }
    pub fn calculate(&mut self) {
        let cloned_neurones = self.neurones.clone();
        self.neurones.iter_mut().for_each(|x| {
            match x {
                Node_Type::Normal(ref mut x) => {},
                Node_Type::Memory(ref mut x) => {},
                _=>()
            }
        });
    }
}


