#!/usr/bin/env /usr/local/bin/bash

 # * <bitbar.title>Kubernetes status bar</bitbar.title>
 # * <bitbar.version>v1.0</bitbar.version>
 # * <bitbar.author>sdbondi</bitbar.author>
 # * <bitbar.author.github>sdbondi</bitbar.author.github>
 # * <bitbar.desc>Displays kubernetes pods running in the given namespaces</bitbar.desc>
 # * <bitbar.dependencies>bash</bitbar.dependencies>
 # * <bitbar.dependencies>kubectl</bitbar.dependencies>
 # * <bitbar.dependencies>jq</bitbar.dependencies>
 # * <bitbar.image></bitbar.image>
 # * <bitbar.abouturl>https://github.com/sdbondi/bitbar-plugins</bitbar.abouturl>

NAMESPACES=(production staging)
EXTRA_PATHS=

export PATH="/usr/local/bin:/usr/bin:$EXTRA_PATHS:$PATH"

echo "Kubernetes"
echo "---"

for n in "${NAMESPACES[@]}"; do
  echo $n
  RAW_PODS=$(kubectl get pods -n$n -ojson | jq '.items' | jq -c '.[]')
  readarray -t PODS <<<"$RAW_PODS"
  for p in "${PODS[@]}"; do
    TAG=$(echo $p | jq -r '.spec.containers[0].image | split(":")' | jq -r '.[1]')
    NAME=$(echo $p | jq -r '.metadata.name')
    PHASE=$(echo $p | jq -r '.status.phase')
    if [[ "$PHASE" == "Running" || "$PHASE" == "Succeeded" ]]; then
      echo "--:white_check_mark: $NAME ($TAG): $PHASE | color=#fff terminal=true bash=kubectl param1=-n param2=$n param3=exec param4=-it param5=\"$NAME -- /bin/sh\" refresh=true"
    else
      echo "--:large_orange_diamond: $NAME ($TAG): $PHASE | color=#ff0000"
    fi
  done
done
