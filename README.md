# DockerSwarm
[![CI](https://github.com/ShE3py/DockerSwarm/actions/workflows/docker.yaml/badge.svg?event=push)](https://github.com/ShE3py/DockerSwarm/actions/workflows/docker.yaml)

Lancement :
```
# modprobe br_netfilter
# docker swarm init --advertise-addr lo
# docker compose build
# docker stack deploy -c docker-compose.yaml 64
```

Puis le site web sera disponible à l’adresse suivante :  
<http://localhost:8080/>

## Crates

- `hive`: le front-end.
- `worker`: le back-end (lib pour casser des MD5, et bin pour l'exposer via un WebSocket).

## Monitoring

Pour déterminer dynamiquement l'état du stack, l'on peut faire un `ps` en spécifiant
la sortie dans un format autre qu'humain :
```
# docker stack ps 64 --format json
{"CurrentState":"Running 2 minutes ago","DesiredState":"Running","Error":"","ID":"5zw0yrffuunm","Image":"hive:latest","Name":"64_hive.1","Node":"pastel","Ports":""}
{"CurrentState":"Running 2 minutes ago","DesiredState":"Running","Error":"","ID":"tlryownwrmkt","Image":"worker:latest","Name":"64_worker.1","Node":"pastel","Ports":""}
{"CurrentState":"Running 14 seconds ago","DesiredState":"Running","Error":"","ID":"6fpvp0fio3dj","Image":"worker:latest","Name":"64_worker.2","Node":"pastel","Ports":""}
{"CurrentState":"Running 14 seconds ago","DesiredState":"Running","Error":"","ID":"fz7xgpar5hrc","Image":"worker:latest","Name":"64_worker.3","Node":"pastel","Ports":""}
```

## License

Copyright (C) 2024, 2025 Lieselotte <52315535+she3py@users.noreply.github.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
