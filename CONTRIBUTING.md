# Contributing to Lamda RED

Bug fixes, diagram examples, documentation and translations, and improvements
to the compiler or VS Code extensions are welcome. Please follow the
[Code of Conduct](./CODE_OF_CONDUCT.md) in project spaces.

## Discuss or report a change

Check [open issues](https://github.com/Asncodes-80/lamda_red/issues) for related
work. For a larger change to the Drizzle language, CLI, or output, open an issue
to discuss the intended behavior before implementing it. For a small fix, feel
free to open a pull request directly.

For a bug report, include the expected and actual behavior and the steps that
reproduce it. If the CLI is involved, include `lmdred --version`, your operating
system, a small `.zz` example, and the command you ran. For a preview issue,
include the extension version as well.

## Set up a branch

Fork the repository on GitHub, clone your fork, and branch from `main`. For
example, replace `YOUR-USERNAME` and use a short branch name that describes your
change.

```sh
git clone https://github.com/YOUR-USERNAME/lamda_red.git
cd lamda_red
git switch -c docs/clarify-preview-setup
```

Follow the [developer guide](./DEVELOPER.md) to build and test your changes.

## Write commit messages

Every commit must have a short subject and a self-contained body, separated by
a blank line. Use an established prefix such as `docs:` or `feat(preview):`.
Explain what changed, why the change was needed, and how it affects behavior
where relevant. Include enough detail for someone reading `git log` to
understand the change on its own. Link related issues and record relevant
verification or compatibility details when they help explain the change.

```text
docs: clarify preview setup

Document how to open the preview extension in a VS Code Extension
Development Host. Explain how to select a local compiler so contributors
can verify live rendering against a build from their branch.
```

## Use AI tools responsibly

AI coding agents such as Claude Code and OpenAI Codex are welcome. Use them
where they help you learn, work faster, or improve the quality of a contribution.
Contributors at every experience level are welcome. If part of a change is
unclear to you, ask questions or start with a smaller change you can explain.

- **Review the full diff.** You are responsible for everything you submit.
  Understand how the change works, why it belongs in the project, and be ready
  to answer reviewers' questions in your own words.
- **Verify the result.** Run the relevant checks in the
  [developer guide](./DEVELOPER.md) and exercise the affected behavior. In the
  pull request, report the checks you actually ran, their results, and any
  relevant checks you could not run.
- **Protect users and their data.** Review affected code for security and
  privacy regressions, especially around file handling, command execution, and
  dependencies. Keep credentials, personal data, and confidential material out
  of AI prompts. Make sure any third-party material you submit can be shared
  under the project's license and carries any required attribution.
- **Be open about substantial AI assistance.** If an agent drafted or rewrote
  a substantial part of the contribution, briefly say where it helped in the
  pull request description. Routine spelling or formatting assistance needs no
  disclosure.

## Open a pull request

1. Push your branch to your fork and open a pull request against `main` in
   [Asncodes-80/lamda_red](https://github.com/Asncodes-80/lamda_red).
2. Explain the change, link the relevant issue, and list the checks you ran.
   For a change to rendering or preview, include a small example or a screenshot
   that shows the expected result.
3. Update the docs and examples affected by the change. Keep corresponding
   content in the [localized READMEs](./locales/) in sync when you edit the
   main README.
4. Address review feedback by pushing updates to the same branch.
