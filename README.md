**Task Parser**
================

This Rust program is a simple parser for natural language dates in task descriptions. It takes a command-line argument, splits it into a task and an optional date, and formats the task according to the date.

**Usage**
--------

To use this program, save it to a file (e.g., `task_parser.rs`) and then run it from the command line:
```bash
cargo run -- "some task [@@ natural language date]"
```
For example:
```bash
cargo run -- "Buy milk @@ tomorrow"
```
The program will output the formatted task in Todoist format:
```markdown
- [ ] #task Buy milk [[due::2023-07-20]]
```
**Command-Line Options**
----------------------

* The first command-line argument is the task description.
* If a date is specified (e.g., `@ tomorrow`), it will be parsed and included in the formatted task.

**Error Handling**
-----------------

If no date is specified, or if the date cannot be parsed, the program will output an error message and not include a date in the formatted task.

**Obsidian Tasks Format**
------------------

The program formats tasks according to [Obsidian Tasks](https://publish.obsidian.md/tasks/Introduction) syntax:

* A hyphen (`-`) at the beginning of each line
* A checkbox (`[ ]`) to indicate a task without a due date
* The task description itself, preceded by `#task`
* An optional date in square brackets (`[[due::YYYY-MM-DD]]`)
