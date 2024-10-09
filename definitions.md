# Systems Architecture

### CPU

The CPU (Central processing unit) is the "brain" of the computer that processes all instructions and is responsible for keeping the computer running. It holds all the registers: PC, IR, CIR, MAR, MDR and the ACC. It also holds the ALU and the L1 Cache. It is connected to the control, address and data bus which connect to other hardware on the system like RAM, ROM, GPUs and Secondary Storage.

### RAM

Random-Access Memory is a volatile primary storage that stores in use programs and their data. This includes the operating system. It is the main storage that is used and is much faster than secondary storage and is also solid state which makes it more durable.

### ROM

Read-Only Memory is a type of storage device that can only be read from. It does not lose its data when without power and typically stores the Basic input/output system (BIOS) and Power-on Self-Test (POST). It can also be used to store all the instructions in embedded systems as they don't need to have instructions stored in data memory because programs generally aren't being written on

### BIOS

For the bios: Look on [wikipedia](https://en.wikipedia.org/wiki/BIOS).

### ALU

The arithmatic Logic Unit (ALU) performs all mathematic and logic operations like addition, subtraction, AND, OR, NOT and shifts. The output is immediatley stored in the ACC

### PC

The program counter (PC) stores the address of the next instruction.

### CIR

The current instruction register (CIR) stores the current instruction (wow, I know).

### MAR

The memory address register (MAR) stores the address of the data being accessed.

### MDR

The memory data register (MDR) stores the data of the address being read from.

### ACC

The accumulator (ACC) stores the immediate output of the ALU.

### CU

The control unit (CU) stores the clock and manages how data is moved on the CPU.

### Control Bus

The control bus is an n-direction and connects the CPU to other hardware components to allow them to interact.

### Address Bus

The address bus is uni-directional and tells memory which address to access.

### Data Bus

The data bus is bi-directional and it holds the data being sent between the CPU and memory.

### Cache

Stores output of recently used instructions to prevent repeated re-calculation of long instructions.

### Pipelining

One instructions processing is started before the rest are finished. They get pushed through a series of processing that is placed in steps.

### Cores

A core in a CPU is a CPU inside the main CPU (*like CPU-ception*) and having multiple cores can speed up processing because instructions can be assigned to different cores when other cores are being used (processing other instructions).

### RISC

Reduced instruction set computer (RISC) is a type of computer with a CPU with instructions that usually take 1 cycle. They are used in GPUs and embedded systems as RISC CPUs use less power. The drawback of this is they cannot be used for large instructions.

### CISC

Complex instruction set computer (CISC) is a type of computer with a CPU with instructions that can take *on average* 4 cycles to execute. These are usually instructions that write to memory. CISC CPUs consume more power however they allow for much more advanced computers like desktop general purpose computers.

### GPU

The graphics processing unit (GPU) is used for number crunching such as ray tracing, machine learning and crypto mining. They are comprised of thousands of cores with RISC CPUs. They cannot be used to write to memory however they are **very** fast at mathematic and logical operations.

### Embedded Systems

Embedded systems are circuit boards made for a specific purpose. They are designed to be very efficient at this job and are used in anything from a dishwasher to data sampling on space rovers. GPUs and CPUs are technically embedded systems too however the system they are used for like general purpose computers is not an embedded system as it has thousands of different uses.

# Input, Output and Storage Devices

### OMR

Optical mark reader (OMR) is an input device to detect markings different to the expected data to represent a 1 or a 0. They are used in lottery tickets and old school registers.

### OCR

Optical character recognition (OCR) is a way of extracting words from physical text.

### MICR

Magnetic ink reader (MICR) reads a special type of ink and converts it into useable data on a system. They were used to read bank checks but have become obsolete with more digitalized versions taking its place.

### Input Devices

An input device provides data to a system ranging from keyboards, mice and microphones to accelerometers used to work out phone orientation.

### Output Devices

An output device takes data from a system and turns it into a useful format like colour information on a computer being displayed on a monitor or sound data being sent through a speaker.

### Magnetic

Magnetic drives (such as magnetic tape and hard disk drives) are stored on a magnetic medium which can have areas charged and discharged to be magnetic or not storing 1s and 0s. This means they have unlimited read/writes. Read/write devices contain moving parts as the medium has to spin to the write position to read the correct data, however this makes them slower than SSDs despite the Read/Write devices having very high RPMs.

### Optical

Optical drives use disks to store data which are read (and sometimes but **not always** written to) by lazers. Information is stored in the rising/falling edges and flats of the disk. A change in the edge can mean a 1 and no change can mean a 0. Data is written by a lazer burning the disk and creating these edges, to remove an edge a special paste has to be applied to repair it.

### Solid State

Solid state drives trap electrons in a gate even without power. They are much faster than other types of read/write non-volatile storage as their are no moving parts. This makes them more durable to external sources and therefore great for portable devices however they have a limited amount of writes. They are also quite expensive however in the future they will get cheaper as the technology becomes more developed.


# Systems Software

### Systems Software

Software that is ran to enable the functionability of the system.

### Applications Software

Software that a user installs for their own uses.

### Utilities Software

Software to help maintain the system.

### Operating Systems

#### Mutli-tasking

An operating system that allows programs to run seemingly at the same time.

#### Real-time

Operating systems that have tight time delays due to real time applications like controlling central heating or gate control on dams

#### Distributed OS

Operating system spread out over multiple devices through a network.

### Kernel

The basic component of an OS that is the foundations for it to work.

### OS Functions

### Interrupts, Device Drivers & BIOS

#### Interrupts
Interrupts stop the current task and push the CPU data to the stack where the CPU will then run the interrupt instruction defined in a sector of code. The stack is then popped and the CPU data returned.
#### Device Drivers
Device drivers allow other devices to communicate with the CPU.
#### BIOS
The basic I/O system holds all the data for a computer to maintain itself in the case of an OS failure.


### Scheduling

Scheduling is managed by the operating system and allows the programs to run seemingly simultaneously. There are 5 types of algorithms used: Round Robin, FCFS, Shortest Job First, Shortest Remaining Time, Multilevel feedback queues. Scheduling allows for other operations to run so that one program does not stop other programs from running completely if it has a loop in it.

### Round Robin

Or First in First out is where each process is given a fixed time slice or “time quanta” as wikipedia says and it can only run within this time slice. If a process overruns its time slice then it is cut off and moved to the back of a queue where it must wait to get to the front to continue. This removes the problem of 

### First Come First Served

Each process is executed in a queue however if a process is running then no other processes can execute until that process has been completed. This can cause issues with long processes however it means that if an important process is running then it will be finished without being interrupted which can be seen as a positive. Again this has the drawback of less important processes using this time instead of the higher priority processes.

### Shortest Job First

The queue is in order of time taken for the process to run starting with the shortest time. Each process is run until completion and any new processes are inserted into the queue at the correct position.

### Shortest Time Remaining

Like Shortest Job First each process is in a queue in order of time for the process to run starting with the shortest time however if a shorter process enters the queue then it takes priority over the current process.

### Multilevel feedback queues

There are multiple queues with different levels of priority. Shorter processes are preferred over long ones and processes with high I/O bursts are preferred over short bursts. I/O bound processes (ones that require an input to continue) are held in a waiting queue to stop the system from locking whilst waiting for their signal.

# Applications Generation

### Applications Software

Software designed to be used by an end user for a specific task.

### Knowledge Based System

A system that checks through a database to get answers to questions asked by an end user.

### Knowledge Base

A database of expert collected data.

### Inference Engine

The controller of the knowledge based system.
+ Takes an input
+ Searches database
+ Applies rules
+ Provides output

### Rule Base

The database that holds the set of the rules used to make decisions.

### HCI

The human-computer interface allows users to interact with the system.

### Open Source

Open source is a type of software that allows users to read and edit the source code.

### Closed Source

Closed source is a type of software that does not allow users to read and edit the source code in the user licence.

### Interpreters

Interpreters are software that runs program files without compiling it into machine code. They require systems with this software to be able to run the uncompiled code.

### Compilers

Compilers create a machine code program from a plain text program. For cross-platform programs, compilers have to be written to be compatible with most CPU types as the instructions on different CPUs vary. They also have to be written for different operating systems.

### Assemblers

Assemblers compile assembly code into machine code.

### Lexical Analysis

The lexical analysis works with the syntax analysis to produce a token tree. The lexical analysis turns the code into tokens before sending them to the syntax analysis.

### Syntax Analysis

Syntax Analysis makes sure that the code follows the rules of the programming language. Any errors caught by this stage should be fed back to the programmer for them to fix.
This doesn't catch logic errors.

### Code Generation

The code generation turns the token tree into machine code.

### Optimisation

Optimisation stops code like `(0..100).sum();` from running and replaces it with the constant value. They can also replace it with a better method of doing this like the mathematical formula for an `r` based summation.

### Linkers

Linkers transform any other files used by the program into machine code to be sent with the main executable.

### Loaders

The loader loads these files into memory for the main program to use.

### Libraries

Libraries are other programs used in another usually written at a different time or even by someone else. This allows for modular code.

















