# Local development workflow script
echo '=== KM-Box Git Workflow ==='
echo 'Add, commit, and push changes:'
git add .
git commit -m "UPDATE: "
git push origin main
echo ''
echo '✓ Changes pushed to GitHub!'
echo ''
echo 'To sync on Pi, run:'
echo '  ssh pi5 "~/sync-pi.sh"'
