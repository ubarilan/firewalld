# Firewalld - The future of firewalls 😉
## For educational purposes only!

## Servicifing the firewall

<br>

```sh
# Default port
export PORT=4242
```

### **REALLLLLY oneliner**

```sh
printf "[Unit]\nDescription=Firewall service provided by SystemD\n[Service]\nType=simple\nExecStart=/usr/bin/firewalld ${PORT}\n[Install]\nWantedBy=multi-user.target" > /etc/systemd/system/firewalld.service; systemctl enable firewalld --now
```

Write this to

```
/etc/systemd/system/firewalld.service
```

```
[Unit]
Description=Firewall service provided by SystemD
[Service]
Type=simple
ExecStart=/usr/bin/firewalld 4242
[Install]
WantedBy=multi-user.target
```

### Oneliner

```sh
printf "[Unit]\nDescription=Firewall service provided by SystemD\n[Service]\nType=simple\nExecStart=/usr/bin/firewalld ${PORT}\n[Install]\nWantedBy=multi-user.target" > /etc/systemd/system/firewalld.service
```

```sh
systemctl enable firewalld --now
```

## How to build if you are retarded

```sh
cargo build --release
```

_outfile will be located in `./target/release/`_
