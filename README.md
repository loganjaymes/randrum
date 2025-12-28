<h1>
<p align="center">
  <br>RANDRUM
</h1>
  <p align="center">
    Random drum grooves to help break writer's block. 
    <br />
    <a href="#about">About</a>
    <a href="#installation">Installation</a>
    <a href="#docs">Docs</a>
    <a href="#roadmap-and-status">Roadmap and Status</a>
  </p>
</p>

## About
RANDRUM is a CLI tool created in Rust that randomly merges individual drumhead MIDI files into full drum grooves.

## Installation
0. Ensure Rust/Cargo is installed on your machine.
```shell-session
cargo --version
```
1. Clone the repo.
```shell-session
git clone git@github.com:loganjaymes/randrum.git
```
2. Run cargo run in `randrum/`.
```shell-session
cargo run
```
3. Edit any parameters in `main.rs`.
> [!NOTE]
>
> Above will be different once a CLI is set up, as a user would be able to just run the .exe.
> Fix this file after such is done.

## Docs
See the [documentation](DOCS.md). 

## Roadmap and Status

|  #  | Task                                                                                   | Status |
| :-: | -------------------------------------------------------------------------------------- | :----: |
|  1  | Beginning documentation                                                                |   ✅   |
|  2  | Basic functionality (randomizing given .mid files & exporting)                         |   ✅   |
|  3  | CLI                                                                                    |   ⚠    |
|  4  | Finalize README (include preview/demo)                                                 |   ⚠    |
|  5  | Implementation for time signatures other than 4/4                                      |   ❌   |
|  6  | Implementation for non-normalized midi files (fe. one .MID is 4 bars vs 8 bars)        |   ❌   |
|  7  | TUI?                                                                                   |    ~   |