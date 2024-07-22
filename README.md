# Bulk image metadata setter

Add date to image metadata when missing.

Date parsed from file name.

```bash
Usage: set_date_image_metadata <PATH>

Arguments:
  <PATH>  The path to the file to read

Options:
  -h, --help  Print help
```

Display the last modification time of FILE

```
date -r <filename> "+%d-%m-%Y %H:%M:%S"
```
