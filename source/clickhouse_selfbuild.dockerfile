FROM clickhouse:lts-jammy
USER root
RUN apt-get update && apt-get install -y --no-install-recommends libcap2-bin && rm -rf /var/lib/apt/lists/*
RUN setcap "cap_ipc_lock=+ep cap_sys_nice=+ep" /usr/bin/clickhouse
USER clickhouse