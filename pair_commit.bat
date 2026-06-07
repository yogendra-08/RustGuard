@echo off
echo ====================================
echo GitHub Pair Extraordinaire Helper
echo ====================================

set /p BRANCH=Enter branch name: 
set /p MSG=Enter commit message: 

echo.
echo Creating branch...
git checkout -b %BRANCH%

echo.
echo Adding files...
git add .

echo.
echo Creating commit with co-author...
git commit -m "%MSG%" -m "Co-authored-by: yashdudhe-28 <dudheyash28@gmail.com>"

IF %ERRORLEVEL% NEQ 0 (
    echo.
    echo Commit failed! Check if there are any changes to commit.
    pause
    exit /b
)

echo.
echo Pushing branch...
git push -u origin %BRANCH%

IF %ERRORLEVEL% NEQ 0 (
    echo.
    echo Push failed!
    pause
    exit /b
)

echo.
echo Creating Pull Request...
gh pr create --fill

echo.
echo ====================================
echo Done! Open GitHub and merge the PR.
echo ====================================
pause