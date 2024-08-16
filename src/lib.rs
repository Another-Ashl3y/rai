#![allow(dead_code)]

use rand::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub enum Activation {
    Tanh,
    Linear
}

#[derive(Clone)]
struct Neuron {
    weights: Vec<f64>,
    bias: f64,
    activation: Activation,
}

impl Neuron {
    fn new(rng: &mut ThreadRng, input_size: usize) -> Self {
        let mut weights: Vec<f64> = Vec::new();
        for _ in 0..input_size {
            weights.push(rng.gen());
        }
        Self {
            weights,
            bias: rng.gen(),
            activation: Activation::Tanh,
        }
    }
    fn mutate(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.number_of_weights() {
            self.weights[i] += rng.gen::<f64>() * 0.001;
        }
        self.bias += rng.gen::<f64>() * 0.001;
    }
    fn load(weights: Vec<f64>, bias: f64, activation: Activation) -> Self {
        Self { weights, bias, activation }
    }
    fn save(&self) -> (Vec<f64>, f64, Activation) {
        (self.weights.clone(), self.bias, self.activation)
    }
    fn process(&self, ins: Vec<f64>) -> f64 {
        let mut total = self.bias;

        total += (0..ins.len()).into_par_iter().map(|i| {ins[i]*self.weights[i]}).sum::<f64>();

        match self.activation {
            Activation::Linear => return total,
            Activation::Tanh => return total.tan()
        }
    }
    fn alter(&mut self, index: usize, ammount: f64) {
        if index >= self.weights.len() {
            self.bias += ammount;
        }
        else {
            self.weights[index] += ammount;
        }
    }
    fn number_of_weights(&self) -> usize {
        self.weights.len()
    }
}

#[derive(Clone)]
pub struct Network {
    neurons: Vec<Vec<Neuron>>,
    score: f64,
    change_position: Vec3,
    change_ammount: f64,
}

impl Network {
    pub fn new(size: Vec<(usize, usize)>) -> Self {
        let mut rng = rand::thread_rng();
        let mut neurons: Vec<Vec<Neuron>> = Vec::new();
        for x in size {
            let mut layer: Vec<Neuron> = Vec::new();
            for _ in 0..x.0 {
                layer.push(Neuron::new(&mut rng, x.1))
            }
            neurons.push(layer);
        }
        Self { neurons, score: 0.0, change_position: Vec3::new(), change_ammount: 0.0}
    }
    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        for layer in self.neurons.iter_mut() {
            for node in layer {
                if rng.gen() {
                    node.mutate(&mut rng);
                }
            }
        }
    }
    pub fn save(&self) -> Vec<Vec<(Vec<f64>, f64, Activation)>> {
        let mut output: Vec<Vec<(Vec<f64>, f64, Activation)>> = Vec::new();

        for layer in self.neurons.clone() {
            let mut out_layer: Vec<(Vec<f64>, f64, Activation)> = Vec::new();
            for n in layer {
                out_layer.push(n.save());
            }
            output.push(out_layer);
        }

        output
    }
    pub fn load(net: Vec<Vec<(Vec<f64>, f64, Activation)>>) -> Self {
        let mut neurons: Vec<Vec<Neuron>> = Vec::new();
        for layer_settings in net {
            let mut layer: Vec<Neuron> = Vec::new();
            for setting in layer_settings {
                layer.push(Neuron::load(setting.0, setting.1, setting.2));
            }
            neurons.push(layer);
        }
        Self { neurons, score: 0.0, change_position: Vec3::new(), change_ammount: 0.0}
    }
    pub fn process(&self, ins: Vec<f64>) -> Vec<f64> {
        let mut prev = ins;

        for layer in self.neurons.clone() {
            prev = layer.par_iter().map(|n| n.process(prev.clone())).collect();
        }

        prev
    }
    pub fn go_next_setting(&mut self) {
        if self.change_position.z < self.get_neuron().number_of_weights() {
            self.change_position.z += 1;
            return;
        }
        self.change_position.z = 0;
        if self.change_position.y < self.neurons[self.change_position.x].len() -1 {
            self.change_position.y += 1;
            return;
        }
        self.change_position.y = 0;
        if self.change_position.x < self.neurons.len() -1 {
            self.change_position.x += 1;
            return;
        }
        self.change_position.x = 0;
    }
    pub fn shift(&mut self) {
        self.get_neuron().alter(self.change_position.z, -self.change_ammount)
    }
    pub fn undo_shift(&mut self) {
        self.get_neuron().alter(self.change_position.z, -self.change_ammount)
    }
    pub fn set_shift_rate(&mut self, rate: f64) {
        self.change_ammount = rate;
    }
    pub fn flip_shift(&mut self) {
        self.change_ammount *= -1.0;
    }
    pub fn reduce_rate(&mut self) {
        self.change_ammount *= 0.9998;
    }
    fn get_neuron(&self) -> Neuron {
        self.neurons[self.change_position.x][self.change_position.y].clone()
    }
}

struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}
impl Vec3 {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

