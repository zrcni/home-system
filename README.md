1. start Dev Container in current folder

2. Open (vscode) terminal within the docker container

3. create docker network, which is used by all docker compose files within the project

```bash
docker network create home-system
```

4. run docker compose up in the project that you're developing, or at the root

```bash
docker compose up
```

5. Open another terminal within the docker container, and start the project you're developing (e.g "cargo run", "go run .", "npm start")
