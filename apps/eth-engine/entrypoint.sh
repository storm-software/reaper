#!/bin/sh

echo "Release: $RELEASE"
reaper -c /etc/reaper/reaper.yaml -r $RELEASE
