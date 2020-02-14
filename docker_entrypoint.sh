#!/bin/bash
set -e

if [ "$1" = 'splitterrust_discord' ]; then
    chown -R splitterrust: "$DISCORD"
    su splitterrust -s /bin/bash -c "$DISCORD"
fi
