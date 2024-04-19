# justfile

build_docker:
    sudo docker build -t libcachesim .

run_docker:
    sudo docker run -v "$HOME/Downloads/cachelib-workload/libcachesim:/workload:ro" --name libcachsim_docker -it libcachesim