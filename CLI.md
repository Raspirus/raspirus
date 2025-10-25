# Raspirus CLI Mode

Raspirus supports a comprehensive command-line interface (CLI) that allows you to use the application without a graphical interface. This is particularly useful for automation, scripting, and server environments.

## Table of Contents
- [Available Arguments](#available-arguments)
- [Usage Examples](#usage-examples)
- [JSON Output](#json-output)
- [Priority and Behavior](#priority-and-behavior)

## Available Arguments

### `--nogui`
Does not attempt to open a GUI. The application will exit after completing any requested actions (like scanning or updating). Without any other arguments, the application will display a warning and exit.

**Example:**
```bash
raspirus --nogui --scan /path/to/folder
```

### `--fullscreen`
Launches the graphical interface in fullscreen mode.

**Example:**
```bash
raspirus --fullscreen
```

### `--update`
Attempts to update the virus definition rules from the CLI. This will download and compile the latest YARA rules before launching the GUI (or exit if `--nogui` is specified).

**Example:**
```bash
# Update and exit
raspirus --update --nogui

# Update then launch GUI
raspirus --update
```

### `--threads <NUMBER>`
Sets a scan thread limit, overriding the configuration file setting. This is useful for temporarily controlling system resource usage.

**Example:**
```bash
raspirus --threads 4 --scan /path/to/folder
```

### `--scan <PATH>`
Scans the specified file or directory path. The scan will be executed before the GUI launches (if GUI is enabled).

**Example:**
```bash
# Scan and exit
raspirus --scan /home/user/downloads --nogui

# Scan then launch GUI
raspirus --scan /home/user/downloads
```

### `--nolog`
Suppresses all log messages. Useful when you only want to see the final output or when piping results to other commands.

**Example:**
```bash
raspirus --scan /path/to/folder --nogui --nolog
```

### `--debug`
Sets the log level to output debug messages, even in production builds. Useful for troubleshooting and detailed output.

**Example:**
```bash
raspirus --scan /path/to/folder --nogui --debug
```

### `--json`
Outputs the scan results in JSON format. This is particularly useful for parsing the output in scripts or integrating with other tools. When used, logging is automatically suppressed to ensure clean JSON output.

**Example:**
```bash
raspirus --scan /path/to/folder --nogui --json
```

## Usage Examples

### Basic Scanning
Scan a directory and see the results:
```bash
raspirus --scan /home/user/downloads --nogui
```

### Scan with JSON Output
Scan a directory and output results as JSON for parsing:
```bash
raspirus --scan /home/user/downloads --nogui --json > results.json
```

### Update Virus Definitions
Update virus definitions without launching the GUI:
```bash
raspirus --update --nogui
```

### Scan with Custom Thread Count
Scan using only 2 threads (useful for low-resource systems):
```bash
raspirus --scan /home/user/downloads --nogui --threads 2
```

### Debug a Scan
Run a scan with detailed debug output:
```bash
raspirus --scan /path/to/suspicious/files --nogui --debug
```

### Combined Operations
Update definitions and then scan:
```bash
raspirus --update --scan /home/user/downloads --nogui --json
```

## JSON Output

When using the `--json` flag, the output follows this structure:

### Successful Scan
```json
{
  "scanned_path": "/path/to/scanned/location",
  "total_files": 150,
  "total_size": 52428800,
  "notable_files": [
    {
      "Skip": {
        "path": "/path/to/file",
        "reason": "Permission denied"
      }
    },
    {
      "Flag": {
        "path": "/path/to/suspicious/file",
        "rules": []
      }
    }
  ]
}
```

### Error
```json
{
  "status": "error",
  "error": "Error message here"
}
```

### Fields Description
- `scanned_path`: The root path that was scanned
- `total_files`: Total number of files that were indexed
- `total_size`: Total size in bytes of all scanned files
- `notable_files`: Array of files that were either flagged as suspicious or skipped
  - `Skip`: Files that couldn't be scanned (permission issues, symlinks, etc.)
    - `path`: File path
    - `reason`: Reason why the file was skipped
  - `Flag`: Files that matched virus signatures
    - `path`: File path
    - `rules`: YARA rules that matched (empty for now)

## Priority and Behavior

### Argument Priority
CLI arguments take priority over configuration file settings. For example:
- If your config specifies 8 threads, but you use `--threads 2`, the scan will use 2 threads
- If your config has a log level of "Info", but you use `--debug`, debug logging will be enabled

### Execution Order
1. **Initialization**: Config is loaded, CLI arguments are parsed
2. **Update** (if requested): Virus definitions are updated
3. **Scan** (if requested): The specified path is scanned
4. **GUI Launch**: If `--nogui` is not specified, the GUI launches after any actions complete

### Exit Conditions
The application will exit without launching the GUI when:
- `--nogui` is specified with an action (`--update` or `--scan`)
- `--nogui` is specified without any actions (with a warning)
- An error occurs during an action and `--nogui` is specified

## Tips and Best Practices

1. **Use `--json` for automation**: The JSON output format is stable and easy to parse
2. **Combine with shell tools**: Pipe JSON output to `jq` for filtering and formatting
3. **Set thread limits on low-power devices**: Use `--threads` to prevent system overload
4. **Update regularly**: Run `--update --nogui` in a cron job to keep definitions current
5. **Check exit codes**: The application returns appropriate exit codes for script integration
6. **Debug issues with `--debug`**: When reporting issues, include output from `--debug`

## Shell Scripting Example

Here's a complete example of using Raspirus in a shell script:

```bash
#!/bin/bash

# Update virus definitions
echo "Updating virus definitions..."
raspirus --update --nogui

# Scan downloads folder
echo "Scanning downloads folder..."
raspirus --scan ~/Downloads --nogui --json > scan_results.json

# Check if any files were flagged
FLAGGED=$(jq '[.notable_files[] | select(has("Flag"))] | length' scan_results.json)

if [ "$FLAGGED" -gt 0 ]; then
    echo "Warning: $FLAGGED suspicious file(s) found!"
    exit 1
else
    echo "No threats detected."
    exit 0
fi
```

## Getting Help

For more information about available options, use the built-in help:
```bash
raspirus --help
```

To see the version:
```bash
raspirus --version
```
