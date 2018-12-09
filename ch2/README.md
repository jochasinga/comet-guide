# Chapter 2: Introduction to Operating Systems

A running program does one simple thing: executes instructions, millions of them.

## Basic [Von Neumann][1] model of computing

### The processor (CPU):

- **fetches** an instruction from memory
- **decodes** it (figures out which instruction it is)
- **executes** it (like add two numbers together, access memory, check condition, jump to a function, etc.)
- moves on to the next instruction

![Von Neumann's Bottleneck on Wikipedia][2]

### Operating system is

A software that abstracts the machine's hardware interface and make it easy to run programs,
giving the illusion of running many at once, let programs share memory, and connect programs
to other connected devices.

The whole process is called **virtualization**.

### Operating system does

- virtualize machine's physical resource (CPU, memory, disk) and transform it into a more general easy-to-use **virtual** form of itself
- provide interfaces/APIs that a program can call, also known as the **standard library**

> 👋  We sometimes refer to OS as **virtual machine**.

```rust

use std::env;
use std::process::exit;
use std::{thread, time};

fn main() {
    // Check if the number of command line arguments is 2
    if env::args().len() != 2 {
        eprintln!("usage: cpu <string>");
        exit(1);
    }

    // Retrieve the second argument (the first is the command line invocation).
    let string = env::args().nth(1).unwrap_or_else(|| "".to_string());
    let one_second = time::Duration::from_millis(1000);

    // Print the argument every one second
    loop {
        println!("{}", string);
        thread::sleep(one_second);
    }
}

```

Figure 2.1: **Simple Example: Code That Loops and Prints** [![open playground](../assets/open-playground-3b8277.svg)][3]

### 2.1 Virtualizing the CPU

In Figure 2.1 the system runs program, which repeatedly checks the time until one second has elapsed
and prints the input string passed in by user. (The program runs forever so press `Ctrl-c` to terminate).

> 👋  `Ctrl-c` is known as `KeyboardInterrupt`. This signal interrupts the processor from whatever it is working on.

```shell

$ cd cpu  # make sure you're in the project dir
$ cargo build
$ ./target/debug/cpu "A"
A
A
A
A
^C # Ctrl-C
$

```

Now let's try running the program in several processes.

```shell

$ cd target/debug # chdir for convenience
$ (./cpu A &) ; (./cpu B &) ; (./cpu C &) ; (./cpu D &)
A
B
D
C
A
B
D
C
A
C
B
D
# ...

```

Figure 2.2: **Running Many Programs At Once**

The OS (with some help from the hardware) creates an **illusion** that the system has a
very large number of virtual CPUs when it only has one. This process is called **virtualizing the CPU**.

OS has a **policy** that determines which program *should* run at a particular time (and which should wait)
when more than one program wants to run at the same time. This makes the OS the role of **resource manager**.

```rust

use std::process;
use std::{thread, time};
use std::alloc::{alloc, Layout};

fn main() {
    unsafe {
        let layout = Layout::new::<isize>();
        let p = alloc(layout);
        println!("({}) address pointed to by p: {:p}", process::id(), p);
        *(p as *mut isize) = 0;
        loop {
            thread::sleep(time::Duration::from_millis(1000));
            *(p as *mut isize) += 1;
            println!("({}) p: {}", process::id(), *p);
        }
    }
}

```

Figure 2.3: **A Program That Accesses Memory** [![open playground](../assets/open-playground-3b8277.svg)][4]

> ⚠️ Beware that the code in figure 2.3 is unsafe.
> This is normal in the C/C++ world, but not in Rust.
> Please see `main_safe.rs` in the same directory
> or visit the [safe code in the playground][5].

## 2.2 Virtualizing Memory

Memory model in modern machines is simple--Memory is just an array of bytes.

```text

[ 0x200000 | 0x34d203 | 0x102000f | ... ]

```

To **read** memory, specify the **address** to access the data stored there.
To **write** (or **update**) memory, also specify the value to be written.

The program does not only store the data specified by in the program. It also stores each instruction
in the memory. Thus memory is access on each instruction fetch.

Let's try running the code (building Rust code and `cd`ing into the relevant directory will be omitted from now on).

```shell

$ ./mem
(11721) address pointed to by p: 0x7ffee5504d48
(11721) p: 1
(11721) p: 2
(11721) p: 3
(11721) p: 4
(11721) p: 5
^C

```

In Rust, manually allocating memory is considered unsafe. What the programs does are

- Initializes / allocates some memory for type `isize` (architecture-dependent sized integer)
- Prints out the address of the memory
- Loops, delays for a second, and increment the value at that address
- Also prints what is called the process identifier (PID) of the program

```shell

$ (./mem &); (./mem &)
(17415) address pointed to by p: 0x7ffeec85fd68
(17417) address pointed to by p: 0x7ffee018ed68
(17415) p: 1
(17417) p: 1
(17415) p: 2
(17417) p: 2
(17415) p: 3
(17417) p: 3
(17415) p: 4
(17417) p: 4
# ...

```

Figure 2.4: **Running the Memory Program Multiple Times**

When run multiple instances of the program we see the program has
allocated memory at the same address(*0x7ffeec85fd68*) for both processes. But
each *seems* to be updating the value at *0x7ffeec85fd68* independently.

It is as if each running program has its own private memory instead of sharing
the same physical memory with other running programs.

This is the OS **virtualizing memory**. Each proceess accesses its own private
**virtual address space** (sometimes called **address space**), which the OS maps onto the physical memory of the machine.

A memory reference within a running program does not affect the address space of other processes (or the OS itself). The reality is the physical memory is a shared resource managed by the OS.

## 2.3 Concurrency

**Concurrency** is a term to refer to a host of problems when working on many things at once (i.e., concurrently) in the same program.

```rust

use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, thread};

fn main() {
    // Arc atomically count references to this resource making it possible
    // to call `counter.clone()` later.
    let counter = Arc::new(Mutex::new(0));
    let loops: isize;

    if env::args().len() != 2 {
        eprintln!("usage: threads <value>");
        exit(1);
    }

    let first_arg = env::args().nth(1).unwrap_or_else(|| "".to_string());
    loops = first_arg.parse::<isize>().unwrap();

    println!("Initial value : {}", counter.lock().unwrap());

    let mut children = Vec::new();

    for _ in 0..2 {
        let counter = counter.clone();
        let child = thread::spawn(move || {
            for _i in 0..loops {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            // the lock is unlocked here when `num` goes out of scope.
        });
        children.push(child)
    }

    for child in children {
        child.join().unwrap();
    }

    println!("Final value    : {}", counter.lock().unwrap());
}

```

Figure 2.5: **A Multi-threaded Program** [![open playground](../assets/open-playground-3b8277.svg)][6]

## 2.4 Persistence

```rust

use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/tmp/file")
        .unwrap()
        .write_all(b"hello world\n")?;
    Ok(())
}

```

Figure 2.6: **A Program That Does I/O** [![open playground](../assets/open-playground-3b8277.svg)][7]

Data can be easily lost in system memory, since devices like [DRAM][8] store values in a **volatile** manner.

The software in the OS that manages the disk is called the **file system**.

Unlike the abstractions OS provides for CPU and memory, it does not create a private, virtualized disk for each application. It is assumed that users will want to **share** information that is in files. Files are shared across different processes.

In figure 2.6 the code creates a file (`/tmp/file`) that contains the string "Hello world".

The program makes three calls into the OS.

1. `open()` opens the file and creates it
2. `write()` writes some bytes of data to the file
3. `close()` closes the file (implicit in Rust)

These **system calls** are routed to the part in OS called the **file system**.

## 2.5 Design Goals

### The OS

- takes physical **resources** (like CPU, memory, or disk) and **virtualizes** them
- handles **concurrency** issues
- stores files **persistently**

### Some goals

- to build **abstractions** to make the system easy to use
- to provide high **performance** or **minimize the overheads** of the OS
- to provide **protection** between applications as well as between OS and applications (**isolation**)
- to provide high **reliability**

### Other goals

- **energy-efficiency**
- **security** against malicious applications
- **mobility** in smaller devices

## 2.6 Some History

### Early OS: Just Libraries

In the beginning, the OS was a set of libraries of commonly-used functions; Instead of writing low-level I/) handling code, the "OS" would provide such APIs for the programmer.

### Beyond Libraries: Protection

Protect application from accessing hardware freely. The idea of **system call** was invented. Instead of providing OS routines as a library that any application code can call as a **procedure call** (think modern RPC), the system call transfers control (i.e., jumps) into the OS while raising the **hardware privilege level**. User applications run in what is referred to as **user mode** which means the hardware restricts what applications can do; for example, an application running in user mode can't initiate an I/O request to disk, access physical memory page, or send a package on the network.

When a system call is initiated (through a special hardware instruction called a **trap**), the hardware transfers control to a pre-specified **trap handler** and raises the privilege level to **kernel mode**.

In kernel mode, the OS has full access to the hardware and can do things like initiate an I/O request or make more memory available to a program. When OS is done, it passes control back to the user via a special **return-from-trap** instruction and reverts to user mode and passing control back to where the application left off.

### The Era of Multiprogramming

TBD...

[1]: https://en.wikipedia.org/wiki/John_von_Neumann
[2]: https://upload.wikimedia.org/wikipedia/commons/thumb/e/e5/Von_Neumann_Architecture.svg/1920px-Von_Neumann_Architecture.svg.png
[3]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=b4424d0f10aa8db25eb2b1429021ea4c
[4]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0ab4045f3b0fad1708f99230cd81e90a
[5]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=d30e99297261bfcd32036639f2bb0aca
[6]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d882688e6201d137614fc9b300b89429
[7]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=573cca48bc1ce5e4b7f62df00fd22690
[8]: https://en.wikipedia.org/wiki/Dynamic_random-access_memory