# UI
- [ ] Make scoring rules obvious in the UI
- [ ] Able to see your point total during game play
- [ ] Let the player know when they fail to connect over websocket and the reason.
  - The two main reasons  would be:
  (1) The server intentionally refused because of a duplicate player id. 
      - They opened a second browser tab
      - They are a hacker
  (2) Unintentional connection issues:
    - server is down
    - network failure
    - loss of internet

# Game Features
- [ ] Multi room support
- [ ] Prompts submitted by players?
- [ ] Head 2 head mode (Quiplash) instead of Judging mode (Apples 2 Apples)
- [ ] Make scoring penalties optional

# Technical
- [ ] reset websocket connection in response to change of player id.
- [ ] Make Player id into Signal<String> instead of Signal<Option<String>>
- [ ] Fix leptos browser console warnings
- [ ] Make client messages more incremental, prevent resending the entire state.

## Multi room client
- [ ] Start new room button
- [ ] Copy room url to clipboard button
