docker build -t cargo_ntfy .
echo "killing old containers"
ssh worker "docker rm -f \$(docker ps -a -q --filter ancestor=cargo_ntfy) 2>/dev/null || true"
echo "uploading image"
docker save cargo_ntfy | pv -s $(docker image inspect cargo_ntfy --format='{{.Size}}') | bzip2 | ssh worker "docker load"
echo "creating new instance"
ssh worker "docker run -d --restart=unless-stopped --name cargo_ntfyr -e scanning_interval=60  cargo_ntfy"
