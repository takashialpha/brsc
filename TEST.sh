#!/bin/bash

cat TESTING.md
sleep 20
read -p "Have you made the requested changes? (yes/no): " response
if [ "$response" == "yes" ]; then
  read -p "Do you want to proceed with cargo test? (yes/no): " proceed
  if [ "$proceed" == "yes" ]; then
    cargo test
  else
    echo "Operation canceled."
  fi
else
  echo "Please make the requested changes first."
fi
