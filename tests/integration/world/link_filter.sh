#!/bin/sh
# meshnet-lab --link-command hook: drop mDNS on Internet-only links.
#
# network.py runs this inside the "switch" namespace twice per link, once for
# each end, as:
#
#   link_filter.sh {action} {ifname} {internet_only}
#
# gen_world.py marks every Internet link with internet_only=1. Radio links
# carry no such field, so meshnet-lab substitutes an empty string and this
# exits without touching them.
#
# Why: a lab link carries everything between two nodes. Two LAN-enabled
# gateways joined by one (W1-H1) would discover each other by mDNS, become LAN
# neighbours and merge their communities into one Local sphere. Dropping UDP
# 5353 on that link leaves only the Internet transport able to connect.
# qaul's mDNS runs over IPv4 only (connections/lan.rs), but IPv6 is dropped
# too so the filter does not depend on that.
#
# The port rules alone leak. libp2p-mdns puts every listen address in one
# reply, each repeating a random 32-63 character name, so replies vary around
# the 1500-byte MTU, and the larger ones leave as IP fragments. tc flower does
# not read UDP ports on a fragment, not even the first, so a fragmented reply
# passes the port rules untouched. Every fragment of an mDNS packet still
# carries the mDNS multicast MAC, so the MAC rules catch it. Tested in a
# private network namespace on 2026-11-12: a 2000-byte packet to
# 224.0.0.251:5353 crossed the port rules as two fragments, and the MAC rule
# stopped it.
#
# meshnet-lab installs this only after the link is already up and bridged, so
# a link created between two running LAN nodes is open for a moment. That is
# handled by when the link is created, not here: see gen_world.py.
#
# A failure here aborts `network.py apply`, which is wanted: a filter that
# cannot be installed must stop the run, not let two communities merge.
set -eu

action="${1:-}"
ifname="${2:-}"
internet_only="${3:-}"

[ "$internet_only" = "1" ] || exit 0

case "$action" in
  create)
    tc qdisc add dev "$ifname" clsact
    for hook in ingress egress; do
      # mDNS multicast MACs: 224.0.0.251 and ff02::fb. Catches fragments.
      tc filter add dev "$ifname" "$hook" protocol ip \
        flower dst_mac 01:00:5e:00:00:fb action drop
      tc filter add dev "$ifname" "$hook" protocol ipv6 \
        flower dst_mac 33:33:00:00:00:fb action drop
    done
    for proto in ip ipv6; do
      for hook in ingress egress; do
        # Unfragmented mDNS by port, including any sent unicast.
        tc filter add dev "$ifname" "$hook" protocol "$proto" \
          flower ip_proto udp dst_port 5353 action drop
        tc filter add dev "$ifname" "$hook" protocol "$proto" \
          flower ip_proto udp src_port 5353 action drop
      done
    done
    ;;
  update | remove)
    # Nothing to change on update. On remove, deleting the veth takes its
    # qdisc and filters with it.
    ;;
  *)
    echo "link_filter.sh: unknown action '$action'" >&2
    exit 1
    ;;
esac
