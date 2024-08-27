#!/bin/bash

cargo make
sudo rm /usr/local/bin/penrose
sudo cp ./target/debug/penrose /usr/local/bin/penrose
