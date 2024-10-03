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


