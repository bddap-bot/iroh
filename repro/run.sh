#!/usr/bin/env bash
set -euo pipefail
ip link add v0 type veth peer name v1
ip link set v0 addrgenmode none
ip link set v1 addrgenmode none
ip -6 addr add fe80::1/64 dev v0 nodad
ip -6 addr add fe80::2/64 dev v1 nodad
ip link set lo up
ip link set v0 up
ip link set v1 up
idx=$(ip -o link show v1 | cut -d: -f1)
echo "interfaces: $(ip -o link show | cut -d: -f1,2 | tr -d ' ' | tr '\n' ' ')"
exec "$@" "$idx"
