import math
import pygame
import random
import time




parameters = []
test_data = [(x, (x/10)**2) for x in range(-255,255,5)]


class Parameter:
    def __init__(self, value) -> None:
        self.value = value
        self.original = value
        self.type = type(value)
    def add(self, value) -> None:
        if self.type in [int, float]:
            self.value += value
    def apply(self, value) -> None:
        if self.type in [int, float]:
            self.value += value
            self.original += value
        elif self.type == str:
            self.original = self.value
    def get_variations(self) -> list:
        if self.type in [int, float]:
            return [self.value-0.0001, self.value, self.value+0.0001]
        return []
    def set(self, value):
        self.value = value
    def default(self):
        self.value = self.original
    def finalize(self):
        self.original = self.value

class Node:
    def __init__(self, weights=None, bias=None, activation=None, connection_nodes=None, connection_range=range(0,0)) -> None:
        self.weights = weights      # These are actually indices
        # self.pre_bias = pre_bias    # Still just indices
        self.bias = bias            # This is actually an index
        self.leaky = None
        self.connection_nodes = connection_nodes
        self.output = 0
        self.activation = activation # N T R S | Normal Tanh ReLU Step

        if self.leaky == None:
            self.leaky = len(parameters)
            parameters.append(Parameter(0))#random.uniform(0,0.05)))
        if self.connection_nodes == None:
            self.connection_nodes = [random.choice(connection_range) for _ in range(random.choice(connection_range))]
        if self.weights == None:
            self.weights = range(len(parameters), len(parameters)+len(self.connection_nodes))
            for _ in self.weights:
                parameters.append(Parameter(0))#random.uniform(-1,1)))
        # if self.pre_bias == None:
        #     self.pre_bias = range(len(parameters), len(parameters)+len(self.connection_nodes))
        #     for _ in self.pre_bias:
        #         parameters.append(Parameter(random.uniform(-4,4)))
        if self.bias == None:
            self.bias = len(parameters)
            parameters.append(Parameter(-0))#random.uniform(-1,1)))
        
        if self.activation == None:
            self.activation = len(parameters)
            parameters.append(Parameter(random.choice(['R'])))#random.uniform(-1,1)))
            
    
    def calculate(self, network:list):
        # print(self.get_child_indices(), len(network))
        children = [network[i] for i in self.get_child_indices()]
        self.output = parameters[self.bias].value
        for i, w in enumerate(self.weights):
            children[i].calculate(network)
            if parameters[self.activation].value == 'R': # ReLu
                x = children[i].output*parameters[w].value
                self.output += max(max(0,self.leaky)*x,x)
            elif parameters[self.activation].value == 'T':
                self.output += math.tanh(children[i].output*parameters[w].value)
            else:
                self.output += children[i].output*parameters[w].value
        if parameters[self.activation].value == 'S':
            if self.output > 0:
                self.output = 1
            else:
                self.output = 0

    def reset(self):
        self.calculations = 0
    
    def get_child_indices(self):
        return self.connection_nodes

    def __repr__(self) -> str:
        return f"{self.output - self.bias}"

class Input_Node(Node):
    def __init__(self) -> None:
        self.output = 0
    def calculate(self, network: list):
        return network
    def reset(self):
        pass
    def __repr__(self) -> str:
        return f"Input: {self.output}"

def main():

    WIDTH,HEIGHT = 1200,800
    win = pygame.display.set_mode((WIDTH,HEIGHT))

    network_size = range(0,13)
    input_neuron_index = 0
    neural_network = [Node(connection_range=network_size,connection_nodes=[input_neuron_index]) for _ in range(6)]
    neural_network += [Node(connection_range=network_size,connection_nodes=range(len(neural_network)-6,len(neural_network))) for _ in range(8)]
    neural_network += [Node(connection_range=network_size,connection_nodes=range(len(neural_network)-8,len(neural_network))) for _ in range(20)]
    neural_network += [Node(connection_range=network_size,connection_nodes=range(len(neural_network)-20,len(neural_network))) for _ in range(1)]
    neural_network.insert(input_neuron_index, Input_Node())
    output_neuron_index = len(neural_network) - 1
    neural_network[output_neuron_index].activation = len(parameters)
    parameters.append(Parameter('R'))


    COMPLETE_DISPLAY = True
    LEARN = True
    LEARN_DISPLAY = LEARN
    CORRECTIONS_PER = 10

    total_error = -1

    multiplyer = 1 # The value for the "zoom" of the graph | higher number = slower speed on full display
    run = True
    while run:
        # Clear screen
        win.fill((0,0,0))



        # Calculate Each x and display whole length if enabled
        if COMPLETE_DISPLAY:

            # Notation parameters
            prev = None
            prev_percentage = None
            time_since = time.time()

            for input_index in range(-int((WIDTH/2)*multiplyer), int((WIDTH/2)*multiplyer), 1):

                # # Display fance progress bar
                # percentage = round(100*((input_index/multiplyer+WIDTH/2)/(WIDTH)),0)
                # if percentage != prev_percentage:
                #     percentage_bar = int(percentage+2)*"█"
                #     print(f"{percentage_bar+int(100-len(percentage_bar))*"-"} {percentage+2}% there. {round(time.time() - time_since,2)}s") 
                #     time_since = time.time()
                #     prev_percentage = percentage

                
                # Setting Inputs
                x = input_index/multiplyer
                neural_network[input_neuron_index].output = x


                # Process the model with the input data
                # for i, n in enumerate(neural_network): 
                #     if i != input_neuron_index: n.calculate(neural_network)
                # print("START")
                neural_network[output_neuron_index].calculate(neural_network)

                # Get output of the model
                q = neural_network[output_neuron_index].output
                
                # Plot output data
                point = (x+WIDTH/2, HEIGHT/2-q*1)
                if prev != None:
                    pygame.draw.line(win, (0,255,255), prev, point, 2)
                prev = point

                # Run this because of very high delay before the main loop repeats
                for event in pygame.event.get():
                    if event.type == pygame.QUIT:
                        run = False

            pygame.display.update()


        if LEARN:
            for parameter_index, parameter in enumerate(parameters):
                
                parameter.default()

                error = 0
                for data in test_data:
                    
                    neural_network[input_neuron_index].output = data[0]

                    # Process model
                    neural_network[output_neuron_index].calculate(neural_network)

                    # Get output of model
                    network_output = neural_network[output_neuron_index].output
                    
                    # Add error to the total
                    error += (data[1] - network_output)**2

                total_error = error
                
                prev_percentage_bar = None
                
                # # Repeat the corrections ammount of times
                # for correction in range(CORRECTIONS_PER):

                changing = True
                changes = 0
                while changing and changes < CORRECTIONS_PER:
                    
                    # Display fance progress bar
                    parameter_percentage = round((parameter_index/len(parameters)),2)
                    # correction_percentage = round((correction/CORRECTIONS_PER), 2)
                    parameter_percentage_bar = int(parameter_percentage*100/2)*"█"
                    # correction_percentage_bar = int(correction_percentage*100/2)*"█"
                    # percentage_bar = f"{parameter_percentage_bar + "-"*(50-len(parameter_percentage_bar))} {"-"*(50-len(correction_percentage_bar))+correction_percentage_bar}"
                    
                    percentage_bar = f"{parameter_percentage_bar + "-"*(50-len(parameter_percentage_bar))} {changes}"

                    if percentage_bar != prev_percentage_bar:
                        print(percentage_bar + f" {round(total_error,5)}")
                        prev_percentage_bar = percentage_bar

                    # Generate variations of the parameter
                    parameter.default()
                    parameter_range = parameter.get_variations()

                    errors = []
                    
                    # if LEARN_DISPLAY:
                    #     win.fill((0,0,0))

                    for variation_index, variation in enumerate(parameter_range):
                        # Set parameter to variation before calculating
                        parameters[parameter_index].set(variation)
                        error = 0
                        for data in test_data:
                            
                            neural_network[input_neuron_index].output = data[0]

                            # Process model
                            neural_network[output_neuron_index].calculate(neural_network)

                            # Get output of model
                            network_output = neural_network[output_neuron_index].output

                            # Graph output and data if varaiation is original parameter
                            # if variation == parameter:
                            if LEARN_DISPLAY and variation_index == 1:
                                # pygame.draw.ellipse(win,(int(255*(correction/CORRECTIONS_PER)),255-int(255*(parameter_index/len(parameters))),0),(data[0]+WIDTH/2,HEIGHT/2-network_output,6,6))
                                pygame.draw.ellipse(win,(int(255),255-int(255*(parameter_index/len(parameters))),0),(data[0]+WIDTH/2,HEIGHT/2-network_output,6,6))
                                pygame.draw.ellipse(win,(0,0,255),(data[0]+WIDTH/2,HEIGHT/2-data[1],6,6))
                            # Add error to the total
                            error += (data[1] - network_output)**2
                            
                            # Run this because of very high delay before the main loop repeats
                            # aka to stop the pygame window from crashing
                            for event in pygame.event.get():
                                if event.type == pygame.QUIT:
                                    run = False
                        

                        error/=len(test_data)
                        errors.append(error)
                    
                    if LEARN_DISPLAY:
                        c = 255 # int((correction/CORRECTIONS_PER)*255)
                        # pygame.draw.ellipse(win, (c,c,c),(parameters[parameter_index].value + WIDTH/2, HEIGHT/2 - errors[1], 20, 20))
                        pygame.display.update()
                    
                    if parameter.type in [float, int]:

                        # Calculate average gradient
                        gradient = ( (errors[1] - errors[0]) / (parameter_range[1] - parameter_range[0]) 
                                + (errors[2] - errors[1]) / (parameter_range[2] - parameter_range[1]))/2


                        # Calculate parameter shift
                        try:
                            movement = -gradient / abs(gradient)
                        except ZeroDivisionError:
                            # parameter.value = random.uniform(-1,1)
                            # movement = 1
                            break
                    
                    else: 
                        # print("ACtivation ")
                        break

                    parameters[parameter_index].add(movement)
                    error = 0
                    for data in test_data:
                        
                        neural_network[input_neuron_index].output = data[0]

                        # Process model
                        neural_network[output_neuron_index].calculate(neural_network)

                        # Get output of model
                        network_output = neural_network[output_neuron_index].output
                        
                        # Add error to the total
                        error += (data[1] - network_output)**2


                    if error <= total_error:
                        parameters[parameter_index].apply(movement*(changes/CORRECTIONS_PER)*1)
                        total_error = error
                        changes += 1
                        
                    else:
                        # breakpoint()
                        changing = False
                        # print(error, total_error)
                        # breakpoint()
                    
                    parameter.default()


                    

                    

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                run = False
        
        pygame.display.update()





if __name__=="__main__":
    main()