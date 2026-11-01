"""Exercise the real scene TUI through a PTY, including RAII terminal teardown."""
import fcntl
import json
import os
import pathlib
import pty
import re
import unicodedata
import select
import signal
import struct
import subprocess
import sys
import tempfile
import termios
import time

binary = str(pathlib.Path(sys.argv[1]).resolve())
with tempfile.TemporaryDirectory(prefix="grimm-tui-") as directory:
    root = pathlib.Path(directory)
    (root / "story.yaml").write_text("""schema_version: 2
id: tui_smoke
version: 1
title: TUI smoke
start: room
globals: {clue: false}
contexts:
  - id: room
    locals: {visited: true}
    cues:
      - {cue_id: intro, kind: narrative, narrative_id: intro, duration_ms: 50}
      - {kind: actions, available_actions: eligible}
    actions:
      - id: finish
        effects: [{op: set, var: global.clue, value: true}]
        outcomes: [{goto: end}]
  - id: end
    terminal: true
    cues: [{kind: narrative, narrative_id: end, duration_ms: 50}]
""")
    (root / "scenes.yaml").write_text("schema_version: 1\nid: tui_scenes\nversion: 1\nstory_id: tui_smoke\nstory_version: 1\nscenes: {room: demo.room, end: demo.end}\n")
    (root / "text.yaml").write_text("""narratives:
  room.intro: 'Snow gathers. Lea waits.'
  end.end: 'The lamp is lit. Safe at last.'
actions:
  room.finish: 'Light the lamp'
""")
    master, slave = pty.openpty()
    saved = termios.tcgetattr(slave)
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 40, 150, 0, 0))
    process = subprocess.Popen([binary, "play-scene", "story.yaml", "text.yaml", "--lifecycle", "scenes.yaml"], cwd=root,
                               stdin=slave, stdout=slave, stderr=slave)
    received = bytearray()

    def screen_text():
        # Interpret the cursor-addressed diff stream, rather than assuming that
        # ratatui writes each visible sentence as one contiguous byte string.
        cells = [[" " for _ in range(180)] for _ in range(60)]
        x = y = 0
        stream = received.decode("utf-8", errors="replace")
        tokens = re.findall(r"\x1b\[[0-?]*[ -/]*[@-~]|[^\x1b]", stream)
        for token in tokens:
            if token.startswith("\x1b["):
                body, code = token[2:-1], token[-1]
                if body.startswith("?"):
                    continue
                args = [int(n) if n else 0 for n in body.split(";")]
                if code in "Hf":
                    y = max(0, (args[0] or 1) - 1)
                    x = max(0, (args[1] if len(args) > 1 else 1) - 1)
                elif code == "J" and args[0] in (2, 3):
                    cells = [[" " for _ in range(180)] for _ in range(60)]
                elif code == "K" and y < 60:
                    for col in range(min(x, 180), 180):
                        cells[y][col] = " "
            elif token == "\r":
                x = 0
            elif token == "\n":
                y += 1
            elif token.isprintable():
                if y < 60 and x < 180:
                    cells[y][x] = token
                x += 2 if unicodedata.east_asian_width(token) in ("W", "F") else 1
        return "\n".join("".join(row) for row in cells)

    def wait_for(text, timeout=8, current=False):
        target = text.encode()
        limit = time.monotonic() + timeout
        while time.monotonic() < limit:
            if (not current and target in received) or text in screen_text():
                return
            if process.poll() is not None:
                raise AssertionError(f"exited {process.returncode}: {received!r}")
            if select.select([master], [], [], 0.05)[0]:
                received.extend(os.read(master, 65536))
        raise AssertionError(f"missing {text!r}: {screen_text()}")

    try:
        wait_for("Light the lamp")  # auto duration releases choices, no Enter
        os.write(master, b"\x1bOQ")  # F2: docked inspector
        wait_for("CUE INSPECTOR")
        wait_for("WORLD STATE")
        wait_for("clue")
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 70, 0, 0))
        process.send_signal(signal.SIGWINCH)
        wait_for("demo.room | Playing", current=True)
        # Input remains usable after narrow/full-width inspector resize.
        os.write(master, b"\x1bOQ")
        wait_for("NARRATIVE", current=True)
        os.write(master, b"\x1b")
        wait_for("SESSION", current=True)
        os.write(master, b"2")
        wait_for("Save failed")
        os.write(master, b"1")  # resume active scene
        wait_for("Playing", current=True)
        os.write(master, b"1")  # select action
        wait_for("checkpoint available", current=True)
        os.write(master, b"2")  # persistent stable boundary
        wait_for("Saved", current=True)
        os.write(master, b"3")
        wait_for("Loaded", current=True)
        os.write(master, b"1")  # prepare next resources before Begin
        wait_for("Safe at last")
        os.write(master, b"\x1b")
        time.sleep(0.1)
        os.write(master, b"4")
        process.wait(timeout=8)
        assert process.returncode == 0, received
        assert b"\x1b[?1049h" in received, "alternate screen not entered"
        # Drain final teardown bytes before checking exit sequence.
        while select.select([master], [], [], 0)[0]:
            received.extend(os.read(master, 65536))
        assert b"\x1b[?1049l" in received, "alternate screen not left"
        assert termios.tcgetattr(slave) == saved, "terminal attributes not restored"
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()
        os.close(master)
        os.close(slave)

