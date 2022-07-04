import os
def locator(game_path):
    for root, dirs, files in os.walk(game_path):
        for name in dirs:
            if "save" in name.lower() and os.path.isdir(os.path.join(root,name)):
                return os.path.join(root, name)
    raise Exception("No save directory found by automatic search for " + game_path)