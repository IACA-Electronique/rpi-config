# INI FILE CONFIG PROGRAM

## ℹ️ Purpose

Program to configure `.ini` file. It's useful for OS automation.

## 📖 Usage

### Show help

```bash
ini-config -h
```


### Set parameter

```bash
ini-config set <param> <value>
```

Specify a section (by default, it's `all`)

```bash
ini-config set -s <section> <param> <value>
```

### Remove parameter

```bash
ini-config del -s <param> <value>
```

### Load preset

It's possible to directly load entire preconfigured `.ini` file from *presets directory*.
By default, *presets directory* need to be called `presets` and need to be located at `WORKING_PATH`. This directory is not automatically created.
But it can be specified with `-d` option.
```bash
ini-config preset load <preset name>
```


```bash
ini-config preset -d <presets dir path> load <preset name>
```

### Backup

At each write command (except `restore`), original `.ini` file is copied in directory.
This directory path is build as following : `<config_file_path>.dir`.

####  1. List backup available

```bash
ini-config list-backup 
```

#### 2. Restore

```bash
ini-config restore <index>
```

> `index` is given in step 1.

### Specify config filepath

By default, it's `/boot/firmware/config.txt` but it can be changed by option `-f config_file_path`.

Exemple :

```bash
ini-config -f /boot/conf.txt set -s <section> <param> <value>
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