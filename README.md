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

### Run example

```bash
$ endara -vvv ~/tmp/pics/
[2024-10-01T21:54:30Z DEBUG endara] Reading path: "/home/eguzki/tmp/pics/"
[2024-10-01T21:54:30Z DEBUG endara::stats] /home/eguzki/tmp/pics/428934165_278732.jpg 428934165_278732.jpg
num_files: 1
num_files_with_datetime_tag: 0
num_files_failed_tag_parsing: 0
num_files_missing_datetime_tag: 1
num_files_failed_filename_parsing: 1
num_files_successfully_tagged: 0
num_files_failed_tagging: 0
filenames_tag_unparseable: []
filenames_name_unparseable:
- /home/eguzki/tmp/pics/428934165_278732.jpg
filenames_name_untaggeable: []
```

### Misc

Display Exif metadata

```
sudo apt install exif
exif picture.jpg
```

