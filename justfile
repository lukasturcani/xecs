# List all recipes.
default:
  @just --list


# Install development build.
dev:
  maturin develop --extras dev
