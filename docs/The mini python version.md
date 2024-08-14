# The mini python version

I used python to write some quick code to help learn machine learning. This was very helpful as when  I started this project I had no idea how to teach Rai and I still don't because I want them to be free and creative although I doubt I have the computing power to be able to do this, maybe if I made a super computer out of GPU's I could but who knows how many times more powerful the human brain is than our best commercial hardware at this current time.
<br>
The method I think I will use to teach Rai is through "back propogation". Now, I'm not sure if what I actually implemented in the [v3 python script](https://github.com/Another-Ashl3y/rai/tree/master/src/tests/v3.py) is truly back propogation but I will just call it that for now until someone tells me otherwise. The way it works is by going through each weight/bias and giving them a left and right variant. Then we run these variants through our test data and calculate the error. 

The error is calculated as:

    error = (true_data - prediction_data)^2 / data_length

Doing this for the left, original and right variations we get an array containing 3 error values. We can use these to calculate a gradient which will tell us which way to move our weight or bias. The goal of this is to reach the turning point of a parabola that is our error term. The turning point is the smallest possible error at that time which is what we want.
Then we calculate an average gradient from this where parameter is our weight or bias:

    gradient = (error[1] - error[0]) / (parameter[1] - parameter[0])
             + (error[2] - error[1]) / (parameter[2] - parameter[1])

We can't just add the gradient to our parameter and be done with it as it may over shoot and create a bigger error causing it to over shoot more but in the opposite direction and start a really bad negative feedback loop. To stop this we divide it by `10^4` which is a random number I chose that will stop it from over shooting by large margins. At first I divided by `10^min( tick, 4 )` but that wasn't as useful for running through multiple neurons. After that we just add it to our parameter and move on to the the next one.

The psuedocode for this:
```
FOR EACH parameter:

    variations = [parameter - 1, parameter, parameter + 1]
    errors = []
    FOR EACH variation:

        error = 0
        FOR EACH data:
            prediction = CALCULATE(data) USING variation
            error += (true_value - prediction) ^ 2
        errors.add_item(error)

    gradient = ( (errors[1] - errors[0])/(variations[1] - variations[0])
               + (errors[2] - errors[1])/(variations[2] - variations[1]) ) / 2

    movement = tanh(gradient/10**4)
    parameter -= movement
```

This is repeated until the user stops the program or rate of error reduction reaches 0 (error reduction is calculated by the `previous_error - error` which is gathered from a full model calculation instead of a parameter variation calculation).