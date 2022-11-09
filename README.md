# honjitsu

Create daily report for you

## lint/format

```bash
cargo fmt
cargo clippy --fix
```

## set environment variables using GitHub CLI

You need to set the following secrets

- `TOGGL_EMAIL`
  - Your toggl account's email
- `TOGGL_PASSWORD`
  - Your toggl account's password
- `TODOIST_TOKEN`
  - Your todoist account's API token
- `TWITTER_CONSUMER_KEY`
  - Twitter API key
- `TWITTER_CONSUMER_SECRET`
  - Twitter API secret
- `TWITTER_ACCESS_TOKEN`
  - Twitter Access token
- `TWITTER_ACCESS_TOKEN_SECRET`
  - Twitter Access token secret

Write the pairs of the key and its value in `.env`

```bash
gh secret set -f .env
```

## references

- [Toggl Track](https://developers.track.toggl.com/docs/)
- [todoist API](https://developer.todoist.com/sync/v9/)
- [GitHub CLI secrets](https://cli.github.com/manual/gh_secret_set)
- [Twitter API](https://developer.twitter.com/en/docs/twitter-api)
- [Twitter API samples](https://github.com/twitterdev/Twitter-API-v2-sample-code)
