git add .
ping localhost-n 2 >nul
git commit -m 'Update'
ping localhost-n 2 >nul
git remote add origin https://github.com/daanbreur/AdventofCode2019.git
ping localhost-n 2 >nul
git pull origin master --allow-unrelated-histories
ping localhost-n 2 >nul
git push -u origin master
ping localhost-n 2 >nul
echo Done :D
pause