import argparse
from pathlib import Path

import pandas as pd


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path, help="input data_dir")
    parser.add_argument("output", type=Path, help="output data dir")
    args = parser.parse_args()
    return args


def remove_actors_not_in_movies(actors: pd.DataFrame, connections: pd.DataFrame) -> pd.DataFrame:
    actors_to_remove = set(actors.index) - set(connections["person_id"])
    actors = actors.drop(actors_to_remove)
    return actors


def add_actor_connectivity(actors: pd.DataFrame, connections: pd.DataFrame) -> pd.DataFrame:
    connectivity = connections.groupby("person_id").count().rename(columns={"movie_id": "connectivity"})
    # merge
    actors = actors.merge(connectivity, how="left", right_on="person_id", left_index=True)
    actors.index.name = "id"
    return actors


def make_actors_lowercase(actors: pd.DataFrame) -> pd.DataFrame:
    actors["name"] = actors["name"].str.lower()
    return actors

def remove_accents(actors: pd.DataFrame) -> pd.DataFrame:
    actors["name"] = actors["name"].str.normalize('NFKD').str.encode('ascii', errors='ignore').str.decode('utf-8')
    return actors


def main():
    args = get_args()
    input_dir: Path = args.input
    output_dir: Path = args.output

    input_movies = input_dir / "movies.csv"
    input_actors = input_dir / "people.csv"
    input_connections = input_dir / "stars.csv"

    movies = pd.read_csv(input_movies, index_col="id")
    actors = pd.read_csv(input_actors, index_col="id")
    connections = pd.read_csv(input_connections)

    actors = remove_actors_not_in_movies(actors, connections)

    # actors = add_actor_connectivity(actors, connections)

    actors = make_actors_lowercase(actors)

    actors = remove_accents(actors)

    output_dir.mkdir(parents=True, exist_ok=True)

    # rename birth to birth_year
    actors = actors.rename(columns={"birth": "birth_year"})
    actors = actors.rename(columns={"name": "full_name"})
    connections = connections.rename(columns={"person_id": "actor_id"})

    # save as parquet files
    actors.to_csv(output_dir / "actors.csv", index=True, float_format="%d")
    movies.to_csv(output_dir / "movies.csv", index=True)
    connections.to_csv(output_dir / "connections.csv", index=False)

    return


if __name__ == "__main__":
    main()
