#! /usr/bin/env bash

source /share/apps/gaussian/env.rc

input=${1%.gjf}.com
xdh rewrite $1 > $input
g09 $input
xdh obtain ${input%.com}.log
