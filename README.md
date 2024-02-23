# QuakeLogParserCliApp

A cli app for parsing local Quake log files

Check Time Management of this Project at: https://wakatime.com/projects/QuakeLogParserCliApp

# Instructions

Build Docker Image:

```docker build -t quake-log-parser-cli:v1 .```

Run by Docker Compose:

```docker-compose up```

# Author Notes

This App basically serve as cli for quake_log_parser_lib crate.

Args: 

- **config-file:** config file name, should be placed in same binary folder.
- **log-file:** log file name to be parsed, should be placed in same binary folder.
- **enable-kill-by-means:** if set as true includes kill means in match stats
- **self-kill-score-increase:** if set as true when a player kill itself it should increase player kill score.
- **enable-kill-by-means:** if set as true when a player is killed it kills score got decreased.

Callbacks:

 - **warnings:** throw warning messages at stdout.
 - **success:** throw the Parsing Output as an info message at stdout.
 
