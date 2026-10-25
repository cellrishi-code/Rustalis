# Filesystem: first milestone

Rustalis currently has a fixed-capacity, allocation-free RAM filesystem prototype in
`kernel/src/fs.rs`. It is a bring-up primitive for use before the kernel heap,
block-device layer, and persistent filesystem driver exist.

## Current API

- `RamFs::<N>::new()`: create a filesystem with N file slots.
- `create(path)`: create an empty file at an absolute path.
- `write(path, bytes)`: replace a file's contents.
- `read(path)`: borrow file contents as bytes.
- `remove(path)`: remove a file and release its slot.
- `path_at(index)` and `file_count()`: enumerate occupied entries.

Paths must be absolute, non-root paths. Empty components, trailing slashes, `.`,
and `..` are rejected. Each path is limited to 64 bytes and each file to 256
bytes. Capacity is selected at compile time. Errors are returned as `FsError`;
the implementation does not allocate or perform disk I/O.

## Important limitations

This is **not yet a persistent filesystem or a complete VFS**. Data is held in
kernel memory and disappears on reboot. The current prototype does not implement
directories as independent objects, file descriptors, permissions, concurrent
access, block devices, mounting, or a disk format. It must not be used as a
security boundary.

## Next milestones

1. Add a VFS trait and mount-point/path resolution.
2. Add directory metadata and file metadata (size, type, timestamps, permissions).
3. Define a block-device interface with bounds-checked sector I/O.
4. Implement a small on-disk filesystem behind the VFS.
5. Add file descriptors and syscalls only after process isolation and syscall
   validation are in place.
6. Test storage parsing against malformed images and run integration tests in QEMU.

The kernel boot path performs a small smoke check by creating and reading
`/welcome.txt`. Unit tests cover CRUD behavior, invalid paths, duplicate names,
capacity limits, and oversized writes.
