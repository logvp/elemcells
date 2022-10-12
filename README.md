# Elementary Cellular Automata Simulator
Simulates and visualizes [Elementary Cellular Automata](https://en.wikipedia.org/wiki/Elementary_cellular_automaton). Every simulation requires a rule and starting condition. The rule must be given, and the default initial condition is one living cell. The most famous of these rules is [rule 110](https://en.wikipedia.org/wiki/Rule_110) and some fun other ones to try out are 124, 90, 150.

```
                                                  █
                                                 █ █
                                                █   █
                                               █ █ █ █
                                              █       █
                                             █ █     █ █
                                            █   █   █   █
                                           █ █ █ █ █ █ █ █
                                          █               █
                                         █ █             █ █
                                        █   █           █   █
                                       █ █ █ █         █ █ █ █
                                      █       █       █       █
                                     █ █     █ █     █ █     █ █
                                    █   █   █   █   █   █   █   █
                                   █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █
                                  █                               █
                                 █ █                             █ █
                                █   █                           █   █
                               █ █ █ █                         █ █ █ █
                              █       █                       █       █
                             █ █     █ █                     █ █     █ █
                            █   █   █   █                   █   █   █   █
                           █ █ █ █ █ █ █ █                 █ █ █ █ █ █ █ █
                          █               █               █               █
                         █ █             █ █             █ █             █ █
                        █   █           █   █           █   █           █   █
                       █ █ █ █         █ █ █ █         █ █ █ █         █ █ █ █
                      █       █       █       █       █       █       █       █
                     █ █     █ █     █ █     █ █     █ █     █ █     █ █     █ █
                    █   █   █   █   █   █   █   █   █   █   █   █   █   █   █   █
                   █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █
                  █                                                               █
                 █ █                                                             █ █
                █   █                                                           █   █
               █ █ █ █                                                         █ █ █ █
              █       █                                                       █       █
             █ █     █ █                                                     █ █     █ █
            █   █   █   █                                                   █   █   █   █
           █ █ █ █ █ █ █ █                                                 █ █ █ █ █ █ █ █
          █               █                                               █               █
         █ █             █ █                                             █ █             █ █
        █   █           █   █                                           █   █           █   █
       █ █ █ █         █ █ █ █                                         █ █ █ █         █ █ █ █
      █       █       █       █                                       █       █       █       █
     █ █     █ █     █ █     █ █                                     █ █     █ █     █ █     █ █
    █   █   █   █   █   █   █   █                                   █   █   █   █   █   █   █   █
   █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █                                 █ █ █ █ █ █ █ █ █ █ █ █ █ █ █ █
```

## Quick Start
All you need is a rule to get started. Using `./elemcells 124 -i 10` produces
```
X...................................................................................................
XX..................................................................................................
XXX.................................................................................................
X.XX................................................................................................
XXXXX...............................................................................................
X...XX..............................................................................................
XX..XXX.............................................................................................
XXX.X.XX............................................................................................
X.XXXXXXX...........................................................................................
XXX.....XX..........................................................................................
```
`124` is the rule that governs this simulation. It determines each sequential generation using the previous generation as input. The `-i 10` tells it to print the first 10 iterations and then exit. Running the command without it `./elemcells 124` enters interactive mode. The command used to generate the Sierpiński Triangle above is `./elemcells 18 --width 70 --middle -i 32 -s`
## Interactive mode
When a `-i` argument is not present, the similation will be interactive. Here you can press simply press enter to advance the simulation one step, or you can specify a number and the simulation will render that many steps. To exit the simulation and terminate the program enter `quit` or `q`.
## Command Line Arguments
The rule is the only required argument. The only constraint on the order of arguments is that the rule must come before the custom state if present.
Some commands are exclusive of each other.
`./elemcells RULE [--width WIDTH] [--iterations NUM] [--display CHARACTERS | --solid] [--random | --middle | CUSTOM]`
Use `-h` or `--help` for more information.