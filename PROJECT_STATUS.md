# Project status helper

Use `scripts/update_project_status.py` to update the GitHub project board status for completed issues.

## Prerequisites
- Python 3.8+
- `GITHUB_TOKEN` environment variable with access to repository issues and the project board.

## Usage
- Update all closed issues with the `codex:done` label:
  ```bash
  GITHUB_TOKEN=<token> python3 scripts/update_project_status.py
  ```
- Update specific issues by number:
  ```bash
  GITHUB_TOKEN=<token> python3 scripts/update_project_status.py 12 34
  ```

The script targets the user project `https://github.com/users/AlexanderZagaynov/projects/2` and sets the `Status` field to `Done` for matching issue items.
