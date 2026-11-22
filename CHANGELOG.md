# Changelog

## 1.2.0

- Add `--export-set` to write editable mxGraph XML and compact, document,
  and high-resolution PNG variants from the same diagram layout.
- Render the PNG variants at 1×, 2×, and 4× by default and scale them
  proportionally for diagrams that would exceed the PNG canvas limits.

- Give the installed compiler priority over earlier user PATH entries on
  Windows, including after a reinstall.
- Move the installed compiler directory ahead of older PATH entries in
  Fish startup configuration.

## 1.1.0

- Draw repeated Drizzle connections once, including repeated loops and
  detours.
- Join matching incoming connections at a common junction with one final
  arrow. Draw each shared segment once in PNG and editable mxGraph XML.
