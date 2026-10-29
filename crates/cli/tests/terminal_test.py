"""Standard-library PTY regression driver (Unix only), no third-party packages."""
import os
import pty
import select
import subprocess
import sys
import tempfile
import termios
import time
from pathlib import Path


class Terminal:
    def __init__(self, *args, cwd=None):
        self.master, self.slave = pty.openpty()
        self.before = termios.tcgetattr(self.slave)
        self.process = subprocess.Popen([sys.argv[1], *args], stdin=self.slave,
                                        stdout=self.slave, stderr=self.slave, cwd=cwd)
        self.text = b""
        self.cursor = 0

    def until(self, needle):
        needle = needle.encode()
        deadline = time.monotonic() + 8
        while needle not in self.text[self.cursor:]:
            remaining = deadline - time.monotonic()
            assert remaining > 0, self.text.decode(errors="replace")
            if select.select([self.master], [], [], remaining)[0]:
                self.text += os.read(self.master, 65536)
            assert self.process.poll() is None or needle in self.text[self.cursor:], self.text
        self.cursor = self.text.index(needle, self.cursor) + len(needle)

    def send(self, text):
        os.write(self.master, text.encode())

    def finish(self):
        assert self.process.wait(timeout=5) == 0, self.text
        assert termios.tcgetattr(self.slave) == self.before, "terminal settings leaked"

    def close(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait()
        os.close(self.master)
        os.close(self.slave)


with tempfile.TemporaryDirectory(prefix="grimm-keys-") as folder:
    # Untimed legacy games use immediate keys too; redirected demos are unchanged.
    t = Terminal()
    try:
        t.until("1. Continue")
        t.send(":quit\r")  # Colon commands are unavailable outside the Esc menu.
        time.sleep(0.03)
        assert t.process.poll() is None
        t.send("1")
        t.until("Ending: Demo complete")
        t.finish()
    finally:
        t.close()

    path = Path(folder) / "timer.yaml"
    path.write_text("""schema_version: 2
id: terminal-timer
version: 1
title: Key test
time: {}
start: first
nodes:
  - id: first
    content: [{kind: narration, text: First deadline.}]
    decision: {limit_ms: 350, timeout: {target: second}}
    choices: [{id: choose, label: First choice, target: done}]
  - id: second
    content: [{kind: narration, text: Second deadline.}]
    decision: {limit_ms: 2000, timeout: {target: expired}}
    choices: [{id: choose, label: Second choice, target: done}]
  - id: done
    content: [{kind: narration, text: Accepted}]
    ending: {id: done, title: Chosen}
  - id: expired
    content: [{kind: narration, text: Missed}]
    ending: {id: expired, title: Expired}
""")
    # A single digit, without a newline, commits immediately; exit restores tty.
    t = Terminal("play", str(path))
    try:
        t.until("1. First choice")
        t.send("1")
        t.until("Ending: Chosen")
        t.finish()
    finally:
        t.close()

    # Esc pauses the deadline; Save/Load/Continue each use one key. Menu keys
    # cannot become story choices and menu time cannot spend the decision budget.
    t = Terminal("play", str(path), cwd=folder)
    try:
        t.until("1. First choice")
        t.send("\x1b")
        t.until("Session menu")
        t.send("2")
        t.until("Saved to")
        t.until("Session menu")
        checkpoint = Path(folder) / ".grimm-saves/story-terminal-timer.json"
        import json
        save = json.loads(checkpoint.read_text())
        assert save["node_id"] == "first"
        assert save["temporal"]["decision"]["phase"] == "paused"
        remaining = save["temporal"]["decision"]["remaining_ms"]
        assert 0 < remaining <= 350
        time.sleep(0.5)
        t.send("3")
        t.until("Loaded")
        t.until("Session menu")
        t.send("2")
        t.until("Saved to")
        t.until("Session menu")
        restored = json.loads(checkpoint.read_text())
        assert restored["temporal"]["decision"]["remaining_ms"] == remaining
        assert restored["node_id"] == "first"
        assert b"Second deadline" not in t.text
        t.send("1")
        t.until("1. First choice")
        t.send("1")
        t.until("Ending: Chosen")
        t.finish()
    finally:
        t.close()

    # Failure to load leaves the menu and pending interaction intact.
    t = Terminal("play", str(path), cwd=folder)
    try:
        t.until("1. First choice")
        t.send("\x1b")
        t.until("Session menu")
        checkpoint.write_text("invalid checkpoint")
        t.send("3")
        t.until("invalid JSON save")
        t.until("Session menu")
        t.send("\x1b")
        t.until("1. First choice")
        t.send("1")
        t.until("Ending: Chosen")
        t.finish()
    finally:
        t.close()

    # With no blocking read, an unattended timer enters its successor directly.
    t = Terminal("play", str(path))
    try:
        t.until("1. Second choice")
        assert b"Press Enter to acknowledge" not in t.text
        t.send("1")
        t.until("Ending: Chosen")
        t.finish()
    finally:
        t.close()

    # Real story: reading leads to a compact decision that activates by itself,
    # then single-key choices and command editing remain available.
    t = Terminal("play", sys.argv[2])
    try:
        for _ in range(3):
            t.until("Press Enter when finished reading...")
            t.send("\r")
        t.until("1. Answer the captain")
        assert b"12.0s" in t.text
        assert b"Press Enter" not in t.text[t.text.index(b"The relay snaps open"):]
        t.send("\x1b")
        t.until("Session menu")
        t.send("1")
        t.until("1. Answer the captain")
        for expected in ["1. Compare the signal", "1. Keep the decoded"]:
            t.send("1")
            t.until(expected)
        t.send("1")
        for _ in range(2):
            t.until("Press Enter when finished reading...")
            t.send("\r")
        t.until("1. Send the decoded")
        t.send("1")
        t.until("Ending: A Voice That Answers")
        t.finish()
    finally:
        t.close()

    # Menus during unread narration do not acknowledge it, and Quit is one key.
    t = Terminal("play", sys.argv[2], cwd=folder)
    try:
        t.until("Press Enter when finished reading...")
        t.send("\x1b")
        t.until("Session menu")
        t.send("2")
        t.until("Saved to")
        t.until("Session menu")
        checkpoint = Path(folder) / ".grimm-saves/story-the-silent-signal.json"
        assert json.loads(checkpoint.read_text())["flow"]["phase"] == "presenting"
        t.send("1")
        t.until("Press Enter when finished reading...")
        assert b"A carrier tone rises" not in t.text
        t.send("\x1b")
        t.until("Session menu")
        t.send("4")
        t.finish()
    finally:
        t.close()

    # A saved paused session starts in the menu, not at unusable story choices.
    t = Terminal("play", sys.argv[2], "--load", str(checkpoint))
    try:
        t.until("Session menu")
        t.send("1")
        t.until("Press Enter when finished reading...")
        t.send("\x03")
        t.finish()
    finally:
        t.close()
