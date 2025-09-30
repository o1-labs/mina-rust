# Install Cypress dependencies for Ubuntu 24.04
# These packages are required for running Cypress end-to-end tests
# Note: libasound2t64 is used instead of libasound2 in Ubuntu 24.04

sudo apt-get update
sudo apt-get install -y libgtk2.0-0 libgtk-3-0 libgbm-dev libnotify-dev \
  libnss3 libxss1 libasound2t64 libxtst6 xauth xvfb