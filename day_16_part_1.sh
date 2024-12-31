#!/bin/sh

grep -v 'children. [0124-9]'  input/2015/day16.txt |
  grep -v 'cats: [0-689]' |
  grep -v 'samoyeds: [013-9]' |
  grep -v 'pomeranians: [0-24-9]' |
  grep -v 'akitas: [1-9]' |
  grep -v 'vizslas: [1-9]' |
  grep -v 'goldfish: [0-46-9]' |
  grep -v 'trees: [0-24-9]' |
  grep -v 'cars: [013-9]'
