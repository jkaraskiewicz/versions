<h1>versions</h1>
<h2>Simple module based version control system</h2>

## Introduction to _versions_

_versions_ is a simple Version Control System (VCS) focused on maintaining multiple states (versions) of files within specific directories (modules). Unlike systems like Git, it does not operate on commits in the traditional sense and doesn't track granular changes *within* a single version. Instead, it manages distinct snapshots of a module's content.

_versions_ uses two primary concepts: modules and versions. A module is directly linked to a single directory within your repository. Within each module, you can define multiple versions. Selecting a specific version changes the content of the module's directory to match that version's snapshot.

To begin using _versions_, you first need to initialize a repository in your desired location:

```bash
    $ versions init
    > Repository initialized successfully.
```

After initializing the repository, the next step is to define modules linked to your project directories. Let's create a couple of directories first:

```bash
    $ mkdir sample_dir
    $ mkdir another_dir
    $ versions module add sample_dir
    > Module sample_dir added.
```

You can also assign a custom name to a module, distinct from its directory name, by providing two arguments to the `module add` command: the desired module name followed by the directory path.

```bash
    $ versions module add sample sample_dir
    > Module sample added.
```

With the module added, let's select it. Selecting a module sets it as the current context, allowing you to work with its versions without needing to specify the module name in subsequent commands.

```bash
    $ versions module select sample
    > Module sample selected.
```

Upon adding a new module, `versions` automatically creates an initial `default` version for it. You can list the available versions within the currently selected module using:

```bash
    $ versions version list
    > default
```

Now, let's add some content to the `sample_dir` directory, which belongs to our selected `sample` module:

```bash
    $ echo "First file!" > sample_dir/my_new_file.txt
```

To see how the current directory content differs from the selected version's baseline, use the `status` command:

```bash
    $ versions version status
    > + sample_dir/my_new_file.txt
      --- original
      +++ modified
      @@ -0,0 +1 @@
      +First file!
```

Excellent. Since `versions` doesn't use explicit commits, the current state of the directory is implicitly tied to the selected version (`default` in this case). You don't need a separate 'commit' step; the changes are part of this version until you switch.

Let's create a new version to capture a different state:

```bash
    $ versions version add new_version
    > Version new_version added.
```

Switch to the newly created version:

```bash
    $ versions version select new_version
    > Version new_version selected.
```

Selecting a different version implicitly saves the current state of the working directory to the previously selected version (`default`).
The working directory is then updated to match the state of the newly selected version (`new_version`),
which initially mirrors the state `default` was in when `new_version` was created.

And our __new_version__ branches out from that state.

With new_version selected, let's modify the file:

```bash
    $ echo "Some text" >> sample_dir/my_new_file.txt
```

Check the status again. The diff now compares the current state against the baseline of `new_version` (which included "First file!"):

```bash
    $ versions version status
    > sample_dir/my_new_file.txt 
      --- original
      +++ modified
      @@ -0,0 +1 @@
      First file!
      +Some text
```

Great! Now, let's switch back to the `default` version:

```bash
    $ versions version select default.
    > Version default selected.
```

Similar to the previous switch, the changes made while `new_version` was selected are now associated with `new_version`. The working directory (`sample_dir`) is restored to the state associated with the `default` version:

```bash
    $ cat sample_dir/my_new_file.txt
    > First file!
```

So far, all operations have been within the `sample` module, which manages the `sample_dir` directory. Remember that modules are independent entities, each maintaining its own distinct set of versions.

Recall that we created another directory, `another_dir`, earlier. Let's create and select a new module named `another` linked to this directory:

```bash
    $ versions module add another another_dir
    > Module another added.

    $ versions module select another
    > Module another selected.
```

As with the first module, creating the `another` module automatically generates a `default` version for it, which is now selected. You can manage versions for `another_dir` independently.

For a complete list of commands and options, use the `--help` flag:

```bash
    $ versions --help
    > Simple version control system
    >
    > Usage: versions <COMMAND>
    >
    > Commands:
    > init         Initialize repository
    > module       Module commands
    > version      Version commands
    > show         Show repository state (modules, versions)
    > completions  Generate shell completions
    > help         Print this message or the help of the given subcommand(s)

    > Options:
    > -h, --help     Print help
    > -V, --version  Print version
```
