#bin/bash
# before running the script add your remote by running:
# git remote add github "your@github.email"

#git config --global credential.helper store
#piscine-rust git:(main) ✗ git config --global user.email aberhili0man@gmail.com
#piscine-rust git:(main) ✗ git remote add github https://github.com/madaghaxx/piscine-rust

git add .
git commit -m "$1"
git push 
git push github