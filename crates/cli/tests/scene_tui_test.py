"""Exercise the real scene TUI through a PTY, including RAII terminal teardown."""
import fcntl
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
    (root / "text.yaml").write_text("""narratives:
  room.intro: 'Snow gathers. Lea waits.'
  end.end: 'The lamp is lit. Safe at last.'
actions:
  room.finish: 'Light the lamp'
""")
    master, slave = pty.openpty()
    saved = termios.tcgetattr(slave)
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 40, 150, 0, 0))
    process = subprocess.Popen([binary, "play-scene", "story.yaml", "text.yaml"], cwd=root,
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

    def wait_for(text, timeout=8):
        target = text.encode()
        limit = time.monotonic() + timeout
        while time.monotonic() < limit:
            if target in received or text in screen_text():
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
        # Input remains usable after narrow/full-width inspector resize.
        os.write(master, b"\x1bOQ")
        os.write(master, b"\x1b")
        wait_for("SESSION")
        os.write(master, b"2")
        wait_for("Saved")
        os.write(master, b"3")
        wait_for("Loaded")
        os.write(master, b"1")  # resume
        os.write(master, b"1")  # choose, never Enter
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

    # An error after entering raw/alternate screen must restore the terminal too.
    (root / "text.yaml").write_text("narratives: {}\nactions: {}\n")
    master, slave = pty.openpty()
    saved = termios.tcgetattr(slave)
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
    broken = subprocess.Popen([binary, "play-scene", "story.yaml", "text.yaml"], cwd=root,
                              stdin=slave, stdout=slave, stderr=slave)
    try:
        broken.wait(timeout=5)
        output = bytearray()
        while select.select([master], [], [], 0)[0]:
            output.extend(os.read(master, 65536))
        assert broken.returncode != 0, output
        assert b"missing text binding" in output, output
        assert b"\x1b[?1049l" in output, output
        assert termios.tcgetattr(slave) == saved
    finally:
        if broken.poll() is None:
            broken.kill()
            broken.wait()
        os.close(master)
        os.close(slave)

    # EOF never completes prose and never abandons the next authored timeout.
    (root / "story.yaml").write_text("""schema_version: 2
id: eof_chain
version: 1
title: EOF chain
start: first
contexts:
  - id: first
    cues:
      - {kind: timer, at_ms: 0, set_deadline_ms: 100, visible: true}
      - {kind: narrative, narrative_id: wait, duration_ms: 500}
    timeout: {goto: second}
  - id: second
    cues:
      - {kind: timer, at_ms: 0, set_deadline_ms: 500, visible: true}
      - {kind: narrative, narrative_id: wait, duration_ms: 500}
    timeout: {goto: end}
  - id: end
    terminal: true
    cues: [{kind: narrative, narrative_id: done, duration_ms: 1}]
""")
    (root / "text.yaml").write_text("narratives: {first.wait: FIRST_WAIT, second.wait: SECOND_WAIT, end.done: AUTHORED_END}\nactions: {}\n")
    for mode in ("--auto-reading", "--manual-reading"):
        result = subprocess.run([binary, "play-scene", "story.yaml", "text.yaml", mode], cwd=root,
                                input=b"", stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5)
        assert result.returncode == 0, result.stderr
        for text in (b"FIRST_WAIT", b"SECOND_WAIT", b"AUTHORED_END"):
            assert text in result.stdout, result.stdout
    pending = subprocess.Popen([binary, "play-scene", "story.yaml", "text.yaml", "--auto-reading"], cwd=root,
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    # Synchronize with the actual successor presentation, while stdin still
    # has the first revision's pending read. No guessed sleep decides ordering.
    prefix = bytearray()
    limit = time.monotonic() + 5
    while b"SECOND_WAIT" not in prefix and time.monotonic() < limit:
        if select.select([pending.stdout], [], [], 0.05)[0]:
            prefix.extend(os.read(pending.stdout.fileno(), 65536))
    assert b"FIRST_WAIT" in prefix, prefix
    assert b"SECOND_WAIT" in prefix, prefix
    output, errors = pending.communicate(b":menu\n", timeout=5)
    output = prefix + output
    assert pending.returncode == 0, errors
    assert b"Stale input ignored" in output, output
    assert b"AUTHORED_END" in output, output
    assert b"session menu" not in output.lower(), output

    # Legacy schema 1 has no clock until an explicitly completed introduction.
    # Its future scheduled warning must not make EOF spin forever at frozen T=0.
    (root / "story.yaml").write_text("""schema_version: 1
id: legacy_eof
version: 1
title: Legacy EOF
start: room
contexts:
  - id: room
    cues:
      - {kind: narrative, narrative_id: intro}
      - {kind: timer, start_ms: 100}
      - {kind: narrative, narrative_id: warning, at_ms: 50}
    timeout: {goto: end}
  - id: end
    terminal: true
    cues: []
""")
    (root / "text.yaml").write_text("narratives: {room.intro: LEGACY_INTRO, room.warning: UNSEEN_WARNING}\nactions: {}\n")
    legacy = subprocess.run([binary, "play-scene", "story.yaml", "text.yaml"], cwd=root,
                            input=b"", stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5)
    assert legacy.returncode == 0, legacy.stderr
    assert b"LEGACY_INTRO" in legacy.stdout
    assert b"UNSEEN_WARNING" not in legacy.stdout
