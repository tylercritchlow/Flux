<div align="center" markdown="1"> 
 <img src="assets/bitmap.png" width="250" alt="Flux Logo">
 <h1>Flux</h1>

</div>

Flux is a simple version control system that is easy to use and easy to understand. As a non-programmer friendly version control system, it is designed in a way to be easily traversed through the .flux directory. You can tell what is going on in the repository by looking at the .flux directory. 

Everything is stored in the .flux directory, each branch is a directory, and each commit is a file. The file contains the commit message, the author, and the date. the file name is the commit hash. Once you open the file, you will see the changes made in that commit.

.branches-lock is a directory that contains a file for each branch. These files are used in the background to revert to, checkout, and switch to branches. these files should not be edited by the user, if edited expect the whole repository to break.

The config file is a toml file that contains the configuration for the repository. It will be able to be edited in a way to change how the repository behaves. i.e. the default branch, the remote repository, how it handles merge conflicts, auto push, auto new branch on commit, etc.

The .flux directory is not meant to be edited by the user, but it is meant to be easily traversed through. The user should be able to easily understand what is going on in the repository by looking at the .flux directory. The only time you should edit in the .flux directory is to change the config file.

## Features
- [ ] Inititalize Repo
```
 - [x] Create .flux directory
 - [x] Create config file
 - [ ] create branches directory
```
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

 **Debian/Ubuntu**
```bash
sudo apt install flux
```

**Arch Linux**
```bash
sudo pacman -S flux
```

**Fedora**
```bash
sudo dnf install flux
```

**Void Linux**
```bash
sudo xbps-install -S flux
```

**Gentoo**
```bash
sudo emerge flux
```

**FreeBSD**
```bash
sudo pkg install flux
```

**OpenSUSE**
```bash
sudo zypper install flux
```

**macOS**
```bash
brew install flux
```

**Windows**
```bash
winget install flux
```
-->

## Usage
```
Basic Commands:

init     Initialize a new Flux repository
add      Stage files for the next commit
remove   Remove files from the repository
commit   Commit staged changes
status   Show the status of files
log      Display the commit history
branch   List, create, or delete branches
checkout Switch to a different branch
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
