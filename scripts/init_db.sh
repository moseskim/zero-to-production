#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# 커스텀 사용자가 설정되어 있는지 확인한다. 설정되어 있지 않으면 'postgres'로 설정한다
DB_USER="${POSTGRES_USER:=postgres}"
# 커스텀 비밀번호가 설정되어 있는지 확인한다. 설정되어 있지 않으면 'password'로 설정한다.
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# 커스텀 데이터베이스 이름이 설정되어 있는지 확인한다. 설정되어 있지 않으면 'newsletter'로 설정한다.
DB_NAME="${POSTGRES_DB:=newsletter}"
# 커스텀 포트가 설정되어 있는지 확인한다. 설정되어 있지 않으면 '5432'로 설정한다.
DB_PORT="${POSTGRES_PORT:=5432}"
# 커스텀 호스트가 설정되어 있는지 확인한다. 설정되어 있지 않으면 'localhost'로 설정한다.
DB_HOST="${POSTGRES_HOST:=localhost}"

# 도커화 된 Postgres 데이터베이스가 이미 실행되고 있다면 도커를 스킵한다.
if [[ -z "${SKIP_DOCKER}" ]]
then
  # postgres 컨테이너가 실행 중이라면, 이를 중지시키는 명령얼 출력하고 종료한다.
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=postgres' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a postgres container already running, kill it with"
    echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi
  # 도커를 사용해서 postgres를 기동한다.
  docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d \
      --name "postgres_$(date '+%s')" \
      postgres -N 1000
      # ^ 테스팅 목적으로 최대 커넥션 수를 증가시킨다.
fi

# Postgres가 명령어를 받을 수 있을 때까지 계속 확인(ping)한다
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
