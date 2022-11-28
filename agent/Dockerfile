FROM python:3.9-slim

MAINTAINER jcristau@mozilla.com

WORKDIR /app

# %include agent/requirements
COPY requirements/ /app/requirements/
RUN pip install -r requirements/base.txt

# %include agent/src
COPY src/ /app/src/
# %include agent/tests
COPY tests/ /app/tests/
# %include agent/scripts
COPY scripts/ /app/scripts/
# %include agent/run.sh
# %include agent/MANIFEST.in
# %include agent/pyproject.toml
# %include agent/setup.py
# %include agent/version.json
COPY run.sh MANIFEST.in pyproject.toml setup.py version.json /app/
# test-only stuff
# %include agent/tox.ini
# %include agent/version.txt
COPY tox.ini version.txt /app/

RUN python setup.py install

# Using /bin/bash as the entrypoint works around some volume mount issues on Windows
# where volume-mounted files do not have execute bits set.
# https://github.com/docker/compose/issues/2301#issuecomment-154450785 has additional background.
ENTRYPOINT ["/bin/bash", "/app/run.sh"]
CMD ["agent"]
