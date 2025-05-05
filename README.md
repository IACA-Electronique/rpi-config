# IACA OS RPI CONFIG

## ℹ️ Purpose

Allow to configure `/boot/firmware/config.txt` from command line.

## 📖 Usage

### Show help

```bash
os rpi-config -h
```


### Set parameter

```bash
os rpi-config set <param> <value>
```

Specify section (by default it's `all`)

```bash
os rpi-config set -s <section> <param> <value>
```

Availabe sections :
* `all`
* `cm4`
* `cm5`
* `pi3`
* `pi4`
* `pi5`

### Remove parameter


```bash
os rpi-config del -s <param> <value>
```

### Load preset

It's possible to directly load entire preconfigured `config.txt` file from remote server.

```bash
os rpi-config preset load <preset name>
```

### Backup preset

At each preset command, original `config.txt` is kept.

```bash
os rpi-config preset backup
```

In case where are few backup, command promp to user to choose version.