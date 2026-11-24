# Record: vhs .github/assets/demo.tape
# Edit the steps below and re-run vhs after the tool changes.

Output .github/assets/demo.gif

Set Shell "bash"
Set FontSize 20
Set Width 1100
Set Height 560
Set Padding 20
Set Framerate 20
Set TypingSpeed 50ms

Hide
Type 'REPO_ROOT="$PWD"'
Enter
Wait
Type 'export HOME="$(mktemp -d)"'
Enter
Wait
Type 'export XDG_CONFIG_HOME="$(mktemp -d)"'
Enter
Wait
Type 'BIN="$(mktemp -d)"'
Enter
Wait
Type `printf '#!/bin/sh\nexec node "%s/dist/cli.js" "$@"\n' "$REPO_ROOT" > "$BIN/hinge"`
Enter
Wait
Type 'chmod +x "$BIN/hinge"'
Enter
Wait
Type 'export PATH="$BIN:$PATH"'
Enter
Wait
Type 'WORK="$(mktemp -d)"'
Enter
Wait
Type 'cd "$WORK"'
Enter
Wait
Type `printf '{"headers":{"authorization":"Bearer DEMO-PLACEHOLDER-TOKEN"}}' > hinge-session.json`
Enter
Wait
Type "clear"
Enter
Wait
Type "clear"
Enter
Sleep 200ms
Show

Type "hinge --help"
Sleep 400ms
Enter
Wait
Sleep 2s

Sleep 2s
Type "hinge tier"
Sleep 400ms
Enter
Wait
Sleep 2s

Sleep 1s
Type "hinge auth import hinge-session.json"
Sleep 400ms
Enter
Wait
Sleep 2s

Sleep 2s

Sleep 3s
