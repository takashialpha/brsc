#!/bin/bash

if [ "$EUID" -ne 0 ]; then
  echo "You must run this script as root."
  exit
fi

echo "Uninstalling brsc..."

if [ ! -f "/bin/brsc" ]; then
  echo "brsc is not installed in /bin."
  exit 1
fi

rm -f /bin/brsc

echo "brsc has been uninstalled."
