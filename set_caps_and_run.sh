#!/bin/sh
set -euo pipefail

echo "Adding CAP_NET_ADMIN to '$1'"
sudo setcap cap_net_admin=ep "$1"
echo "Adding CAP_NET_RAW to '$1'"
sudo setcap cap_net_raw=ep "$1"
echo "Executing '$1'"
"$@"
