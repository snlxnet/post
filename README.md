> post simplifies publishing your markdown files

## Status
The project is depreacted. It was created for publishing my own site, I've since come up with [a better solution](https://snlx.net/mk-api)

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

