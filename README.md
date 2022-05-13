# Firewalld - The future of firewalls

## Servicifing the firewall

<br>

### **REALLLLLY oneliner**

```sh
printf "[Unit]\nDescription=Firewall service provided by SystemD\n[Service]\nType=simple\nExecStart=/usr/bin/firewalld\n[Install]\nWantedBy=multi-user.target" > /etc/systemd/system/firewalld.service; systemctl enable firewalld --now
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
ExecStart=/usr/bin/firewalld
[Install]
WantedBy=multi-user.target
```

### Oneliner

```sh
printf "[Unit]\nDescription=Firewall service provided by SystemD\n[Service]\nType=simple\nExecStart=/usr/bin/firewalld\n[Install]\nWantedBy=multi-user.target" > /etc/systemd/system/firewalld.service
```

```sh
systemctl enable firewalld --now
```

## How to build if you are retarded

```sh
cargo build --release
```

_outfile will be located in `./target/release/`_
