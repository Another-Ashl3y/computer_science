# A Level Project (Choosing The Project)

## Solving A Problem

Solving a problem involves projects that implement AI or bringing easier solutions than tools that already exist provide. 


## Game Engine

+ Path Finding
+ Collision detection
+ Pixel Art Program
+ Rendering
+ In-Built interfacing language

  ### Stakeholders

  + Game Developers

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)
  #### Initial Notes

  Making a game engine is a lot of work, however, there are lots of in depth aspects to it as the mark scheme requires. Despite having made a 3D renderer before, it's a lot of work and getting collision working in 3D would be even more work and not contribute to the time, therefore making a 2D engine is a better use of the given time. There are many 2D game engines out there so making one that stands out would be quite difficult. An engine I find quite interesting is the [falling everything engine](https://nollagames.com/fallingeverything/) which has every pixel simulated and looks *really* cool. Making an engine like this would leave lots of room for prototyping due to the immense amount of optimisation required. It would also use cellular automata which would make graphics a lot easier to create as it doesn't have to be made in other software and can be created through terrain simulation. This would combine physics and maths into a large computer programming project that can be developed after graduating and can be used by many people who are looking into the game development career. It would be a tool to make unique interesting games as free game engines don't have this feature yet.

  Making a programming language to interface with the language would also be quite a large task but making a block programming language would be easier to program and easier to learn and use by consumers. The project should be left open source however, to allow people to add their own features if the in-built language does not provide their needs. The engine should be accessible to new developers and have enough capabilities and features for seasoned developers. This would encourage more people to use the engine and also make it useable in education settings. The code should also be compiled and optimised before running the game however it might be best for the program to interpret the code so that consumers don't have to wait for it to finish compiling every time they want to test a feature.

  Collision detection is quite a key feature of a game engine. The collision detection shouldn't restrict what consumers can do and should also include options that send signals to code to be run when a certain collision happening rather than the game engine handling everything and restricting the use of the engine.
  The Pixel Art program (if the game engine is built like the [falling everything engine](https://nollagames.com/fallingeverything/)) should have colour options, animation options and physics options as the pixels could be simulated to fall, summon other pixels and stick to other pixels (or stay in place) etc. It should have an accessible UI as it is a tool that can be used in educational settings.
  
  The downsides of using the programming language Rust for this this project is that there is a lack of easy APIs for GPUs which would be a path for accelerating program speed capabilities. The upsides are: I already know the language, it is easy to maintain programs written in this language, it relies on structured code like structs and functions (no pesky mutable globals) which are needed for higher marks, it's fast speed and memory safety.

  I will be using a rust crate (library) called minifb to handle the window for the engine. It also has handles for keyboard and mouse which makes life easier when making player controls (and UI controls).


## Maze Solving Algorithm

+ Generating Maze
+ Solving Maze

  ### Stakeholders

  + Truck Drivers

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

  A maze solving algorithm is a fundamental project. A basic one with a generated maze like the ones you find on the back of children's cereal boxes doesn't have many real world uses but the algorithm used to solve it can have many. Generating the mazes only purpose would be to test the solver or to generate levels for games but no serious real world appliances.

  Making a maze solver is quite simple and has lots of space for optimisation. Take a look at the A* algorithm for example, it's much faster than other ones and is a good starting baseline for your code. Making an A* algorithm would mean you can focus on optimising other aspects of your code instead of the solving algorithm. On the contrary, a genetic algorithm like the one inspired by ant colonies can be looked at in many different routes to a faster program.

  The overall purpose of a maze solving algorithm is to generate a path finding algorithm. This can be used in game engines or path generating in satellite navigation for delivery and taxi drivers who rely on doing their job quickly.

  Writing this in rust has the only benefits of rust being rust and speed. As this is quite a small project there aren't any language technicalities that could help with development. Same goes for any other language on that matter.

  Minifb (A rust window handler) could be used for visualizing the paths and mazes.


## Nuclear Reactor Simulation

+ Basic Graphics
+ Basic Maths and Collision Checking

  ### Stakeholders

  + Physics Teachers
  + Nuclear Reactor Workers

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)
  + Rand (Rng Crate)

  #### Initial Notes

  A Nuclear Reactor Simulator would deceptively be a simple program. It's main features would be core simulation and reactor controls to stop it from having a meltdown like a child. It's main application would be in an education and research setting, for example it could be used to teach children about nuclear reactors and the risks involved. It could also be used to design safer reactors and to design programs to maintain a reactor to prevent the devastation of a meltdown.
  
  The graphics would be quite basic as it is just simple particle simulation with very basic collision detection. There would also be basic UI with sliders or something similar for the control rods. There would also be a small system for temperature tracking and maybe screen shake to alert the user of critical temperature.

## Bird Migration AI

+ Graphing
+ Machine Learning
+ Database for training data
+ Database for tracking progress

  ### Stakeholders

  + Ornithologists

  ### Tools

  + Rust (Programming Language)
  + Python (Programming Language)

  #### Initial Notes

  Machine Learning is a large subject and a very deep rabbit hole to get into. Making this project would require an understanding of machine learning, neural networks and a database to train the AI off. The database could be found online however a quick search revealed that this might not be easy to come by in a quickly useable format, this would likely mean making up data or plotting existing data into a program (such as clicking coordinates. Wouldn't have to be particularly accurate as the model would still be able to find a pattern) and then taking that data and using it to train the model. This could be used to track world problems like climate change or look into predicting the places of endangered birds that need to be taken into captivity to increase their population.
 
  A plotting program would be used to create the data from bird migration pattern images online. It could, for example, take the positions of mouse clicks and log them into a dataset. The data would have to be a 3D dataset of x, y and time and the AI would have to output a position based on a given time.
 
  Rust would be used for the actual machine learning aspect of the program. I have already written a library for this so I know how to do it which would make development a lot quicker and easier.

  Python would be used for the data representation and any linker programs of the machine learning output into a python useable format. Matplotlib (a python library for plotting graphs) would be used to show the data too as this is much easier than writing a plotting library in rust even though that wouldn't take long using minifb either.

## Ecosystem Simulation Game (Like Rain World)

+ Complex AI programming
+ Collision Checking
+ Player Controls
+ Rendering

  ### Stakeholders

  + Gamers
  + People interested in simulated Ecosystems

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)
  + Godot (A game engine to use in place of Rust and Minifb)
  + Aseprite (Pixel Art Software)

  #### Initial Notes

  An Ecosystem Simulation Game is a very complex project, especially to the level of rain world. Each creature in the game is processed both on and off screen. Each creature has to know how to catch prey, hide from predators and shelter from weather, which is a really complex program to make. There aren't many real world applications for this so the only people interested in the game would be programmers and gamers which is still quite a large market. This project would also show advanced programming skills as each creature has to be able to process efficiently and also need to have the capability to traverse terrain. It would be easier to do this in 2D. The creatures would walk around with kinematics and could be trained with their inputs using a neural network which could create an in-game genetic algorithm.
  The downsides to this program is that it is a very complex project as realistic creature activity is hard to code.

## Evolution Simulator

+ Genetic Algorithm
+ Logging Database
+ Graphing
+ Data Analysis

  ### Stakeholders

  + Biologists
  + Researchers in genetics
  + Researchers in Machine Learning

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

  This would be a very fun project (I have made a few in the past and they are fun to watch). This could be tackled in multiple ways allowing for prototyping. These include: neural networks and hard coded behaviours based off genes. Genetic algorithms are a method of training AI to become better at there task by giving them a score and passing their genes onto the next generation (it's inspired by natural selection after all) and has applications like path finding, and finding solutions to other problems.
 
  Using rust would be good as these types of algorithms take a lot of computing power so relying on a fast language is necessary. Also processing large amounts of data has risks of memory leaks so a memory safe language is very useful in this case.

  The minifb crate would be used to visualize the data like if there are creatures, their position and action could be shown on screen. However drawing to a window takes up a lot of time on top of the 100-2000 creatures being simulated so logging to a database could be a better method and then the data could be shown post-processing to speed up the process.

## DJ Software

+ Sound Card Driver
+ Audio Processing
+ Complex UI
+ Sound Board
+ Sound Effect Options

  ### Stakeholders

  + Sound Designers
  + Music Developers

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

  This would be a large project as making hardware drivers is no small task and a lot of the programming would be in making the user interface which is not what you get marks for.
  Code could be written to simulate sound but DJ's require low latency software and taking in audio, saving it and manipulating it would be quite hard.
  Also, I do not know any DJ's so testing it would be quite difficult.

## Physics Projectile Simulation

+ Physics engine
+ ~collision detection~
+ possibly 3D rendering

  ### Stakeholders

  + Physics Teachers
  + Physics Enthusiasts
  + Golphers
  + Military
  + Athletes

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

  A physics projectile simulator would be a very small project but would have many real world applications. Due to its simplicity, making it in 3D may be better for a 2 year project.
  Again rust would be used for speed.
  Minifb would be used for visualization.

## Retro Style Game
  ### Stakeholders

  + Gamers

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

  A poor project that has no real world applications and doesn't help anything other than entertainment. The game could be used to teach people key life skills, for example a cooking game, however, this should be used as a last resort as it doesn't have enough relevant depth to it.

## Disease Simulation
  ### Stakeholders
 
  + Doctors

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes
  
  Another fun project very similar to the evolution simulator. Could be used to simulate the spread of disease which can be used to test methods of restricting the spread. There would need to be two main elements: people and disease. 
  People would have features like: immunity which could be increased with vaccines or already having had the virus/disease, wearing masks, behavioural patterns near people, for example some people may stay away from others who cough a lit.
  Diseases/Viruses would have features like: symptoms such as coughing which increases spread but also runs the risk of killing the host, method of being spread such as airborne, through water and through touch.
  Database could be used to log data and graphs in python could be used to display data such as population, amount of people infected, resistance over time.

## Sudoku Solver

+ Sudoku solving algorithms
+ Sudoku visualizer (Perhaps in terminal like nudoku)

  ### Stakeholders

  + Sudoku Enthusiasts

  ### Tools
  + Rust (Programming Language)

  #### Initial Notes

  Fun project be minimal real world applications.

