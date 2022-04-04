#!/usr/bin/env bash

cargo build --release
mv -u ./target/release/cloneit .

printf "\e[1;96m \n[+] Removing artifacts...\n \e[0m"
rm -rf target

printf "\e[1;96m \n[+] Now, you can run cloneit like this:\n \e[0m"
printf "\e[1;93m \t./cloneit <github_url> \e[0m"
