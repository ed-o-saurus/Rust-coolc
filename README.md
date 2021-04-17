# COOL Compiler in Rust

## What is COOL?
COOL is the *Classroom Object Oriented Language* which was created by [Professor Alex Aiken](https://theory.stanford.edu/~aiken/) for the purpose of teaching compiler design. A complete description can be found in the [The COOL Reference Manual](http://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf).

## What is the Purpose of this Project?
Several years ago, I took an online version of Prof Aiken's [compilers course](https://www.edx.org/course/compilers). The course had a series of challenging projects that, together, formed a functioning compiler. Recently, I wanted to learn [Rust](https://www.rust-lang.org/). To that end, I decided to implement a COOL compiler entirely in Rust. 

## Building the Compiler
In order to build the compiler, you must have the rust compiler installed. More information is available at https://www.rust-lang.org/.
Building the compiler is accomplished in the standard way:

    > cargo build
    
## Using the Compiler
### Running the Compiler
To compile COOL source files (i.e. `file1.cl`, `file2.cl`)

    > ./coolc file1.cl file2.cl

By default, the output assembly file is the name of the first source file with a `.s` extension. The output can be set is the `-o` option.

    > ./coolc file1.cl file2.cl -o output.s

### Executing 
The output of `coolc` is [MIPS](https://en.wikipedia.org/wiki/MIPS_architecture) assembly intended to run on the [SPIM](http://spimsimulator.sourceforge.net/) emulator. 

In addition to installing SPIM, you must obtain the COOL [`trap.handler`](http://theory.stanford.edu/~aiken/software/cooldist/lib/trap.handler) file.

To run the assembly file (i.e. `name.s`), run 

    > spim -exception_file trap.handler -file name.s

