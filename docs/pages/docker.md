# docker

> Container platform for building and running applications.
> See also: docker-compose, podman, kubectl
> Keywords: container, image, build, run, deploy, virtualization

- Run a container:

`docker run -it ubuntu bash`

- Run detached (background):

`docker run -d nginx`

- Run with port mapping:

`docker run -p 8080:80 nginx`

- Run with volume mount:

`docker run -v /host/path:/container/path image`

- Run with environment variable:

`docker run -e MY_VAR=value image`

- List running containers:

`docker ps`

- List all containers (including stopped):

`docker ps -a`

- Stop a container:

`docker stop container_id`

- Remove a container:

`docker rm container_id`

- List images:

`docker images`

- Pull an image:

`docker pull ubuntu:22.04`

- Build image from Dockerfile:

`docker build -t myimage:tag .`

- View container logs:

`docker logs container_id`

- Follow logs:

`docker logs -f container_id`

- Execute command in running container:

`docker exec -it container_id bash`

- Copy files from container:

`docker cp container_id:/path/file ./local/path`

- Remove all stopped containers:

`docker container prune`

- Remove unused images:

`docker image prune`

- Remove all unused resources:

`docker system prune -a`

- Inspect container details:

`docker inspect container_id`

## Flags

- `-d, --detach`: Run in background
- `-it`: Interactive with TTY
- `-p`: Port mapping host:container
- `-v`: Volume mount
- `-e`: Environment variable
- `--name`: Assign container name
- `--rm`: Remove container when it exits
- `-f`: Follow logs, or force remove

## Common Commands

- `docker run`: Create and start container
- `docker ps`: List containers
- `docker images`: List images
- `docker build`: Build image
- `docker pull`: Download image
- `docker push`: Upload image
- `docker exec`: Run command in container
- `docker logs`: View container logs
- `docker stop/start/restart`: Control containers
- `docker rm/rmi`: Remove containers/images

## Exit Codes

- `0`: Success
- `1`: Generic error
- `125`: Docker daemon error
- `126`: Command cannot be invoked
- `127`: Command not found
- `137`: Container killed (OOM or SIGKILL)
- `143`: Container terminated (SIGTERM)

## Common Errors

- "permission denied" - Add user to docker group or use sudo
- "port is already allocated" - Port in use, choose different
- "no space left on device" - Run docker system prune
- "Cannot connect to Docker daemon" - Start docker service
