import psycopg2
import pandas as pd
from pathlib import Path

def read_csv(file: Path) -> pd.DataFrame:
    
    df = pd.read_csv(file)
    print(df.head())
    return df

def make_empty_db():
    


def main():
    _ = read_csv(Path('../data/new_large/actors.csv'))
    print("done")


if __name__ == '__main__':
    main()