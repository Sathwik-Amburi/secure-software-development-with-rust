#!/bin/bash

if [ "$1" = "on" ]; then
    echo "The lights are on."
elif [ "$1" = "off" ]; then
    echo "The lights are off."
elif [ "$1" = "dim" ]; then
    echo "The lights are dim."
else
    echo "Invalid input. Please use 'on', 'off', or 'dim'."
fi
