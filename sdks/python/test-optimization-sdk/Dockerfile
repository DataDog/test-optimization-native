# Use the official Python image as the base
FROM python:3.10-slim-bookworm@sha256:a1a9bb579ff76c1751323cbdefb416462b225868dadc657e44053c266d63091a

# Set the working directory
WORKDIR /usr/src/test-optimization-sdk

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the setup and requirements files first to leverage Docker cache
COPY setup.py requirements.txt ./

# Copy the source code
COPY src ./src
COPY tests ./tests

# Install the package in development mode using pinned dependencies
RUN pip install --upgrade pip && \
    pip install -r requirements.txt && \
    pip install -e . --no-deps

# Run the tests
CMD ["pytest", "--capture=no"] 