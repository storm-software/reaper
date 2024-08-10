#!/bin/bash

if [ "$SHOULD_UNLOCK_GIT_CRYPT" = "1" ]; then
    echo "Added git-crypt and git-lfs"
    apk --update add git-crypt git-lfs

    echo "Unlocking monorepo..."
    git-crypt unlock

  fi
else
  echo "Not unlocking - SHOULD_UNLOCK_GIT_CRYPT is not set"
fi
