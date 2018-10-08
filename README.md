# Introduction

`tapbpm` was written to provide a quick, command line bpm tap-tempo utility.
It is used by [cdj_mode](https://github.com/theaugy/cdj_mode) to set bpm. It also functions as a standalone program.

# Usage

Run the program, and press the Enter key at regular intervals. `tapbpm` will use the average of these intervals and convert it to bpm.

Example output:

```
~ $ tapbpm
Tap!

Tap!

146.71

146.38

146.28

146.94

148.51

148.55

147.79
^C
~ $
```

