# Gif to Ascii

## Overview

This is a gif converter that changes it to an ascii output (technically it's just
a text output, because it can be utf-8 if I want it to it just changes it to arbritary
characters and since rust handles utf-8 that's really the limit)

It's also the project I am using to get into rust, I've done a few other practice
things but this is the first propper piece that will *do* something.
This means that the code is probably awful, I mean really bad. I'm hoping
future Jemma will come back and refactor it, make it something that runs quickly
and easily but that's maybe a pipe dream.

## Arguments it can take

I've found the wonderous crate "clap" (yes Jemma's got the clap!) this is kinda cool
 and means taking command line arguments has never been so easy! Here are all the ones
 I take and what I expect from you

 -h --help : The help menu (bascially this)
 -V --version : The version number and stuff (I should update that right?)
 -f --file : The path to the file you want to convert, works from current dir
 -W -Width : The width of the output image (this will default to the maximum number
 of characters in your terminals width, basically filling out whatever terminal
 (looks pretty good))
 -H -Height: Same kinda thing as width, defaults to maximum number of lines of terminal
 (something to remember is that most fonts are 1x2 meaning they are twice as high as they
 are wide, this means that for it to render in a ~ square you need half the height as you
 do width) :)
-p --print : If the gif gets shown in the terminal (THIS SHOULD probably BE UPDATED TO BE
SWITCHED AROUND) it's more likely you will want to see it than not
-l --luminance : Currently I have two functions to calculate luminance, one actually
calculates the value the other uses a different (mostly worse calculation) but I've found
some images work best with one and worse with another, so options are good, this will almost
certainly become close to defunct with adding colour
-t --time : The time between frames, withou ta value it will extract that of the last frame in the
sequence,

## TODO list

We all love a good todo list, and this one is no different

[] - Colour output
[] - Handels interlaced gifs (they're more common than you'd think)
[] - More downsampling functions / options
[] - Upscaling for images smaller than terminal resolution
[] - Using half height block and background / foreground colour to double vertical
resolution at a loss of the "ascii" aestetic
