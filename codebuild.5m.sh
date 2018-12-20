#!/usr/bin/env /usr/local/bin/bash

 # * <bitbar.title>Codebuild status bar</bitbar.title>
 # * <bitbar.version>v1.0</bitbar.version>
 # * <bitbar.author>sdbondi</bitbar.author>
 # * <bitbar.author.github>sdbondi</bitbar.author.github>
 # * <bitbar.desc>Displays last builds for codebuild projects</bitbar.desc>
 # * <bitbar.dependencies>bash</bitbar.dependencies>
 # * <bitbar.dependencies>aws-sdk</bitbar.dependencies>
 # * <bitbar.dependencies>jq</bitbar.dependencies>
 # * <bitbar.image></bitbar.image>
 # * <bitbar.abouturl>https://github.com/sdbondi/bitbar-plugins</bitbar.abouturl>

PROJECTS=(my-project-1 my-project-2)
MAX_BUILDS=5
EXTRA_PATHS=

export PATH="/usr/local/bin:/usr/bin:$EXTRA_PATHS:$PATH"

echo "CodeBuild"
echo "---"

for p in "${PROJECTS[@]}"; do
  echo $p
  BUILDS=$(aws codebuild list-builds-for-project --project-name $p --max-items $MAX_BUILDS | jq '.ids' | jq -c '.[]')
  for b in "${BUILDS[@]}"; do
    CMD=`echo aws codebuild batch-get-builds --ids $(echo $BUILDS | xargs printf -- '\"%s\" ')`
    RAW_PROJECT_BUILDS=$(eval $CMD | jq '.builds' | jq -c '.[]')
    readarray -t PROJECT_BUILDS <<<"$RAW_PROJECT_BUILDS"
    for pb in "${PROJECT_BUILDS[@]}"; do
      echo "--$(echo $pb | jq -r '.sourceVersion') - $(echo $pb | jq -r '.buildStatus')"
    done
  done
done
