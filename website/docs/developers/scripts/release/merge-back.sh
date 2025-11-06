#!/bin/bash
set -e

echo "WARNING: This will merge main into develop and push directly."
echo "This operation modifies the develop branch and cannot be easily undone."
echo ""

read -r -p "Are you sure you want to proceed? (y/N): " confirm
if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
    echo "Operation cancelled."
    exit 1
fi

echo "Merging main back to develop..."
BRANCH_NAME=tmp-merge-branch-"$(date +%s)"
git checkout develop
git pull origin develop
git checkout -b "${BRANCH_NAME}"
git merge main --no-edit
git push origin "${BRANCH_NAME}"
gh pr create --title "Merge main back to develop" --body "Automated merge of main into develop." --base develop --head "${BRANCH_NAME}"

echo "Successfully merged main back to develop"
