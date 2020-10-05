# PFS: **P**oor man's large **F**ile **S**torage

An experiment to replace Git LFS with two text files.

PFS enables you to store (large) files somewhere else and not in your git repository. These files are referenced using entries in the `.gitignore` file of the current git repository.

## Before you continue...

This is an experiment and has a number of flaws compared to Git LFS. If you just want to use LFS on a different storage, take a look at the [LFS Test Server](https://github.com/git-lfs/lfs-test-server) or other LFS server implementations.

## Setup

You can directly execute the `git-pfs` executable but a more convenient method is to create a [git alias](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases):

    git config --global alias.pfs '!/path/to/git-pfs'

Once you have done that you have to let PFS know where the storage for your repository is located. Inside your git repository set the `pfs.storage` value in the git config:

``` sh
# an absolute path is required
git config pfs.storage /absolute/path/to/storage
```

## Usage

PFS works by invoking the three main commands:

+ `add`: Add a file to the storage or update an existing one
+ `unlink`: Remove a file link to the storage but keep them on the fileystem
+ `sync`: Synchronize with the storage (this has to be run manually after each git operation)

When you call PFS for the first time it will create some lines in your `.gitignore`.

**IMPORTANT**: PFS relies heavily on some contents of your `.gitignore`. Do not edit the lines between `#>pfs` and `.pfstrack` on your own!

## How it works

PFS uses two files: `.gitignore` and `.pfstrack` to keep track of which files have to be uploaded/removed/downloaded.

`.pfstrack` is ignored by git and therefore not changed when git operations happen. Therefore it always represents the last state when PFS was run. By comparing `.gitignore` and `.pfstrack` PFS can determine which files have to be transferred.

In the `.gitignore` the filenames and their sha256 hashes are stored (the order is important). The hashes are used as filenames in the storage.

## TODO

+ Make incremental copies to the storage
+ Recursive add/unlink
+ Integrate with git hooks
+ Better error handling
+ Use a git library?
+ Keep order of lines in `.gitignore`
