# Rust Chess AI & Game

Welcome! This is a chess game made with Rust in which you can play against an AI. The Rust program is compiled to web assembly and is running [on this website](https://x2dtu.github.io/rust-chess/) where you can play against the AI yourself! I hope you have fun!

The frontend is made using Yew, a rust framework made for creating reliable and efficient web applications. Yew is modeled after React.js which makes it easier than other frameworks to pick up and get started. 

The AI uses the following features in order to play at about a level of 1700 elo[^1]:
* Minimax base search algorithm
* Alpha-beta pruning
* Sophisticated move ordering which boosts
  * Checks
  * Captures (with better captures being boosted more over worse captures)
  * 'Killer' moves
  * Historical, early killer moves over later ones
* Sophisticated evaluation function taking into account
  * Material counts for both players
  * King safety
  * Optimal piece locations
  * Castling rights
* Search extensions
* Quiescence searching
* Transposition table
* Opening book preparation

I hope you like my chess AI. More features are planned for the future, but if you have any suggestions, feel free to let me know by either making an issue on this repository or emailing me at michaelga<at>vt<dot>edu.

[^1]: This chess AI was pitted up against chess.com's computer players. In my testing, it was able to beat bots consistently up to 1600 elo, then was a bit more even with wins and losses at 1700 elo, and consistently lost to the 1800 elo bot. 1700 elo makes this AI in the 98th percentile of players according to https://www.chess.com/leaderboard/live/rapid 
