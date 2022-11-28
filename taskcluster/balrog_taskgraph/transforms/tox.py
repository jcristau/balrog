# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
"""
Tox-specific transforms
"""

from taskgraph.transforms.base import TransformSequence

transforms = TransformSequence()


@transforms.add
def update_env(config, jobs):
    for job in jobs:
        pr_number = config.params.get("pull_request_number", "")
        env = job.pop("env", {})
        env["CI_PULL_REQUEST"] = str(pr_number)
        job["worker"].setdefault("env", {}).update(env)
        yield job


@transforms.add
def set_command_context(config, jobs):
    for job in jobs:
        if isinstance(job["run"]["command"], str):
            repo_url = config.params["head_repository"]
            if not repo_url.endswith(".git"):
                repo_url += ".git"
            job["run"].setdefault("command-context", {})["head_repo"] = repo_url
        yield job
