# Patterns


## Sequence - logical `AND`

Do this then do that.

- If child 1 succeeds, proceed to child 2
- If child 1 fails, yield failure

## Fallback - logical `OR`

Try this, but if it fails do that

- If child 1 succeeds, yield success
- If child 1 fails, proceed to child 2