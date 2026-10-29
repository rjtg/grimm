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
    def __init__(self, *args):
        self.master, self.slave = pty.openpty()
        self.before = termios.tcgetattr(self.slave)
        self.process = subprocess.Popen([sys.argv[1], *args], stdin=self.slave,
                                        stdout=self.slave, stderr=self.slave)
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

    # No old blocking read/Enter barrier in terminal mode. A partially entered
    # command remains bound to its original revision across asynchronous expiry.
    t = Terminal("play", str(path))
    try:
        t.until("1. First choice")
        t.send(":pa")
        t.until("1. Second choice")
        t.send("use\r")
        t.until("Input from the previous prompt was ignored")
        t.until("1. Second choice")
        assert b"Paused. Use" not in t.text
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
        t.send(":pause\r")
        t.until("Paused. Use")
        t.send(":resume\r")
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

    # Ctrl-C is handled as host exit in raw mode, with restoration on return.
    t = Terminal("play", sys.argv[2])
    try:
        t.until("Press Enter when finished reading...")
        t.send("\x03")
        t.finish()
    finally:
        t.close()
