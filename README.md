# Rotaract Event Tracker

This is a small utility meant for tracking the amount of time each member of a Rotaract Club spends doing events and socials in a year. It is generalised, so it doesn't need to be for a Rotaract club specifically, that's just what I'm using it for.

IMPORTANT:
This project is not affiliated with or endorsed by Rotary or Rotaract in any way.

## Developing

This project has been developed with [Helix](https://github.com/helix-editor/helix) and [Zellij](https://github.com/zellij-org/zellij) on [NixOS](https://nixos.org). It is recommended that you use the `nix-shell` configuration for developing this project.
```bash
git clone https://github.com/BenDundon/event-tracker.git
nix-shell
zellij
```
From within Zellij, run `hx` to start the helix editor. This will ensure all dependencies are met.
If you wish to use another editor, you can, but please ensure that it is compatible with `nix-shell`. If you are not using `nix-shell`, please ensure you read the definition file well and have all dependencies for your local environment installed.
