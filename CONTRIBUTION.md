# Contribution Guide for Polify

## Branching Strategy
We use the [Gitflow Workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow). 
* **`master`**: Production-ready code only.
* **`dev`**: The main active development branch.
* **`feat/...`**, **`fix/...`**: Branches for specific features or bug fixes (branched off `dev` and merged back via PR).

## Versioning
We use [Semantic Versioning (SemVer)](https://semver.org/). 
Releases and tags follow the `MAJOR.MINOR.PATCH` format (e.g., `v1.2.0`).

## Commit Structure
Please format your commit messages to keep our history clean and readable. Prefix your commits with one of the following:
* **`feat:`** A new feature
* **`fix:`** A bug fix
* **`docs:`** Documentation changes
* **`refactor:`** Code structure changes that neither fix a bug nor add a feature
* **`chore:`** Tooling, configuration, or dependency updates

*Example:* `feat: add payment polling`

## Issue Structure
When opening an issue, please format it clearly so anyone can jump in and help. A good issue should include:
* **Title:** A short, descriptive summary.
* **Type:** Is this a [Feature], [Bug], or [Chore]?
* **Description:** A detailed explanation of the feature or the bug.
