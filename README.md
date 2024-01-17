# Proxy Converter

### Description

A rust script to help print out proxies for mtg cards.

### How to use

1. Download your decklist images from [Moxfield](https://www.moxfield.com/).
2. Copy the zip file into the `decks` directory.
3. Change the `name` constant at the beginning of the `main` function at line 109 of `main.rs`. The name may inculde the `.zip` extension or you can leave it off, but if the name is wrong, or you don't change it, the program will choose the first `.zip` file it finds in the `decks` directory.
4. Run the program with either `cargo run` or through your IDE.
5. Open the `output` directory, look for `[deckname].html`, and open it with your browser.
6. Print the browser page to pdf. Make sure to choose portrait mode, no margins, and turn on background graphics.
7. You may delete the output files and the input file once you are done.
