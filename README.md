# Water Sorter Solver

This is an attempt to write a water sorter solver program. This is based on the basic games where you have to pour different coloured liquids into tubes resulting in each tube only containing one colour. The following are the basic rules for this game (that may be extended in future):

- Each tube is 4 cells large
- There are always 2 empty tubes at the start of each game
- You can only pour one cell of liquid into a tube if it is either
  - empty
  - the top filled cell contains the same colour liquid
- Liquid can only be poured if there is space in the tube
- Multiple cells can be poured in one move if there is sufficient space, and the top existing colour is the same, or the tube is empty.
- Always pour as much liquid as you can.

## Steps

This project will (hopefully) comprise of various different phases:

1. Implenting the game functionality, so you can "play" a very basic command line version.
2. Implementing a brute force solver. This is will probably only be feasible for simple games.
3. Implementing a more sophisticated solver that will try and find the shortest solution path. More research needs to be done before implementing this.

## Current Bugs

1. Tube number indexing should be stored as 0-based, but displayed as 1-based. This needs to be updated in tube.rs mainly and the tests need to be changed.
2. Update REPL so that it has all the functionality already present in the backend.
3. Add full integration tests as a separate file, with separate cargo config.
