#!/bin/sh
# Server half of a split world (ring-1): give chosen nodes of the server's lab
# a link to the server's own host, where S1 and r1 run, and nothing else.
#
#   ns-<id> "hostlink" 198.19.0.(k+1)/32  <-veth->  host "qh-<id>" 198.19.0.1/32
#
# - Every link's host end has the same address, 198.19.0.1, so a target is
#   dialled at one address whoever dials it: S1 at
#   /ip4/198.19.0.1/udp/9229/quic-v1, r1 at /ip4/198.19.0.1/udp/9230/quic-v1.
# - Each link is point to point (a /32 route each way). Nothing is bridged
#   or forwarded, so two lab nodes can't reach each other over these links.
# - mDNS is dropped on the node's end, both ways, the same way link_filter.sh
#   drops it on Internet links. The host also drops UDP 5353 to and from qh-*.
# - The server's firewall must accept S1's and r1's ports from qh-*: table
#   qaul_hybrid drops them from anywhere but the home address. That rule is
#   added by hand, once (see REPORT-ring-world.md).
# - 198.19.0.0/24 is in the RFC 2544 test range, apart from egress.sh's
#   198.18.0.0/24 and the lab's 10.0.0.0/8.
#
# Port forwarding (ring-2: laptop gateways dial server-lab gateways): each
# forwarded public UDP port, from the home address only, is rewritten to
# the node's hostlink address and its own QUIC port, and leaves the host as
# 198.19.0.1, so the node answers over its hostlink. Nothing else is
# forwarded into or out of a qh-* link. ip_forward is switched on for it and
# set back by `down`.
#
# Usage, as root on the server:
#   hostlink.sh up ID...                         a link per server-lab node id
#   hostlink.sh forward HOME_IP PUB:ID:PORT...   forward public UDP PUB to node ID's PORT
#   hostlink.sh down                             remove every link, forward and table
#   hostlink.sh status
set -eu

TABLE=qaul_hostlink
FWD_TABLE=qaul_portfwd
STATE=/tmp/qaul_hostlink.state
HOST=198.19.0.1
NET=198.19.0

die() { echo "hostlink.sh: $*" >&2; exit 1; }

drop_mdns() {  # $1 = namespace, $2 = device
  ip netns exec "$1" tc qdisc add dev "$2" clsact
  for hook in ingress egress; do
    ip netns exec "$1" tc filter add dev "$2" "$hook" protocol ip \
      flower dst_mac 01:00:5e:00:00:fb action drop
    ip netns exec "$1" tc filter add dev "$2" "$hook" protocol ipv6 \
      flower dst_mac 33:33:00:00:00:fb action drop
    for proto in ip ipv6; do
      ip netns exec "$1" tc filter add dev "$2" "$hook" protocol "$proto" \
        flower ip_proto udp dst_port 5353 action drop
      ip netns exec "$1" tc filter add dev "$2" "$hook" protocol "$proto" \
        flower ip_proto udp src_port 5353 action drop
    done
  done
}

up() {
  [ $# -ge 1 ] || die "usage: hostlink.sh up ID..."
  [ $# -le 250 ] || die "at most 250 links fit in $NET.0/24"
  for id in "$@"; do
    ip netns list | grep -q "^ns-$id\b" || die "no namespace ns-$id; apply the topology first"
    ip link show "qh-$id" >/dev/null 2>&1 && die "qh-$id exists already; run 'hostlink.sh down' first"
  done
  k=1
  for id in "$@"; do
    n="$NET.$((k + 1))"
    ip link add "qh-$id" type veth peer name hostlink netns "ns-$id"
    ip addr add "$HOST/32" dev "qh-$id"
    ip link set "qh-$id" up
    ip route add "$n/32" dev "qh-$id" src "$HOST"
    ip -n "ns-$id" addr add "$n/32" dev hostlink
    ip -n "ns-$id" link set hostlink up
    ip -n "ns-$id" route add "$HOST/32" dev hostlink src "$n"
    drop_mdns "ns-$id" hostlink
    echo "  ns-$id: hostlink $n -> qh-$id $HOST"
    k=$((k + 1))
  done
  nft -f - <<EOF
table inet $TABLE {
  chain in {
    type filter hook input priority filter; policy accept;
    iifname "qh-*" udp dport 5353 counter drop comment "no mDNS from the lab"
  }
  chain out {
    type filter hook output priority filter; policy accept;
    oifname "qh-*" udp dport 5353 counter drop comment "no mDNS into the lab"
  }
}
EOF
  echo "hostlink up: $# link(s), host end $HOST"
}

forward() {
  [ $# -ge 2 ] || die "usage: hostlink.sh forward HOME_IP PUB:ID:PORT..."
  home="$1"; shift
  case "$home" in *[!0-9.]* | "") die "HOME_IP must be an IPv4 address, got '$home'" ;; esac
  wan="$(ip -4 route show default | awk '{for (i = 1; i < NF; i++) if ($i == "dev") { print $(i + 1); exit }}')"
  [ -n "$wan" ] || die "no IPv4 default route"
  [ -e "$STATE" ] && die "forwarding is up already ($STATE); run 'hostlink.sh down' first"
  rules=""
  for spec in "$@"; do
    pub="${spec%%:*}"; rest="${spec#*:}"; id="${rest%%:*}"; port="${rest#*:}"
    nip="$(ip -n "ns-$id" -4 -o addr show dev hostlink 2>/dev/null | awk '{print $4}' | cut -d/ -f1)"
    [ -n "$nip" ] || die "ns-$id has no hostlink; run 'hostlink.sh up' for it first"
    rules="$rules
    iifname \"$wan\" ip saddr $home udp dport $pub counter dnat to $nip:$port comment \"$id\""
    echo "  :$pub -> ns-$id $nip:$port"
  done
  echo "FORWARD_BEFORE=$(cat /proc/sys/net/ipv4/ip_forward)" > "$STATE"
  nft -f - <<EOF
table ip $FWD_TABLE {
  chain pre {
    type nat hook prerouting priority dstnat; policy accept;$rules
  }
  chain post {
    type nat hook postrouting priority srcnat; policy accept;
    oifname "qh-*" ct status dnat counter masquerade comment "reach the node as 198.19.0.1"
  }
  chain forward_q {
    type filter hook forward priority filter; policy accept;
    oifname "qh-*" ct status dnat counter accept comment "a forwarded port"
    iifname "qh-*" ct state established,related counter accept comment "its answers"
    oifname "qh-*" counter drop comment "nothing else into the lab"
    iifname "qh-*" counter drop comment "nothing else out of it"
  }
}
EOF
  sysctl -q -w net.ipv4.ip_forward=1
  echo "forwarding up: $# port(s) from $home on $wan"
}

down() {
  for ifc in $(ip -o link show | awk -F': ' '{print $2}' | sed 's/@.*//' | grep '^qh-' || true); do
    ip link del "$ifc" 2>/dev/null || true
  done
  nft delete table inet "$TABLE" 2>/dev/null || true
  nft delete table ip "$FWD_TABLE" 2>/dev/null || true
  if [ -f "$STATE" ]; then
    before="$(sed -n 's/^FORWARD_BEFORE=//p' "$STATE")"
    [ -n "$before" ] && sysctl -q -w "net.ipv4.ip_forward=$before"
    rm -f "$STATE"
    echo "hostlink down: links, forwards and tables removed, ip_forward back to $before"
  else
    echo "hostlink down"
  fi
}

status() {
  ip -br addr show | grep '^qh-' || echo "no links"
  nft list table inet "$TABLE" 2>/dev/null || true
  nft list table ip "$FWD_TABLE" 2>/dev/null || true
}

cmd="${1:-}"; [ $# -gt 0 ] && shift
case "$cmd" in
  up) up "$@" ;;
  forward) forward "$@" ;;
  down) down ;;
  status) status ;;
  *) die "usage: hostlink.sh up ID... | forward HOME_IP PUB:ID:PORT... | down | status" ;;
esac
