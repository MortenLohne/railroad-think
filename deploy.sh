#!/bin/sh

TARGET=2023/railroad-think

USERNAME=$PROD_USERNAME
HOSTNAME=$PROD_HOSTNAME
BASE_PATH=$PROD_BASE_PATH
BASE_URL=${PROD_BASE_URL/http:/https:}

if [[ "$1" = "--stage" ]]
then
  echo "Deploying to STAGE environment"
  USERNAME=$STAGE_USERNAME
  HOSTNAME=$STAGE_HOSTNAME
  BASE_PATH=$STAGE_BASE_PATH
  BASE_URL=$STAGE_BASE_URL
fi

echo "rsync -a --progress -e ssh dist/ $USERNAME@$HOSTNAME:${BASE_PATH}${TARGET}"
rsync -a --progress -e ssh dist/ $USERNAME@$HOSTNAME:${BASE_PATH}${TARGET}
