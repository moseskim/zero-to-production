#!/usr/bin/env bash
set -x
set -eo pipefail

# redis 컨테이너가 실행 중이라면, 이를 중지시키는 명령얼 출력하고 종료한다.
RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a redis container already running, kill it with"
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# 도커를 사용해서 redis를 기동한다.
docker run \
    -p "6379:6379" \
    -d \
    --name "redis_$(date '+%s')" \
    redis:7

>&2 echo "Redis is ready to go!"