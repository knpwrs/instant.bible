# instant.bible

[![Web App](https://img.shields.io/badge/visit-web-orange)][web]
[![iOS App](https://img.shields.io/badge/get-app%20store-blue)][apple]
[![Android App](https://img.shields.io/badge/get-play%20store-green)][google]
[![License](https://img.shields.io/badge/license-MIT-yellow.svg)][mit]
[![Badges](https://img.shields.io/badge/badges-5-orange.svg)][shields]

[instant.bible][web] is a Bible search engine which returns results instantly
while you type.

[instant.bible][web] is available on the following platforms:

* Web: [instant.bible][web]
* iOS: [App Store][apple]
* Android: [Play Store][google]

## This Repository

This repository is a monorepo containing the codebases for the search engine as
well as all of the individual applications. All application and engine code is
in the [`./packages`](./packages/) directory. See `README.md` in that directory
for further instructions on getting set up for development.

Make sure to run `npm ci` in this root repository to get some common tooling
set up. You should also install [`direnv`] and run `direnv allow` (after reading
`.envrc`) to make sure your environment is set up correctly.

Commit messages should be formatted to match the [Conventional Commits][cc]
format. `direnv allow` will set up a git hook for this repository to help you
format your commit messages properly.

## License

**MIT**

[`direnv`]: https://direnv.net/
[apple]: https://apps.apple.com/us/app/id1533722003 "instant.bible on the Apple App Store"
[cc]: https://www.conventionalcommits.org/en/v1.0.0/
[google]: https://play.google.com/store/apps/details?id=bible.instant "instant.bible on the Google Play Store"
[mit]: https://opensource.org/licenses/MIT "MIT License"
[shields]: https://blog.burntsushi.net/about/
[web]: https://instant.bible "instant.bible Website"
