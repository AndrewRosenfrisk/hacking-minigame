# Hacking Minigame

## About

In this game, the player must hack a computer by guessing a seven-letter 
word used as the secret password. The computer’s memory banks display the 
possible words, and the player is given hints as to how close each guess was. 
For example, if the secret password is MONITOR but the player guessed CONTAIN, 
they are given the hint that two out of seven letters were correct, because both
MONITOR and CONTAIN have the letter O and N as their second and third letter. 
This game is similar to Project 1, “Bagels,” and the hacking minigame in the 
Fallout series of video games.

## Running the project
* Install Rust: [rustup.rs](https://rustup.rs/)
* Clone the repository locally:
  * `git clone https:://github.com/AndrewRosenfrisk/hacking-minigame`
  * `cd hacking-minigame`
* Build the project with `cargo build`
* Run the project with `cargo run`

Based on the project detailed in the "[Big Book of Small Python Projects](https://inventwithpython.com/bigbookpython/project33.html)"
