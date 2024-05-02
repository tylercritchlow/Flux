<div align="center" markdown="1"> 
 <img src="assets/logo.png" width="250" alt="Flux Logo">
 <h1>Flux</h1>

</div>


> [!NOTE]  
> Flux is not a finished product at all. It is still in development and is not ready for use. I will post a github release when it is ready for use.

Flux is a simple version control system that is easy to use and easy to understand. As a non-programmer friendly version control system, it is designed in a way to be easily traversed through the .flux directory. What that means is you can tell what is going on in the repository by looking at the .flux directory. 

## Features
- [ ] Inititalize Repo
  - [x] Create .flux directory
  - [x] Create config file
  - [x] create branches directory
- [ ] Add files
- [ ] Remove files
- [ ] Commit files
- [ ] Branches
- [ ] Merge branches
- [ ] View commit history
- [ ] Resolve merge conflicts wizard/helper
- [ ] Push to remote
- [ ] Pull from remote
- [ ] Clone remote


### Remote repository hosting

Soon, I will create a remote repository hosting service for flux. It will be free and open source, and you will have to self host.

- [ ] HTTP
- [ ] SSH
<!--

## Installation

**Linux**
Snap:
```bash
snap install flux
```

**macOS**
```bash
brew install flux
```

**Windows**
```bash
winget install flux
```-->

## Usage
```
Basic Commands:

init     Initialize a new Flux repository
add      Stage files for the next commit
rm       Remove files from the repository
commit   Commit staged changes
status   Show the status of files
log      Display the commit history
branch   List, create, or delete branches
switch   Switch to a different branch
merge    Merge changes from another branch

Collaboration Commands:

push     Push local changes to a remote repository
pull     Pull changes from a remote repository
clone    Clone a remote repository

Help Commands:

help     Display this help message
help <command>   Get detailed help for a specific command

For more information, visit:

[Link to Flux documentation]
```

## Contributing to Flux

We welcome contributions from everyone. Before contributing, please read our [Code of Conduct](CODE_OF_CONDUCT.md). We expect everyone to follow the code of conduct anywhere in Flux's project's codebases, issue trackers, chatrooms, and mailing lists.

### How to Contribute

1. Fork the repository on GitHub
2. Clone your fork to your local machine
3. Create a new branch for your contribution
4. Make your changes and commit them
5. Push your changes to your fork
6. Submit a pull request from your fork back to the main repository
7. Wait for your pull request to be reviewed and merged

For more details on how to contribute, please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License

Flux is released under the MIT License. See the bundled [LICENSE](LICENSE.md) file for details.
