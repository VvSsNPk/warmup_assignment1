# AI SYSTEMS PROJECT
This Repo contains code for AI-Systems project. The problem is to navigate
wumpus cave and clean it using vaacuum cleaner. I have managed to implement
a basic greedy algorithm in rust.

> **Small tip before you run it**
> This project only works when you give input files of format that are in the systems project.
> so if you run the code please make sure that you run the problems files of that format.
> You can change the element to *tip* or *warning* by renaming the style attribute below.

## Before you begin
This project only runs using rust compiler. to run the project you need cargo 
so make sure to install rust in your machine when you are running it.

## How to run the project
1. Currently even though it is a binary project i did not implement any functionality
of running the binary from cli. this project takes input as an entire directory of the
problem files of the type that as described in the systems project.

2. You can run the project by default the $DIR problems is hard coded so as long as you put
all the problem files into the problems directory and run the project you will get the solution
files in the solutions directory.

## Project Structure
````Rust
/// the greedy algorithm is written using a binary heap
fn main() -> State{
    binaryheap.pop()
    }
````

## TODO
* [ ] make this project to take cli arguements like single file or a directory.
* [ ] try to make some ui if have more time.
* [ ] try to link some rust code here from generating the rust documentation.