# RustyEcology
This is a program written in Rust using the Bevy game engine to simulate a predator and prey relationship in a given environment. 

<img width="576" height="576" alt="image" src="https://github.com/user-attachments/assets/92fd3902-6499-49d1-9742-083250a4fe30" />


# Inspired Prompt
**Ecosystem Simulation**: Simulate an ecosystem with different species of animals and plants interacting according to simple rules. This could include elements like predation, reproduction, and seasonal cycles.

# Purpose
The purpose of this work was to create a simulator to explore the evolution of species-wide traits in a controller environment. We wish to see the relationship between a predator species and a prey species over a random environment. Simulations such as these may provide insight as to the relationship between traits such as a species size, speed, strength, etc. Overall, the goal is to build a foundation for a simulator that allows researchers 

# Legend
There are three classes of actors (circles) identified by colors:
  1. Predator (light blue: hungry predator, medium blue: predator looking to reproduce, dark blue: dead predator)
  2. Prey (light pink: hungry prey, medium pink: prey looking to reproduce, dark pink: dead prey)
  3. Plants (red: vegetation for sustenance, dark red: rotting vegies)

# Simulator Loop
The simulator loop is split into actions. First, actors must establish their movements (assuming they can move) per cycle based on stats such as speed and strength, and neighbors around me. The second section modifies the actor's states based upon where they ended up in the environment and who is around. 

This split between movements and states was established for the sake of future-proofing the simulator. While this feature is not yet implemented, the splitting of the loop between movements and states allows for end users to specify their own rules with a given grammar. For state changes (ie: dying, eating, and reproducing) rules are simple to write for classifications based upon what actors are around and the environment itself.

Separating the movement from the state changes also allows for better simulation as is seen in the case of a predator chasing prey. In this instance, prey may be faster, but depending on the control algorithm the prey uses, it may cause the prey to run directly into another predator. By separating the 'running' from the 'hunting' aspects, we allow the prey to use its full lmovement and speed to try to outrun multiple predators instead of being hunted the moment it's close to a predator.

The sequence for the loop goes as follows:

  Hungry Predators search for any prey (preference to already dead prey).
  Hungry predator starts chasing one prey.
  Responsive prey starts running away. 
  This is repeated for all hungry predators.

  All hungry prey then move towards the closest food source if they can.

  The hungry predators kill any living prey that is directly next to it after everyone has moved.

  The hungry predators eat any prey they hunted.
  The hungry prey eat any plants they found.

  The non-hungry predators try to reproduce.
  The non-hungry prey try to reproduce.
  The plants sprout up randomly on the terrain depending on how green the tile is.

  All hunters, plants, and prey have a chance to die of natural causes given their age.

# Interesting Outcomes

As discovered, the balance between individual stats and probabilities of an event occuring (such as a predator suddenly reproducing), is slightly finicky. A very common outcome is like that seen in the image below. 

<img width="840" height="390" alt="image" src="https://github.com/user-attachments/assets/e50ec120-8340-490a-b074-111bb1973c49" />

The left image is from Cycle 35, where the population of predators is starting to grow, but a majority of them are still adequately fed. The right image is from 12 cycles later. The prey has been virtually eliminated, leaving the predators in large numbers and hungry. 

Further running of the simulation results in an overpopulation of plant life due to the fertilized ground.

# Reflection
- **General**: How far did you get with the implementation? Did you choose a
  project suitable for the time scope? What were the main obstacles and how did
  you, or would you, overcome them? What would the next steps be?

Overall, I'm pretty happy with the progress I made. I would have liked to include a few more features, but the program more than shows it's ability to simulate a (albeit, relatively simple) predator/prey dynamic. Considering my background, the theory wasn't too difficult, but I did struggle with learning Bevy. A lot of the tutorials and documents I used as resources were outdated (due to Bevy being in its infancy stage and changing frequently). The main difficulty was finding the information. Fortunately, there are documents for Rust and Bevy, I just had to dig a little deeper to find what I was searching for. 
  
- **Technical**: Looking specifically at Bevy, what were you impressions of it?
  How easy was it to learn? What are it's main benefits and drawbacks?

  Overall, my impressions of Bevy were good. I appreciate the tools that Bevy provides while also adding minimal clutter. As far as ease of learning, see my previous response. It was difficult to find up-to-date tutorials and resources online (given the projects age and a lack of a widespread community such as that related to something like Python or Unreal Engine). The main benefits seem to be the parallelism that comes. One of my first testing projects was just exploring how systems in bevy execute, and synchronization is definitely something to keep in mind when using this. Though, to be fair, parallelism is always important in computers, and especially in high-volume simulations.

- **Documentation**: During the technical interview we will expect you to
  explain the code so don't worry about adding more documentation than you
  normally would.

  Cool! I just added this for my own reference!
