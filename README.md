# Finder

Finds files and copies them somewhere.

## Display possible options

```bash
./finder
```

Output:

```bash
error: the following required arguments were not provided:
  --file-list <FILE_LIST>
  --source-dir <SOURCE_DIR>
  --target-dir <TARGET_DIR>

Usage: finder --file-list <FILE_LIST> --source-dir <SOURCE_DIR> --target-dir <TARGET_DIR>
```

## Dry-run

```bash
./finder --file_list files.txt --source_dir /mnt/a --target_dir /mnt/b
```

Use `--disable-dry-run` to copy the files.