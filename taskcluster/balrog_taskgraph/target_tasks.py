# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from taskgraph.target_tasks import _target_task, get_method


def filter_for_github_release():
    pass


@_target_task("release")
def target_tasks_release(full_task_graph, parameters, graph_config):
    default_target = get_method("default")
    def filter(task, parameters):
        return 
        return task.attributes.get("build-type", "") == "release"

    return [l for l, t in full_task_graph.tasks.items() if filter(t, parameters)]

