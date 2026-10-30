# WHITEOUT walkthrough — spoilers

The generator and radio can be repaired in either order. Treating Jonas and
restoring heat first lets him walk the ridge later. Radio first buys an early
bearing but leaves him weak; a prepared safety line can still save everyone.
There is no required character death on either order.

All routes below are available in the shipped game. Equipment and phase choices
have semantic IDs for testing; ordinary play uses their visible numbered labels.

## Preparation

Finish the opening and carry Jonas into the stores. For the prepared routes:

1. Put on the thermal liner.
2. Treat Jonas with the medical kit.
3. Take the copper braid, ceramic mug and spanner.
4. Pack the safety line and flare.
5. Take the spare heat-pack battery.
6. Read the log, then leave.

The log explains the repeating call and the de-icer overload. It also unlocks
the isolator action without a separate inspection. If you skip it, inspect the
generator controls instead. These are discoverable alternatives, not a password.

## Four names on the manifest — full rescue

Choose engineering first. After the briefing, deploy the heat-pack battery for
another sixty seconds. With Jonas treated, two ordinary three-second latch
attempts open the cover. Without his help, three ordinary attempts are needed.
Isolate the automatic de-icer, prime/start the generator and acknowledge all three
startup beats. Return to dispatch; warm your hands if needed and save here.

Go to communications. Thread the copper through the mug handle to make the
isolated antenna adapter. Transmit the crew manifest. Lea can now release the
wheel. Return, gather the crew, and coordinate both rescues. With power, a call,
treatment and the prepared equipment, all four survive. Reading the log adds
the completed mystery explanation after the rescue.

The same preparation also permits a radio-first rescue: keep the safety line,
complete either radio solution, then repair the generator and coordinate. The
line compensates for Jonas's longer unheated wait.

## Insulated lever — alternate major-puzzle route

In engineering, combine the spanner and copper braid, then lever the latch open.
This takes five seconds of work instead of six with Jonas or nine without him,
and avoids repeated strain. Isolate the de-icer and start the generator as usual.

That copper is gone. Lea cannot build the mug jumper, so hand-align the dish and
send the signal. She remains at the mast. Keep the safety line from preparation:
coordinated rescue uses it to secure the antenna and carry Jonas, saving everyone.
This route proves the alternate solution has a later resource consequence.

## The empty place — partial survival and player-caused death

Follow the lever/manual-radio route. At the ridge, choose to carry Jonas and
leave Lea to release the manual antenna. Alex, Mara and Jonas survive; Lea dies.
Even if you brought a line, this explicit one-person rescue decision does not
execute the better coordinated plan for you.

A distinct partial outcome is reachable by doing radio first without the safety
line, then repairing generation. Choose to help Lea. Jonas's treated ankle is
not enough after that unheated delay; he does not reach the crossing. Doing the
same help-Lea decision after **generator first**, with treatment and a call,
lets him walk and saves all four. Tests compare these later consequences.

If you abandon repairs, the packed flare can still lead survivors to a coastal
patrol. Jonas cannot make that exposed retreat. The partial ending is a losing
objective with survivors, not an accidental nonterminal deadlock.

## WHITEOUT — catastrophic failure

At the crisis, stay inside despite the intake warning, or let the forty-five
seconds expire. Drifts choke the intake and erase the crossing; all four die.
Repairs alone do not make an unsafe building a permanent shelter. Without a
completed call or a flare fallback, an improvised one-person escape also ends
in station loss.

## Reproduce actual CLI routes without sleeps

The checked-in `.input` files target the real CLI, engine and text renderer.
Use the explicit manual clock for deterministic runs:

```sh
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --manual-clock < examples/whiteout/playthroughs/full-rescue.input
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --manual-clock < examples/whiteout/playthroughs/lever-rescue.input
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --manual-clock < examples/whiteout/playthroughs/partial-survival.input
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --manual-clock < examples/whiteout/playthroughs/station-lost.input
```

These scripts use `@qualified.action`, blank Enter acknowledgements and
`:time MS`. Padding Enter lines are harmless when no narrative is pending; they
allow all conditional narrative beats to finish. These host testing controls
are piped input, not free-text gameplay or terminal menu commands.

Expected final headings are respectively **FOUR NAMES ON THE MANIFEST**, **FOUR
NAMES ON THE MANIFEST**, **THE EMPTY PLACE**, and **WHITEOUT**. Native deterministic
tests additionally assert the different canonical survivor flags and state.
The integration suite also starts a separate CLI process with `--load` from a
saved dispatch checkpoint and completes the remaining adventure.
