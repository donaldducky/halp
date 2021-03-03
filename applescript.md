To see what's available, in Script Editor `File > Open Dictionary` (shift+cmd+o)

Some examples for interacting with Google Chrome:
https://gist.github.com/donaldducky/0f1fc53d56704bf47822c659a78b2f17

```bash
# get the active tab of the frontmost window
osascript -e 'tell application "Google Chrome" to return {title, URL} of active tab of front window'

# looking up a tab by id
id=$(osascript -e 'tell application "Google Chrome" to get {active tab index} of window 1')
osascript -e "tell application \"Google Chrome\" to get {title, URL} of tab $id of window 1"
```
