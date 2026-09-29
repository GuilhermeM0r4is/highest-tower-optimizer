# Highest Tower Optimizer

A program that determines the tallest stable tower that can be built from a set of available block types.

## Overview
Each block is a rectangular cuboid and can be reoriented before it is placed. A block may sit on another block only when both dimensions of its base are strictly smaller than the corresponding dimensions of the block below it.

The program computes the maximum possible tower height and outputs one valid arrangement, from the base to the top, including the orientation of each block.

## Input
The input starts with a positive integer `n`, the number of block types. Each of the following `n` lines contains three positive integers describing one block type’s dimensions.

## Output
The first line contains the maximum tower height and the number of blocks used. Each following line describes one block in the tower, from the base to the top, in its chosen orientation.

## Build and run
Build and run instructions depend on the implementation language and source file. Add the commands here once those are decided.
The program reads from standard input and writes to standard output.

## Project context
This project was developed for the Algorithms Analysis and Synthesis course at Instituto Superior Técnico, academic year 2026/2027.
