#!/bin/bash

if [ "$SHOULD_UNLOCK_GIT_CRYPT" = "1" ]; then
    echo "Added git-crypt and git-lfs"

    curl -L https://github.com/AGWA/git-crypt/releases/download/0.7.0/git-crypt-0.7.0-linux-x86_64 \
        -o ./git-crypt

    echo "Unlocking monorepo..."
    ./git-crypt unlock

else
  echo "Not unlocking - SHOULD_UNLOCK_GIT_CRYPT is not set"
fi
