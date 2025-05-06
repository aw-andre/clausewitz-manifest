# Clausewitz Manifest

A simple, easily extensible web server for serving requests to Paradox Script databases made by [Clausewitz Parser][https://github.com/aw-andre/clausewitz-parser].
This server allows users to search for any key or value in Paradox Script gamefiles and get a list lexicographically sorted by value containing all of its "parents" and "siblings," allowing for quickly searching for instances of modifiers or event lists.

## Getting Started

### Dependencies

- Rust
- Postgres

### Running this Project

1. Clone this project locally.

```bash
git clone https://github.com/aw-andre/clausewitz-parser
```

2. Set your DATABASE_URL to be the same as the Postgres database used by Clausewitz Parser. This can be done in the shell, but the preferred way to do this is in a .env file in the project directory:

```bash
DATABASE_URL='<url>'
```

3. Edit lines 35-37 of `templates/index.html` to include the following:

```html
<a href="form/<gamecolumn>">text</a>
```

where gamecolumn is the exact game name you provided to Clausewitz Parser and text is whatever text you would like (this text is what will be displayed). Add any number of games in your database as you would like.

4. (Optional) If you want, you can add any styles or scripts you would like to any of the html files in `templates`.

5. Run `cargo run --release` and the server should be accessible on 127.0.0.1:8000.

6. Host this server however you would like.

## Contributing

If you can make any improvements to this project, please don't hesitate to send a pull request. Any contribution is welcome!
Some areas of possible improvement include:

- UI design
