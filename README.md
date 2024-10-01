# Endara

CLI tool to add `DateTimeOriginal` Exif Tag to image metadata.

When exif tag is not found, new tag will be set from date parsed from the file name.

```bash
Add date to image Exif metadata when missing

Usage: endara [OPTIONS] <PATH>

Arguments:
  <PATH>  The path to the base directory

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

Display Exif metadata

```
sudo apt install exif
exif picture.jpg
```
