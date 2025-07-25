#bin/bash
# before running the script add your remote by running:
# git remote add github "your@github.email"
git add .
git commit -m "$1"
git push 
git push github