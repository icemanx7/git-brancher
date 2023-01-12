# Introduction

This is a simple tool I created to automate branch creation for me based on our work branching convention.

## example

```
git checkout -b CS-123445-some-piece-of-work
```

What I needed help with was to automatically create the branch by having the description part generate with the dashes. 

```
git-brancher CS-12345 some piece of work
```

Would then generate and apply the command: 

```
git checkout -b CS-123445-some-piece-of-work
```
