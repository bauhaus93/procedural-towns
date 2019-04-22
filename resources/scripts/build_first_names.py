import logging
import os
from zipfile import ZipFile

logger = logging.getLogger()

def create_first_names(zip_filepath):
    logger.info("Creating list of first names from '{}'".format(zip_filepath))

    male_set = set()
    female_set = set()

    with ZipFile(zip_filepath, "r") as zip_file:
        for filename in zip_file.namelist():
            if filename.endswith(".txt"):
                with zip_file.open(filename, "r") as txt_file:
                    new_males, new_females = get_names(txt_file)
                    male_set = male_set.union(new_males)
                    female_set = female_set.union(new_females)
    
    logger.info("Found {} male first names".format(len(male_set)))
    logger.info("Found {} female first names".format(len(female_set)))
    
    names_male = sorted(list(male_set))
    names_female = sorted(list(female_set))

    MALES_FILE = os.path.abspath("names_first_male.txt")
    FEMALES_FILE = os.path.abspath("names_first_female.txt")

    with open(MALES_FILE, "w") as f:
        for name in names_male[:-1]:
            f.write(name + "\n")
        f.write(names_male[-1])
    logger.info("Wrote male first names to '{}'".format(MALES_FILE))

    with open(FEMALES_FILE, "w") as f:
        for name in names_female[:-1]:
            f.write(name + "\n")
        f.write(names_female[-1])
    logger.info("Wrote female first names to '{}'".format(FEMALES_FILE))

def get_names(txt_file):
    names_male = set()
    names_female = set()
    for line in txt_file.read().split():
        try:
            name, gender, _count = line.decode("ISO 8859-1").split(",")
        except ValueError:
            logger.warn("Could not split line to n/g/c: '{}'".format(line))
        else:
            if gender == 'M':
                names_male.add(name)
            elif gender == 'F':
                names_female.add(name)
            else:
                logger.warn("Unknown gender found: {}".format(gender))
    return names_male, names_female

