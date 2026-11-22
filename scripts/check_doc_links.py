#!/usr/bin/env python3
"""Check repository-local Markdown links without third-party packages."""
from __future__ import annotations
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

MARKDOWN_LINK = re.compile(r"!?\[[^\]]*\]\((?:<([^>]+)>|([^\s)]+))(?:\s+[^)]*)?\)")
HTML_LINK = re.compile(r"""(?:href|src)\s*=\s*["']([^"']+)["']""", re.IGNORECASE)

def local_target(source: Path, destination: str, root: Path) -> Path | None:
    destination = destination.strip()
    if not destination or destination.startswith("#"):
        return None
    parsed = urlsplit(destination)
    if parsed.scheme or parsed.netloc:
        return None
    raw_path = unquote(parsed.path)
    if not raw_path:
        return None
    target = root / raw_path.lstrip("/") if raw_path.startswith("/") else source.parent / raw_path
    target = target.resolve()
    if target.is_dir():
        candidates = (target / "README.md", target / "index.md")
        target = next((candidate for candidate in candidates if candidate.is_file()), target)
    return target

def extract_destinations(text: str) -> list[str]:
    fence = chr(96) * 3
    lines = []
    in_fence = False
    for line in text.splitlines():
        if line.lstrip().startswith((fence, "~~~")):
            in_fence = not in_fence
            continue
        if not in_fence:
            lines.append(line)
    text = "\n".join(lines)
    text = re.sub(chr(96) + r"[^" + chr(96) + r"]*" + chr(96), "", text)
    destinations = []
    for match in MARKDOWN_LINK.finditer(text):
        destinations.append(match.group(1) or match.group(2))
    destinations.extend(HTML_LINK.findall(text))
    return destinations

def check_file(markdown_file: Path, root: Path) -> list[str]:
    errors = []
    content = markdown_file.read_text(encoding="utf-8")
    for destination in extract_destinations(content):
        target = local_target(markdown_file, destination, root)
        if target is not None and not target.exists():
            errors.append(f"{markdown_file.relative_to(root)}: broken link '{destination}'")
    return errors

def main() -> int:
    root = Path(__file__).resolve().parent.parent
    markdown_files = sorted(path for path in root.rglob("*.md")
                            if ".git" not in path.parts and "target" not in path.parts)
    errors = [error for path in markdown_files for error in check_file(path, root)]
    if errors:
        print("\n".join(errors), file=sys.stderr)
        print(f"Found {len(errors)} broken local documentation link(s).", file=sys.stderr)
        return 1
    print(f"Checked local links in {len(markdown_files)} Markdown file(s).")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
