# Use the official Python image as the base
FROM python:3.10-slim-bookworm

# Set the working directory
WORKDIR /usr/src/test-optimization-sdk

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the setup files first to leverage Docker cache
COPY setup.py ./

# Copy the source code
COPY src ./src
COPY tests ./tests

# Install the package in development mode and install pytest
RUN pip install --upgrade pip && \
    pip install -e . && \
    pip install pytest

# Run the tests
CMD ["pytest", "--capture=no"] 