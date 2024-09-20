<h1>
  <img src="https://github.com/user-attachments/assets/b081cbb0-0d25-44c6-8dde-87fd111442a0" width="30px">
  Dino Game
</h1>

By <a href="//commons.wikimedia.org/w/index.php?title=User:Carlosbaraza&amp;action=edit&amp;redlink=1" class="new" title="User:Carlosbaraza (page does not exist)">Carlos Baraza</a> - <span class="int-own-work" lang="en">Own work</span> / <a rel="nofollow" class="external free" href="https://github.com/carlosbaraza/web-assembly-logo">https://github.com/carlosbaraza/web-assembly-logo</a>, <a href="http://creativecommons.org/publicdomain/zero/1.0/deed.en" title="Creative Commons Zero, Public Domain Dedication">CC0</a>, <a href="https://commons.wikimedia.org/w/index.php?curid=56494100">Link</a>

<hr>
Have you ever played the google chrome dino game? This is my own version of it, developed in rust using the WASM (web assembly) technology.

## Develop by your own

### My recomendations - Docker

If you want to develop your own version of this game, you should get docker installed, so you don't have to worry about configure anything or make shure that this will work on your machine, is everythig already configured for you. Besides, in a virtualized environment, errors like "it works on my machine" can be avoidable. I made this to be really simple to just "enter and use" with no need for configuration. <!--fuck the history that was deleted because of him, how could he do that with me?-->

<img src="https://upload.wikimedia.org/wikipedia/commons/7/70/Docker_logo.png?20240428132226" alt="docker logo" width="300px"><br/>
<a href="https://commons.wikimedia.org/wiki/File:Docker_logo.png">Docker, Inc.</a>, <a href="http://www.apache.org/licenses/LICENSE-2.0">Apache License 2.0</a>, via Wikimedia Commons

### Start the server

To start the server, use: `run`, this command will make shure that everything necessary to make this project work is being executed and is working correctly.

## Game Aesthetics

- The game must be in black and white
- The textures in pixel art style
- No size variation of the same texture, size variation require a new texture
- No fancy pause menu, just a 'esc' button that unpause the game
- Score and life info must be in the right bottom corner
- No fancy physics
- The jump is very precise and cant be to much or to less
- The terrain should have some variation, it can't be plain, but it can't have to high variation
- The font should be pixelated
- The dinossour must have 3 lifes, and for each life lost the sprite will blink for 3 seconds

## Todo

Those are some stuff that I think that would be cool to add in this project

* [ ]  Set the start time (day / night) of the game to the equivalent to the user's theme
* [ ]  Create a grid to prevent impossible situations for the player, making the game playable
* [ ]  Create a progression difficulty, making the game harder over time
* [ ]  Create a pause menu acessible with 'esc'
* [ ]  Add multplayer compatibility
<!--* [ ]  Create a ranking system to compare with friends <-- that's cool but I need to review (AI recommendation)-->
* [ ]  Pause the game when the user leave the page and play a 5 seconds countdown when the user return to restart the game, pressing esc on this menu will stop the countdown
* [ ]  Save the user's progress if they close the page
* [ ]  Add other dinos to the game
* [ ]  Create a procedural generation of the world, not only obstacles, but the game enviroment as well
* [ ]  Add moon cycle, with different sizes and textures
* [ ]  Add a life system
* [ ]  Add a God mode, make it contains infinite life, a custom texture for the dinossaur and enable double jump

I'm not using any libray, and this project is on a begginig stage, finishing this can take a long time

## History

I had a problem and needed to delete and create the repo again, so the history of this repo before '11 ago 2024' was deleted, but this afected just one week of progression.
