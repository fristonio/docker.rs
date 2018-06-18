#!/bin/bash

if [ -z $(which rustfmt) ]; then
  echo 'It seems like rustfmt is not installed'
  echo 'Install rustfmt first using `rustup component add rustfmt-preview`'
  exit 2
fi

make format
