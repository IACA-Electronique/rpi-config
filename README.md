# IACA OS RPI CONFIG

## ℹ️ Purpose

Allow to configure `/boot/firmware/config.txt` from command line.

## 📖 Usage

### Show help

```bash
rpi-config -h
```


### Set parameter

```bash
rpi-config set <param> <value>
```

Specify section (by default it's `all`)

```bash
rpi-config set -s <section> <param> <value>
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
rpi-config del -s <param> <value>
```

### Load preset

It's possible to directly load entire preconfigured `config.txt` file from presets directory.
Presets directory need to be called `presets` and need to be located at WORKING_PATH. This directory is not automatically created.

```bash
rpi-config preset load <preset name>
```

### Backup

At each write command (except `restore`), original `config.txt` is copied in directory.
This directory path is build as following : `<config_file_path>.dir`.

####  1. List backup available

```bash
rpi-config list-backup 
```

#### 2. Restore

```bash
rpi-config restore <index>
```

> `index` is given in step 1.

### Specify config filepath

By default, it's `/boot/firmware/config.txt` but it can be changed by option `-f config_file_path`.

Exemple :

```bash
rpi-config -f /boot/conf.txt set -s <section> <param> <value>
```

In case where there are few backups, command prompts the user to choose a version.

## 🛠️ Development

**Requirements :**
* cargo (>= 1.86)

### Build

```bash
cargo build 
```

### Build release

```bash
cargo build --release
```

### Run tests

```bash
cargo test
```

## 📜 Licence
`GNU GENERAL PUBLIC LICENSE v3`