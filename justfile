# List all recipes.
default:
  @just --list


# Install development build.
dev:
  pip install maturin
  maturin develop --extras dev

# Install release build.
release:
  pip install maturin
  maturin develop --extras dev --profile release
