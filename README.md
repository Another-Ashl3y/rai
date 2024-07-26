# Rai (an ai in rust called Rai)
Rai is an arteficial intelligence that has learnt to communicate with humans. At the time of writing this it's only form of communication will be through messaging.



# Documentation
## Normal Nodes
The network has two types of nodes: a normal node and a memory node. Normal nodes contain a list of indexes that point to their input connection in the network. As the network does not consist of layers like a typical neural network the node so we do this to get around that issue.
## Memory Nodes
The network also has memory nodes which store data. They have 3 inputs: Write, Data and Read. Write and Read are indexes to a node and use the step activation so that when they are at 1 they turn on. Write sets the stored data to the Data value and Read sets the output value to the stored value. If read is not on then it outputs 0.






