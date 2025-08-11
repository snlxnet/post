> post simplifies publishing your markdown files

## Installation
todo

## Usage
File: `~/vault/hello.md`
```markdown
---
post: "[[blog]]"
---

Hello, world!
```

File: `~/vault/job-issue-512.md`
```markdown
---
post: "[[work notes]]"
---

## Issue description
![[attachment.png]]
(...)
```

All paths are absolute from the vault root (so `attachment.png` is in `~/vault/attachment.png`).

Shell:
```bash
> post ~/vault blog
~/vault/hello.md

> post ~/vault "work notes"
~/vault/job-issue-512.md
~/vault/attachment.png

> post ~/vault blog \
  rsync -av --files-from=- ~/blog/content/
```

