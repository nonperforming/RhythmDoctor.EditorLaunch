# RhythmDoctor.EditorLaunch

Launch the [Rhythm Doctor](https://store.steampowered.com/app/774181/Rhythm_Doctor/) level editor from a `.rdlevel`/`.rdzip` file, like in the [standalone editor](https://giacomopc.itch.io/rdle).
This is a BepInEx 5 plugin.

> [!NOTE]
> The game will close and relaunch in order to launch with Steam.

> [!TIP]
> You may want to install [BepInEx.SplashScreen](https://github.com/BepInEx/BepInEx.SplashScreen) or [enable the console](https://docs.bepinex.dev/articles/user_guide/configuration.html#configuring-bepinex) to check if the game is running

## TODO
- Improve startup times
  - Find if there is a way to influence load priority
    - Prefixing the filename of the plugin dll with "0000000" doesn't seem to work
    - Nor does putting the plugin in a folder with "000000" or "aaaaa"
- See if there is a way to pass a launch option to `steam://launch` without having to drop a file somewhere.
  - steam -applaunch 774181 <params>
    - Requires access to steam executable
