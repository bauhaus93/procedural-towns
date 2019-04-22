import logging
import os

logger = logging.getLogger()

def create_last_names(text_filepath):
    logger.info("Creating list of last names from '{}'".format(text_filepath))

    name_set = set()
    with open(text_filepath, "r") as f:
        for line in f.read().split():
            if line.isalpha():
                name_set.add(line.capitalize().rstrip())
    logger.info("Found {} last names".format(len(name_set)))

    names = sorted(list(name_set))

    NAMES_FILE = os.path.abspath("names_last.txt")
    
    with open(NAMES_FILE, "w") as f:
        for name in names[:-1]:
            f.write(name + "\n")
        f.write(names[-1])
    logger.info("Wrote last names to '{}'".format(NAMES_FILE))
   