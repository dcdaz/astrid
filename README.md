# astrid
Tiny app to execute qemu VMs from YAML configs

## Astrid Configuration

Astrid use a simple config file called `astrid.yml` placed on "Cofig Folder" for Linux systems will be `~/.config/astrid.yml` to allow configure VM Configs path

```yml
vms_path: /home/{USER}/vms
```

## VM Configuration

A VM configuration file is in `yml` format, similar to q proper qemu command

```yml
qemu_arch: x86_64
enable_kvm: true
boot: menu=on
cdrom: /home/{USER}/{ANY_PATH}/OS.iso
drive: /home/{USER}/{ANY_PATH}/Image.qcow2
memory: 4G
cpu: host
vga: virtio
display: sdl,gl=on
```

## How to run it

### Create an Image

```bash
astrid -c ImageName.qcow2 ImageSize
```

> Sample `astrid -c debian.qcow2 40G`

### List configs

#### Execute

```bash
astrid -l
```
#### Result

```bash
debian.yml
haiku.yml
...
```

### Run a config

To run a config just provide name of `yml` file

```bash
astrid -r debian
```

> Command above wil launch a qemu instance with your VM in it

### Print help

```bash
astrid -h
```

