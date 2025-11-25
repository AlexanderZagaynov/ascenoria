#!/usr/bin/env python3
"""
Helper script to update the GitHub project status for completed issues.

The script targets the user project "AlexanderZagaynov / project 2" and
sets the "Status" field to "Done" for issues that already carry the
"codex:done" label. You can also pass specific issue numbers on the
command line to restrict which items are updated.

Requires a GitHub personal access token in the GITHUB_TOKEN environment
variable with access to the repository and the project. No additional
third-party dependencies are needed.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.request
from typing import List, Optional, Tuple

OWNER = "AlexanderZagaynov"
REPO = "ascenoria"
PROJECT_NUMBER = 2
STATUS_FIELD_NAME = "Status"
STATUS_DONE_VALUE = "Done"
GITHUB_API = "https://api.github.com"


def _require_token() -> str:
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        raise SystemExit("GITHUB_TOKEN is required to update project status")
    return token


def _request(url: str, *, token: str, method: str = "GET", payload: Optional[dict] = None) -> dict:
    data: Optional[bytes] = None
    headers = {
        "Authorization": f"Bearer {token}",
        "User-Agent": "ascenoria-status-script",
    }

    if payload is not None:
        data = json.dumps(payload).encode()
        headers["Content-Type"] = "application/json"

    request = urllib.request.Request(url, data=data, headers=headers, method=method)
    with urllib.request.urlopen(request) as response:
        return json.loads(response.read().decode())


def _graphql(query: str, variables: dict, *, token: str) -> dict:
    body = {"query": query, "variables": variables}
    return _request(f"{GITHUB_API}/graphql", token=token, payload=body)


def _fetch_project_metadata(*, token: str) -> Tuple[str, str, str]:
    query = """
    query($owner: String!, $number: Int!) {
      user(login: $owner) {
        projectV2(number: $number) {
          id
          fields(first: 20) {
            nodes {
              ... on ProjectV2SingleSelectField {
                id
                name
                options { id name }
              }
            }
          }
        }
      }
    }
    """

    result = _graphql(query, {"owner": OWNER, "number": PROJECT_NUMBER}, token=token)
    project = result["data"]["user"]["projectV2"]
    if not project:
        raise SystemExit(f"Project {PROJECT_NUMBER} for {OWNER} was not found")

    project_id = project["id"]
    status_field_id = None
    status_done_option = None

    for field in project["fields"]["nodes"]:
        if field and field.get("name") == STATUS_FIELD_NAME:
            status_field_id = field["id"]
            for option in field.get("options", []):
                if option.get("name") == STATUS_DONE_VALUE:
                    status_done_option = option["id"]
            break

    if not status_field_id or not status_done_option:
        raise SystemExit("Status field or Done option not found on project")

    return project_id, status_field_id, status_done_option


def _closed_codex_done_issues(*, token: str) -> List[dict]:
    url = f"{GITHUB_API}/repos/{OWNER}/{REPO}/issues?state=closed&labels=codex:done&per_page=100"
    issues = _request(url, token=token)
    return [issue for issue in issues if "pull_request" not in issue]


def _issue_project_item_id(issue_node_id: str, project_id: str, *, token: str) -> Optional[str]:
    query = """
    query($issueId: ID!) {
      node(id: $issueId) {
        ... on Issue {
          projectItems(first: 20, includeArchived: false) {
            nodes {
              id
              project { id }
            }
          }
        }
      }
    }
    """

    result = _graphql(query, {"issueId": issue_node_id}, token=token)
    items = result["data"]["node"].get("projectItems", {}).get("nodes", [])
    for item in items:
        if item.get("project", {}).get("id") == project_id:
            return item.get("id")
    return None


def _update_status(project_id: str, item_id: str, field_id: str, status_option_id: str, *, token: str) -> None:
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
      updateProjectV2ItemFieldValue(input: {
        projectId: $projectId,
        itemId: $itemId,
        fieldId: $fieldId,
        value: { singleSelectOptionId: $optionId }
      }) {
        projectV2Item { id }
      }
    }
    """

    _graphql(
        mutation,
        {"projectId": project_id, "itemId": item_id, "fieldId": field_id, "optionId": status_option_id},
        token=token,
    )


def _parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Update project status for closed codex issues")
    parser.add_argument(
        "issues",
        metavar="ISSUE",
        nargs="*",
        type=int,
        help="Optional issue numbers to update. Defaults to all closed issues with the codex:done label.",
    )
    return parser.parse_args()


def main() -> None:
    args = _parse_args()
    token = _require_token()

    project_id, status_field_id, status_done_option = _fetch_project_metadata(token=token)

    if args.issues:
        issue_numbers = set(args.issues)
        url_template = f"{GITHUB_API}/repos/{OWNER}/{REPO}/issues/{}"
        selected: List[dict] = []
        for number in sorted(issue_numbers):
            selected.append(_request(url_template.format(number), token=token))
        issues = selected
    else:
        issues = _closed_codex_done_issues(token=token)

    print(f"Updating {len(issues)} issue(s) to Status={STATUS_DONE_VALUE} on project {PROJECT_NUMBER}...")

    for issue in issues:
        issue_number = issue["number"]
        issue_node_id = issue.get("node_id")
        if not issue_node_id:
            print(f"- #{issue_number}: missing node id, skipping")
            continue

        item_id = _issue_project_item_id(issue_node_id, project_id, token=token)
        if not item_id:
            print(f"- #{issue_number}: not present on project, skipping")
            continue

        _update_status(project_id, item_id, status_field_id, status_done_option, token=token)
        print(f"- #{issue_number}: status updated to {STATUS_DONE_VALUE}")


if __name__ == "__main__":
    main()
