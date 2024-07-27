import math
import pygame
import random
import sys

CALC_LIMIT = 3
MODEL_SIZE = 150

sys.setrecursionlimit(MODEL_SIZE**CALC_LIMIT)

class Node:
    def __init__(self, weights=None, bias=None, connection_nodes=None, connection_range=range(0,0)) -> None:
        self.weights = weights
        self.bias = bias
        self.connection_nodes = connection_nodes
        self.calculations = 0
        self.output = 0
        self.activation = 'R' # N T R S | Normal Tanh ReLU Step

        if self.connection_nodes == None:
            self.connection_nodes = [random.choice(connection_range) for _ in range(random.choice(connection_range))]
        if self.weights == None:
            self.weights = [random.uniform(-1,1) for _ in range(len(self.connection_nodes))]
        if self.bias == None:
            self.bias = random.uniform(-1,1)
    
    def calculate(self, network:list):
        children = [network[i] for i in self.get_child_indices()]
        if self.calculations < CALC_LIMIT:
            self.calculations += 1
            # sum(ReLu(o*w))+bias
            self.output = self.bias
            for i, w in enumerate(self.weights):
                children[i].calculate(network)
                if self.activation == 'R': # ReLu
                    self.output += max(0,children[i].output)*w
                elif self.activation == 'T':
                    self.output += math.tanh(children[i].output)*w
            # print(self.output)
    
    def reset(self):
        self.calculations = 0
    
    def get_child_indices(self):
        return self.connection_nodes

    def __repr__(self) -> str:
        return f"{self.output - self.bias}, {self.calculations}"

class Input_Node(Node):
    def __init__(self, weights=None, bias=None, connection_nodes=None, connection_range=range(0, 0)) -> None:
        self.output = 0
    def calculate(self, network: list):
        return network
    def __repr__(self) -> str:
        return f"Input: {self.output}"

def main():

    WIDTH,HEIGHT = 500,500
    win = pygame.display.set_mode((WIDTH,HEIGHT))
    win.fill((0,0,0))

    network_size = range(0,MODEL_SIZE)
    input_neuron_index = 0
    output_neuron_index = 2
    networks = [[Node(connection_range=network_size) for _ in network_size] for _2 in range(16)]


    

    points = []

    multiplyer = 100
    for net_index, neural_network in enumerate(networks):
        prev = None
        for input_index in range(-int((WIDTH/2)*multiplyer), int((WIDTH/2)*multiplyer), 1):
            print(f"{int(100*((input_index/multiplyer+WIDTH/2)/WIDTH))}% there")
            x = input_index/multiplyer
            neural_network[input_neuron_index] = Input_Node()
            neural_network[input_neuron_index].output = x

            for i, n in enumerate(neural_network): 
                if i != input_neuron_index: n.calculate(neural_network)
                for fn in neural_network:
                    fn.reset()

            # neural_network[output_neuron_index].calculate(neural_network)

            q = neural_network[output_neuron_index].output
            # print(q)
            point = (x+WIDTH/2, HEIGHT/2-q*1)
            if prev != None:
                pygame.draw.line(win, (int(255*(net_index/len(networks))),255,255), prev, point, 2+net_index)
            prev = point
            pygame.display.update()
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    run = False
            # points.append(point)


    heights = [p[1] for p in points]

    # for i, p in enumerate(points):
    #     pygame.draw.rect(win, (min(0,i*2),255,255), (p[0],HEIGHT/2-(p[1]/max(heights))*250,2,2))
        
    run = True
    while run:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                run = False
        
        pygame.display.update()

            
    
    for n in neural_network:
        print(n)



if __name__=="__main__":
    main()