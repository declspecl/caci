# caci (khaki (/ˈkakē/))

**C**ross-platform, **A**gnostic **C**ontinuous **I**ntegration

`caci` is a tool to orchestrate continuous integration pipelines on your local machine.
It is cross-platform and agnostic to any version control system.
By leveraging hooks in common VCS tools, `caci` provides the maximum flexibility to the user while emulating CI in the real world.
`caci` is like the Terraform/CDK to your local CI, but without the steep learning curve (and YAML 😨).

## Overview

For simplicity, I will assume the chosen VCS agent is Git for this explanation.
With `caci`, there are two main pieces to the puzzle:
1. `caci` scripts
2. Git hooks

The `caci` scripts are simply scripts that YOU write, and they are stored in the `.caci/scripts` directory.
Each script can be a casual `cargo test` or a complex set of Bash/Python/PowerShell/xyz logic.
These scripts are, assumedly, your CI steps, e.g. formatting, building, unit & integration testing, etc.
`caci` then uses these scripts together with Git hooks in order to provide an automated CI experience.

> So `caci` is just a wrapper around VCS hooks?

Yes!

> So why should I use `caci` then?

For numerous QoL features that `caci` provides:
- TOML configuration: declarative and easily customizable
- Decoupling of logic and hooks.
Want to mix and match different filter/check scripts, for example?
Want to change one script to `post-commit` instead of `pre-commit`?
These are made trivial with `caci`
- Clonability of hooks (`.git/hooks` is not cloned, but `.caci/scripts` will be).
Just `git clone` and `caci write`.
Now your CI pipeline is ready to go!
- Agnostic to each VCS
- Much more

## Features
- [x] TOML configuration
- [ ] CLI configuration interface (WIP)
- [ ] Custom script executors and hook output piping
- [ ] `caci` subdirectories

### Supported Version Control Systems
- [x] Native (None)
- [x] Git
- [ ] Mercurial
- [ ] SVN
