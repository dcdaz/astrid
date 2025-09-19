# astrid
Tiny app to execute qemu VMs from YAML configs

## How to run it

### Create an Image

```bash
astrid -c ImageName.img ImageSize
```

> Sample `astrid -c debian.img 40G`

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

```bash
astrid -n debian
```

> Command above wil launch a qemu instance with your VM in it

### Print help

```bash
astrid -h
```

