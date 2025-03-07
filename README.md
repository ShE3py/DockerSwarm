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
- `worker`: le back-end (lib pour casser des MD5, et bin pour l’exposer via un WebSocket).
- `spy`: le moniteur (auto-scaling, expose le nombre de serveurs via un WebSocket)

## Bogues

Le navigateur Internet se connecte parfois à un worker qui travaille, la solution miracle
est de faire <kbd>Ctrl</kbd> + <kbd>F5</kbd>.

S’il y a écrit « Connexion… », c’est qu’il attend de faire l’handshaking avec le worker.

S’il dit qu’il y a 0 workers actifs/disponibles, c’est qu’il n’est pas encore
connecté/n’a pas encore reçu de message du spy.

## Auto-scaling

Le nombre de workers augmente si plus aucun worker n'est disponible.

Le nombre de workers diminue si la plus de moitié est disponible,
et qu'il y a 4+ workers disponibles.

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
