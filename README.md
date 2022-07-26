# [100 Prisoners Problem](https://en.wikipedia.org/wiki/100_prisoners_problem)
This code was prompted by Derek Muller's [video on the topic](https://www.youtube.com/watch?v=iSNsgj1OCLA) on his youtube channel [Veritassium](https://www.youtube.com/c/veritasium/featured).

This is an implementation of the scenario and algorythm described in his video with configurable constants for the number of trial participants and the number of trials run before estimating the success rate. 

# TODO
The findings from this experiment are significantly larger than the mathematically calculated chance of success. The `rand::shuffle` method may be the cause of this indescepency... maybe I'll look into that.
