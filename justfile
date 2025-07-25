#!just

build_docker:
    sudo docker build -t libcachesim_rs --file ./dockerfile_rs .

run_docker:
    sudo docker run -v "$HOME/Downloads/cachelib-workload/libcachesim:/workload:ro" -v "$(pwd):/libcachsim_repo" --rm --name libcachsim_docker -it libcachesim_rs

copy_libs:
    sudo docker run --rm -v "$(pwd):/output" libcachesim_rs sh -c "cp /libCacheSim/liblibCacheSim.a /libCacheSim/libTraceWrapper.a /output/"

run_sim:
    ./cachesim /workload/meta_kvcache_traces_1.oracleGeneral.bin.zst oracleGeneral lru 1mb,16mb,256mb,8gb

run_analysis:
    ./traceAnalyzer /workload/meta_reag.oracleGeneral.zst oracleGeneral --reqRate
