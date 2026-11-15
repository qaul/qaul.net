#!/bin/sh
# Give chosen lab nodes a way out to test.qaul.net, and nothing else.
#
# In a hybrid run the far nodes (S1, r1) run on test.qaul.net. The lab ends of
# the far links (T1, T2, V1, W1, r3, S2 in world-1) dial them over the real
# Internet. Each of those nodes gets its own point-to-point link to the WSL
# host, and the host NATs it out through its default interface:
#
#   ns-<id> "egress" 198.18.0.(4k+2)/30  <-veth->  host "qe-<id>" 198.18.0.(4k+1)/30
#
# - Inside the node, the only route over that link is to the server (/32), so
#   all other traffic stays in the lab. Members get no link at all.
# - Every node has its own /30, routed and not bridged, so two gateways cannot
#   share a segment, and mDNS (link-local multicast) is never forwarded.
# - The host table "qaul_egress" lets the links reach the server only. It
#   drops anything between them, anything new coming in from outside, and
#   anything addressed to the host itself.
# - 198.18.0.0/15 is reserved for testing (RFC 2544), clear of WSL's 172.x
#   network and the lab's 10.0.0.0/8.
#
# Usage, as root:
#   egress.sh up SERVER_IP ID...   add a link per node id, the NAT and the filter
#   egress.sh down                 remove all of it, restore ip_forward
#   egress.sh status               links, routes and the filter's counters
#   egress.sh counters             "id up_bytes down_bytes" per link, from the host
#
# Removing the lab (network.py clear) deletes the node namespaces, which
# deletes their links too; `down` is still needed for the table and ip_forward.
set -eu

STATE="${EGRESS_STATE:-/tmp/qaul_egress.state}"
TABLE=qaul_egress
NET=198.18.0

die() { echo "egress.sh: $*" >&2; exit 1; }

wan_if() {
  ip -4 route show default | awk '{for (i = 1; i < NF; i++) if ($i == "dev") { print $(i + 1); exit }}'
}

up() {
  [ $# -ge 2 ] || die "usage: egress.sh up SERVER_IP ID..."
  server="$1"; shift
  case "$server" in
    *[!0-9.]* | "") die "SERVER_IP must be an IPv4 address, got '$server'" ;;
  esac
  [ $# -le 63 ] || die "at most 63 nodes fit in ${NET}.0/24"
  [ -e "$STATE" ] && die "already up ($STATE exists); run 'egress.sh down' first"
  wan="$(wan_if)"
  [ -n "$wan" ] || die "no IPv4 default route, so no way out"

  for id in "$@"; do
    ip netns list | grep -q "^ns-$id\b" || die "no namespace ns-$id; apply the topology first"
  done

  echo "FORWARD_BEFORE=$(cat /proc/sys/net/ipv4/ip_forward)" > "$STATE"
  {
    echo "WAN=$wan"
    echo "SERVER=$server"
    echo "NODES=\"$*\""
  } >> "$STATE"

  k=0
  for id in "$@"; do
    h="$NET.$((4 * k + 1))"
    n="$NET.$((4 * k + 2))"
    ip link add "qe-$id" type veth peer name egress netns "ns-$id"
    ip addr add "$h/30" dev "qe-$id"
    ip link set "qe-$id" up
    ip -n "ns-$id" addr add "$n/30" dev egress
    ip -n "ns-$id" link set egress up
    ip -n "ns-$id" route add "$server/32" via "$h" dev egress
    echo "  ns-$id: egress $n/30 -> qe-$id $h, route $server/32"
    k=$((k + 1))
  done

  nft -f - <<EOF
table ip $TABLE {
  chain post {
    type nat hook postrouting priority srcnat; policy accept;
    ip saddr $NET.0/24 oifname "$wan" masquerade
  }
  chain forward_q {
    type filter hook forward priority filter; policy accept;
    iifname "qe-*" oifname "$wan" ip daddr $server counter accept comment "to the server"
    iifname "qe-*" counter drop comment "anything else from an egress link"
    oifname "qe-*" ct state established,related counter accept comment "the server's replies"
    oifname "qe-*" counter drop comment "anything new from outside"
  }
  chain input_q {
    type filter hook input priority filter; policy accept;
    iifname "qe-*" counter drop comment "nothing for the host itself"
  }
}
EOF
  sysctl -q -w net.ipv4.ip_forward=1
  echo "egress up: $# link(s), NAT out of $wan, only to $server"
}

down() {
  for ifc in $(ip -o link show | awk -F': ' '{print $2}' | sed 's/@.*//' | grep '^qe-' || true); do
    ip link del "$ifc" 2>/dev/null || true
  done
  nft delete table ip "$TABLE" 2>/dev/null || true
  if [ -f "$STATE" ]; then
    before="$(sed -n 's/^FORWARD_BEFORE=//p' "$STATE")"
    [ -n "$before" ] && sysctl -q -w "net.ipv4.ip_forward=$before"
    rm -f "$STATE"
    echo "egress down: links and table removed, ip_forward back to $before"
  else
    echo "egress down: was not up (no $STATE); removed any leftovers"
  fi
}

status() {
  if [ -f "$STATE" ]; then cat "$STATE"; else echo "not up"; fi
  ip -br addr show | grep '^qe-' || true
  nft list table ip "$TABLE" 2>/dev/null || true
}

counters() {
  # /proc/net/dev follows the current network namespace; /sys may not.
  # The host receives what the node sends, so rx here is the node's upload.
  awk -F'[: ]+' '/^ *qe-/ {
    sub(/^ */, ""); split($0, f, /[: ]+/)
    id = f[1]; sub(/^qe-/, "", id)
    print id, f[2], f[10]
  }' /proc/net/dev
}

cmd="${1:-}"; [ $# -gt 0 ] && shift
case "$cmd" in
  up) up "$@" ;;
  down) down ;;
  status) status ;;
  counters) counters ;;
  *) die "usage: egress.sh up SERVER_IP ID... | down | status | counters" ;;
esac
