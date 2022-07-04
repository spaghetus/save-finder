# a program that finds all of your steam game saves and puts them together in one place

have you ever:

1. decided not to include your steam games in your backups because they're massive and replaceable
2. lost your data
3. had to restore from backup
4. realized that half of your save data is gone because your games don't support cloud saves and they kept their save data next to their game files?

if so, this program is for you

put it in a cron job like this and include the target directory in your backups, hopefully it will work ok

```cron
0 * * * * save-finder ~/.save-finder
```

## how to add support for new games

if a game's save directory isn't automatically detected by the default scan, you can create a custom script. the script should have the same name as the game's steam directory, plus `.py`. see `OMORI.py` for an example. (OMORI actually works with the default script, but it's included as an example.)

please commit your contributed scripts back to upstream, they help make the program work better