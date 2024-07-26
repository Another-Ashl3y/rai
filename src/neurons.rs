enum Activation {
    Tanh,
    Step,
    Relu
}


enum Node_Type {
    Normal(Neuron),
    Memory(Memory)
}


pub struct Neuron {
    activation: Activation,
    indices: Vec<usize>,
    weights: Vec<f64>,
    bias: f64
}


pub struct Memory {
    write: usize,
    read: usize,
    data: usize
}


pub struct Network {
    neurons: Vec<Node_Type>
}



