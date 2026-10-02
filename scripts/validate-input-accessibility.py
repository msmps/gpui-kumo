#!/usr/bin/env python3
"""Live Linux AT-SPI check. Run in a D-Bus session with an X11 display.

Requires PyGObject, pyatspi, xdotool and the built kumo-gallery binary.
"""
import argparse
import json
import subprocess
import time
from pathlib import Path

import gi
gi.require_version("Gtk", "3.0")
from gi.repository import Gio, GLib, Gdk, Gtk
import pyatspi


def eventually(check):
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        while GLib.MainContext.default().pending():
            GLib.MainContext.default().iteration(False)
        if check():
            return
        time.sleep(0.05)
    raise AssertionError("Timed out waiting for native accessibility update")


def walk(node):
    # This adapter's cache has a different signature from pyatspi's cache.
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=Path("target/debug/kumo-gallery"))
    args = parser.parse_args()
    bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
    bus.call_sync(
        "org.a11y.Bus", "/org/a11y/bus", "org.freedesktop.DBus.Properties", "Set",
        GLib.Variant("(ssv)", ("org.a11y.Status", "ScreenReaderEnabled", GLib.Variant("b", True))),
        None, Gio.DBusCallFlags.NONE, -1, None,
    )
    app = subprocess.Popen([str(args.binary.resolve())], stdout=subprocess.DEVNULL)
    try:
        desktop = pyatspi.Registry.getDesktop(0)

        def find(name, role=None):
            return next((n for n in walk(desktop) if n.name == name
                         and (role is None or n.getRoleName() == role)), None)

        eventually(lambda: find("Email address", "entry") is not None)
        window = subprocess.check_output(["xdotool", "search", "--name", "^GPUI Kumo$"], text=True).strip().splitlines()[-1]
        subprocess.run(["xdotool", "windowfocus", "--sync", window], check=True)
        entry = find("Email address", "entry")
        interfaces = pyatspi.listInterfaces(entry)
        assert "Text" in interfaces, interfaces
        text = entry.queryText()
        assert text.getText(0, -1) == ""
        assert text.characterCount == 0
        assert entry.queryComponent().grabFocus()
        eventually(lambda: pyatspi.STATE_FOCUSED in find("Email address", "entry").getState().getStates())

        def keys(value):
            subprocess.run(["xdotool", "key", "--clearmodifiers", value], check=True)

        def type_text(value):
            if value.isascii():
                subprocess.run(["xdotool", "type", "--clearmodifiers", "--delay", "20", value], check=True)
            else:
                # X11 key synthesis cannot reliably enter astral Unicode. Use
                # the real clipboard/paste path and serve its GTK requests.
                clipboard = Gtk.Clipboard.get(Gdk.SELECTION_CLIPBOARD)
                clipboard.set_text(value, -1)
                keys("ctrl+v")

        def click(name):
            node = find(name, "button")
            assert node is not None, name
            assert node.queryAction().doAction(0)

        type_text("café 🦀")
        eventually(lambda: text.getText(0, -1) == "café 🦀")
        assert text.characterCount == 6
        assert text.getText(3, 6) == "é 🦀"
        eventually(lambda: text.caretOffset == 6)
        bounds = text.getRangeExtents(0, 6, pyatspi.DESKTOP_COORDS)
        assert bounds[2] > 0 and bounds[3] > 0, bounds
        assert text.setSelection(0, 3, 6)
        eventually(lambda: text.getNSelections() == 1 and text.getSelection(0) == (3, 6))
        type_text("日本")
        eventually(lambda: text.getText(0, -1) == "caf日本")
        keys("ctrl+z")
        eventually(lambda: text.getText(0, -1) == "café 🦀")
        assert text.setCaretOffset(4)
        eventually(lambda: text.caretOffset == 4)
        type_text("!")
        eventually(lambda: text.getText(0, -1) == "café! 🦀")

        click("Read only")
        assert entry.queryComponent().grabFocus()
        assert text.setSelection(0, 0, 4)
        eventually(lambda: text.getSelection(0) == (0, 4))
        type_text("blocked")
        time.sleep(0.2)
        assert text.getText(0, -1) == "café! 🦀"
        click("Disable input")
        # AT-SPI reports dispatch success; current-state gating rejects mutation.
        assert text.setSelection(0, 4, 7)
        time.sleep(0.2)
        assert text.getSelection(0) == (0, 4)
        click("Enable input")
        click("Allow editing")
        click("Dark appearance")
        eventually(lambda: text.getText(0, -1) == "café! 🦀")
        assert text.getSelection(0) == (0, 4)
        click("Reset")
        eventually(lambda: text.getText(0, -1) == "" and text.caretOffset == 0)
        assert text.characterCount == 0
        assert entry.queryComponent().grabFocus()
        eventually(lambda: pyatspi.STATE_FOCUSED in find("Email address", "entry").getState().getStates())
        type_text("a" * 100 + "🦀")
        eventually(lambda: text.characterCount == 101 and text.caretOffset == 101)
        tail_bounds = text.getRangeExtents(100, 101, pyatspi.DESKTOP_COORDS)
        assert tail_bounds[2] > 0 and tail_bounds[3] > 0
        print(json.dumps({
            "result": "PASS", "interfaces": interfaces,
            "unicode_text": "café 🦀", "character_count": 6,
            "range_bounds": list(bounds), "scrolled_tail_bounds": list(tail_bounds),
            "checks": ["empty text", "Unicode reads", "caret", "selection", "bounds",
                       "keyboard replacement and undo", "read-only selection", "disabled rejection",
                       "retained node across theme/reset", "horizontal scrolling"],
            "limits": ["adapter does not expose EditableText", "no spoken-reader validation"],
        }, indent=2))
    finally:
        app.terminate()
        app.wait(timeout=5)


if __name__ == "__main__":
    main()
