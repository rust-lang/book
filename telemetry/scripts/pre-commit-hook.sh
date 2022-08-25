NOTEBOOKS=$(git diff --cached --name-only --diff-filter=ACM | grep --color=never '.*\.ipynb')
for NB in $NOTEBOOKS
do
  jupyter nbconvert --ClearOutputPreprocessor.enabled=True --inplace $NB
  git add $NB
done
