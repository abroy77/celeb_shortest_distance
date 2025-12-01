# Celeb Search

This repository has the code for a CLI tool and webapp for finding shortest path between two actors
#### Algorithm
The program uses a Breadth First Search (BFS) algorithm to find the shortest path between two actors. 

## Webapp
The webapp is written with the help of the actix-web framework. 
A sqlite db is used to facilitate fast similarity and prefix based searching of the list of actors.

The frontend is made with the help of the bootstrap CSS library and Javascript. 


The site is live at [actorshortestpath.co.uk](https://actorshortestpath.co.uk)


## CLI TOOL

### Usage
1. Compile using:

        cargo build --release 

2. Run using:
    
        ./target/release/celeb_search <path_to_data> 

For the `<path_to_data>` One should use `data/new_small/` for testing and `data/new_large/` for the final run.


## Data
There are 3 .csv files in data/new_large. These are:
1. actors.csv
2. movies.csv
3. connections.csv

It is sourced from [CS50’s Introduction to Artificial Intelligence with Python](https://cs50.harvard.edu/ai/2023/projects/0/degrees/) 

### Actors
Contains a unique ID, name, and birth year for each actor.

### Movies
Contains a unique ID, title, and year of release for each movie.

### Connections
Contains pairs of actor IDs and movie IDs denoting which actors starred in which movies.

## Some more documentation for the webapp and the actor db for quick search



