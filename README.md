# DockerSwarm
[![CI](https://github.com/ShE3py/DockerSwarm/actions/workflows/docker.yaml/badge.svg?event=push)](https://github.com/ShE3py/DockerSwarm/actions/workflows/docker.yaml)

Lancement :
```
# modprobe br_netfilter
# docker swarm init --advertise-addr lo
# docker compose build
# docker stack deploy -c docker-compose.yaml 64
# docker stack ps 64
```

Puis le site web sera disponible à l’adresse suivante :  
<http://localhost:8080/>

## Crates

- `hive`: le front-end.
- `worker`: le back-end (lib pour casser des MD5, et bin pour l'exposer via un WebSocket).

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
