## Rust formatting rules (MUST FOLLOW)
- Do NOT reflow or wrap existing code just because you touched nearby lines.
- Preserve existing line breaks and indentation unless the change requires it.
- Keep short function signatures on ONE line when they fit within 140 columns:
  `pub fn name(&self, arg: Type) -> Result<_, _> {`
- When editing a function, minimize diff size: change only the necessary lines.
- Never auto-wrap parameter lists unless the signature exceeds 140 columns.
