# List all recipes.
default:
  @just --list


# Install development build.
dev:
  maturin develop --extras dev

# Install release build.
release:
  maturin develop --extras dev --profile release
