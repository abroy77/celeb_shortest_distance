#!/bin/zsh

rsync -azvP \
  --delete \
  --exclude 'target' \
  --exclude '.git' \
  --exclude '__pycache__' \
  --exclude 'data_cleaning' \
  --exclude 'migrations' \
  --exclude 'scripts' \
  ~/Documents/code/rust/celeb_shortest_distance \
  piroy:~/apps/