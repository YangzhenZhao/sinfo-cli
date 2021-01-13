<h1 align="center">sinfo-cli</h1>
<div align="center">
 <strong>
   通过命令行获取系统信息
 </strong>
</div>
<div align="center">

<img src="https://github.com/Yangzhenzhao/sinfo-cli/workflows/CI/badge.svg" />
 
</div>


### Command-line options


```bash
USAGE:
    sinfo-cli [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -m, --memory     Get memory message
    -n, --name       Get name message
    -V, --version    Prints version information
```

### Examples

```bash
$ sinfo-cli -m
total:     8.19GB
available: 2.23GB

$ sinfo-cli -n
os_name:   Darwin 19.6.0
host_name: 192.168.0.103

$ docker run -ti --rm -v ${PWD}:/volume nocilantro/alpine-rust-cn cargo run --release -- -n
os_name:   Alpine Linux 3.12.3
host_name: 8670a9243620

$ docker run -ti --rm -v ${PWD}:/volume nocilantro/alpine-rust-cn cargo run --release -- -m
total:     1.94GB
available: 1.05GB

$ docker run -ti --rm -v ${PWD}:/volume nocilantro/muslrust-cn cargo run --release -- -n
os_name:   Ubuntu 16.04
host_name: ebe5e43c19dc
```

