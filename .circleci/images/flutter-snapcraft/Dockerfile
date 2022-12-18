FROM --platform=linux/amd64 ubuntu:focal

# Install Common Dependencies
ARG DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y \
    git ssh tar gzip ca-certificates \
    curl unzip xz-utils zip libglu1-mesa \
    apt-transport-https clang cmake ninja-build pkg-config libgtk-3-dev apt-utils openssl snapd

# Set up new user
RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

# Download Flutter SDK
ENV FLUTTER_VERSION 3.3.9
ENV PATH "$PATH:/home/developer/flutter/bin"
RUN curl -s https://raw.githubusercontent.com/brenodt/flutter_utilities/main/bin/flinstall | bash /dev/stdin "$FLUTTER_VERSION"

# Run basic check to download Dark SDK
RUN flutter doctor

# Install Snapcraft
RUN snap install snapcraft --classic
RUN snapcraft --version

# Install Flutter Snapcraft Plugin
RUN sudo snap install flutter-snapcraft --classic

# Clone project and set it as the entrypoint
RUN git clone https://github.com/qaul/qaul.net.git
ENTRYPOINT ["/qaul.net"]
