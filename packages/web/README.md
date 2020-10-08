# Web Setup

## Prerequisites

Install [node and npm](https://nodejs.org)

## Install Dependencies

```sh
npm ci
```

## Development Server

```sh
npm run dev
```

## Check types, run lints, etc

```sh
npm run check
```

## Production Build

```sh
npm run build
```

## i18n

If you change, add, or remove any UI strings you will need to extract and
recompile the i18n data:

```sh
npm run lingui:extract
npm run lingui:compile
```
