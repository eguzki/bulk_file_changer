# Bulk image Exif metadata

Add date to image metadata when missing.

Date parsed from file name.

```bash
Add date to image Exif metadata when missing

Usage: set_date_image_metadata [OPTIONS] <PATH>

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
