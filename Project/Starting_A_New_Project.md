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
  + Aseprite (Pixel Art Software)

  #### Initial Notes

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

## Physics Projectile Simulation

  ### Stakeholders

  + Physics Teachers
  + Physics Enthusiasts

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

## Retro Style Game
  ### Tools
  + Rust (Programming Language)
  + Minifb (Window Handling Crate)
## Disease Simulation
  ### Stakeholders
 
  + Doctors

  ### Tools

  + Rust (Programming Language)
  + Minifb (Window Handling Crate)

  #### Initial Notes

## Sudoku Solver
  ### Stakeholders

  + Sudoku Enthusiasts

  ### Tools
  + Rust (Programming Language)

  #### Initial Notes

