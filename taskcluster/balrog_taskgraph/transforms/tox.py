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
        env = job.pop("env", {})
        job["worker"].setdefault("env", {}).update(env)
        yield job
