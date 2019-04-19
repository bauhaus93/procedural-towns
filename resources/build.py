import os
import logging

from utility import setup_logger
from build_first_names import create_first_names
from build_last_names import create_last_names

# source for first names: https://www.ssa.gov/oact/babynames/limits.html 
ZIP_FIRST_NAMES_PATH = os.path.abspath("names.zip")

# source for last names: https://www.census.gov/topics/population/genealogy/data/1990_census/1990_census_namefiles.html
TXT_LAST_NAMES_PATH = os.path.abspath("last_names.txt")

if __name__ == "__main__":
    setup_logger()
    logger = logging.getLogger()

    create_first_names(ZIP_FIRST_NAMES_PATH)
    create_last_names(TXT_LAST_NAMES_PATH)



