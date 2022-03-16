#! /usr/bin/env bash

source /share/apps/gaussian/env.rc

# $1 is supposed to be Gaussian input file: foo.com
cat $1 | xdh rewrite | g09 | tee test.log | xdh obtain
