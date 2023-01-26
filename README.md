# jira-automation

Small tool to help automate some common tasks via the JIRA api.
Intended to be used as part of a CI pipeline.
### Features
* create release
* delete release
* list releases
* update release
* update issue (currently just the fixVersion field)
* transition issue
* JQL search for issues
* list available transitions for issue

### Helptext
```
Usage: jira-automation [OPTIONS] --auth-token <AUTH_TOKEN> --user-email <USER_EMAIL> --base-jira-url <BASE_JIRA_URL> [COMMAND]

Commands:
  list-releases           list and optionally filter releases
  create-release          create a new release
  delete-release          delete a release and optionally update tickets to point to a different one
  update-release          update a release
  list-issue-transitions  list possible transitions for specified issue
  transition-issue        transition issue
  update-issue            update an issue
  search-issues           jql search for issues
  help                    Print this message or the help of the given subcommand(s)

Options:
  -a, --auth-token <AUTH_TOKEN>        jira personal access token
      --output-format <OUTPUT_FORMAT>  how returned items should be formatted [default: csv] [possible values: csv, json]
  -u, --user-email <USER_EMAIL>        email address the auth token belongs to
  -b, --base-jira-url <BASE_JIRA_URL>  base url of the jira instance ex http://potato.atlassian.net
  -h, --help                           Print help information
  -V, --version                        Print version information


```