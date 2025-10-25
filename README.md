# KIKODO - Kiko the Deployment Operator
## Intoduction
Kiko is a Github App, which manages deployments. It listens for pull requests, and if some happen, it will deploy the preview version of the applicaion. Then it will send link of the deployment.
When some new commits is pushed to the pull request, it will redeploy the application.
When the pull request is closed, and merged, it will deploy the main branch into production.
All this will be deployed via docker containers (locally on the same server as bot, or remotelly via tcp connection)

## Features
- [x] Create simple clap interface to load some env variables/arguments
- [x] Listen for data send to webhook
- [ ] Parse data into some structures
- [ ] Handle adding/removing apllication to repos
- [ ] Probably use some database to store data about repos, deployments, etc.
- [ ] Handle pull request events
- [ ] Send simple response to pull request message (via octocrab)
- [ ] Research about tokio event loop and asynchronous programming, and have some event queues
- [ ] Use 'docker' create to connect to docker daemon