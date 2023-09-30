#!/bin/bash

# Define ANSI escape codes for text colors
RED="\033[31m"
GREEN="\033[32m"
BLUE="\033[34m"
RESET="\033[0m"  # Reset text color to default

# Print text in red, green, and blue colors
echo -e "${BLUE}Welcome to Vim101.\nAutograding...${RESET}"
if cmp -s "src/animals.rs" "src/animals_correct.rs"; then
  echo -e "${BLUE}\nChecking src/animals.rs:\n${GREEN} Correct!${RESET}"
else
  echo -e "${BLUE}\nChecking src/animals.rs:\n${RED} Incorrect.${RESET}"
fi
NUM_MATCHES=$(egrep "^([A-Za-z]+(\d*|\.)?( |-)?)*,((Electric|Ground|Fire|Water|Bug|Grass|Normal|Psychic|Poison|Flying|Water|Fairy|Ground|Rock|Ghost|Water|Fighting|Ice|Steel|Dark|Dragon)?,){2}(\d+,){7}([1-6],)(True|False)$" src/pokemon.csv|wc -l)
result=$((NUM_MATCHES / 800))
RESULT=$(echo "scale=3; $NUM_MATCHES / 800" | bc)
if [ "$RESULT" = "1.000" ]; then
  echo -e "${BLUE}Checking src/pokemon.csv\n ${Green}Percent Correct is: $RESULT${RESET}"
else
  echo -e "${BLUE}Checking src/pokemon.csv\n ${RED}Percent Correct is: $RESULT${RESET}"
fi

