# kintone-oauth-token-rs

A command to get a OAuth token for Kintone.

**This command uses a self-signed certificate, so please use this at your own risk.**

## Requirements

- If you use Chrome, you have to change the setting to allow insecure accesses.
    - Open `chrome://flags`
    - Enable `#allow-insecure-localhost`
    - I recommend disabling the setting again after you've gotten an OAuth token.
- Set the following environment variables.
    - `KINTONE_BASE_URL`
    - `KINTONE_OAUTH_CLIENT_ID`
    - `KINTONE_OAUTH_CLIENT_SECRET_ID`
    - `KINTONE_OAUTH_SCOPE`
    - See the details on the page.
        - https://developer.kintone.io/hc/en-us/articles/360001562353-How-to-add-OAuth-clients

## How to use

You can download the binary on the GitHub release page.

```
./kintone-oauth-token
```

## LICENSE

- [MIT](LICENSE)