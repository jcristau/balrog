# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
import os
import subprocess
from pathlib import Path

import requests

FETCHES_DIR = Path(os.environ["MOZ_FETCHES_DIR"])
SECRET_BASEURL_TPL = "http://taskcluster/secrets/v1/secret/{}"


def fetch_secret(secret_name):
    """Retrieves the given taskcluster secret"""
    secret_url = SECRET_BASEURL_TPL.format(secret_name)
    r = requests.get(secret_url)
    r.raise_for_status()
    return r.json()["secret"]


token = fetch_secret("repo:github.com/mozilla-releng/balrog:coveralls")["repo_token"]
env = os.environ.copy()
env["COVERALLS_REPO_TOKEN"] = token
env["CIRCLECI"] = "1"
subprocess.run(["ls", "-l", str(FETCHES_DIR)])
subprocess.run(["coveralls", f"--submit={FETCHES_DIR}/coveralls.out"], check=True, env=env)
