# https://stackoverflow.com/questions/69290453/how-to-properly-use-custom-image-in-vscode-remote-containers
# Using the latest epitech image
FROM epitechcontent/epitest-docker

# Setup a custom user with similar permissions
RUN useradd -rm -d /home/dev -s /bin/zsh -g root -G root -u 1001 dev

# Configure our workspace
WORKDIR /home/dev
RUN touch dummyfile