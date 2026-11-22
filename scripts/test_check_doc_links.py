import importlib.util
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("check_doc_links.py")
SPEC = importlib.util.spec_from_file_location("check_doc_links", MODULE_PATH)
checker = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checker)

class DocumentationLinkTests(unittest.TestCase):
    def test_resolves_relative_file(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            docs = root / "docs"
            docs.mkdir()
            target = docs / "guide.md"
            target.write_text("# Guide", encoding="utf-8")
            self.assertEqual(checker.local_target(docs / "index.md", "guide.md", root), target.resolve())

    def test_resolves_repository_root_path(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "README.md"
            target.write_text("# Project", encoding="utf-8")
            self.assertEqual(checker.local_target(root / "docs" / "x.md", "/README.md", root), target.resolve())

    def test_ignores_external_and_fragment_links(self):
        root = Path("/")
        source = root / "README.md"
        self.assertIsNone(checker.local_target(source, "https://example.com", root))
        self.assertIsNone(checker.local_target(source, "#section", root))
        self.assertIsNone(checker.local_target(source, "mailto:test@example.com", root))

    def test_extracts_markdown_and_html_links(self):
        tick = chr(96)
        fence = tick * 3
        sample = "[guide](docs/guide.md) ![image](img.png) <a href='other.md'>x</a> " + tick + "[ignored](x.md)" + tick + "\n\n" + fence + "\n[fenced](ignored.md)\n" + fence
        links = checker.extract_destinations(sample)
        self.assertEqual(links, ["docs/guide.md", "img.png", "other.md"])

if __name__ == "__main__":
    unittest.main()
