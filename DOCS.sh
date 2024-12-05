#!/bin/bash

echo "Generating documentation..."
cargo doc --no-deps

echo "Documentation generated in target/doc."
