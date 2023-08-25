#!/bin/bash

## Install a version of this script to deploy to a ubuntu server via a webhook

# exit on errors
set -e

# Set this to the branch you want to build
# Note: A future version of this script could get the code from a param from webhook
export GIT_BRANCH="develop"

export DEB_BUILD_VERSION=0.99 # Use a static version for the deb package, so we know what version to install in the webhook
export LOGFILE="/var/log/webhook/notify.log"

echo "Building Notify" >> $LOGFILE
id >> $LOGFILE
cd /home/notify/notify/

LOCKFILE=/tmp/notify-webhook-lock
if [ -e ${LOCKFILE} ] && kill -0 `cat ${LOCKFILE}`; then
    # NOTE: Because we are exiting here, we can potentially skip builds if there are multiple commits in quick succession
    echo "Script Already running" >> $LOGFILE 2>&1
    echo "Script Already running"
    exit
fi

# make sure the lockfile is removed when we exit and then claim it
trap "rm -f ${LOCKFILE}; exit" INT TERM EXIT
echo $$ > ${LOCKFILE}

if [ "$(id -u)" = "0" ]; then
  echo "Running as root. Switching to hsh user..."
  sudo -S -u hsh GIT_BRANCH=$GIT_BRANCH DEB_BUILD_VERSION=$DEB_BUILD_VERSION bash -i -c "cd /home/notify/notify/; ./build_deb_package.sh" >> $LOGFILE 2>&1
else
  echo "Not running as root."
  time ./build_deb_test_package.sh >> $LOGFILE 2>&1
fi
sudo apt -qq --allow-downgrades --reinstall install -y /home/notify/notify/backend/target/debian/notify-server_${DEB_BUILD_VERSION}_amd64.deb >> $LOGFILE 2>&1
rm -f ${LOCKFILE}