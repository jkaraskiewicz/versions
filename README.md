<h1>versions</h1>
<h2>Simple module based version control system</h2>

------------------------------------------------------------------------

## Introduction to _versions_

_versions_ is a simple VCS focused on maintaining multiple versions of files.
As such, it does not operate on commits as such, and does not keep track of changes within a single version.

_versions_ defines two notions - modules and versions. A module is bound to a single directory within the repository.
Within a module, multiple versions can be defined, so that the content of the directory is reflected based on a version selection.

To start working with _versions_ you need to initialize the repository first:

    $ versions init
    > Repository initialized successfully.

Now let's add some modules.

    $ mkdir sample_dir
    $ mkdir another_dir
    $ versions module add sample_dir
    > Module sample_dir added.

If you want your module to be named differently than the directory, you can use that command with 2 parameters.

    $ versions module add sample sample_dir
    > Module sample added.

Now as we added a module, let's select it. In that way, we can operate on that module's versions without specifying it every time.

    $ versions module select sample
    > Module sample selected.

When you add a new module, the default version of that module is created for you.
You can list versions of your module using:

    $ versions version list
    > default

Now let's add some files to our sample_dir directory.

    $ echo "First file!" > sample_dir/my_new_file.txt

Let's run the status command to see the changes in the workspace.

    $ versions version status
    > + sample_dir/my_new_file.txt
      --- original
      +++ modified
      @@ -0,0 +1 @@
      +First file!

Great, since there is no commits in _versions_, there is no need to save our work for now.

Let's create another version.

    $ versions version add new_version
    > Version new_version added.

And let's switch to that version:

    $ versions version select new_version
    > Version new_version selected.

Since we are selecting a different version now, the changes in the working copy were 'ammended' to the previous version state.
And our __new_version__ branches out from that state.

Now we can make changes to our new version.

    $ echo "Some text" >> sample_dir/my_new_file.txt

Let's check the status:

    $ versions version status
    > sample_dir/my_new_file.txt 
      --- original
      +++ modified
      @@ -0,0 +1 @@
      First file!
      +Some text

Great! Now let's switch back to the previous version.

    $ versions version select default.
    > Version default selected.

As before, our last changes before switching the version were 'ammended' to the previous one.
Now as we moved to the default version, our workspace is clean and the file has its previous content:

    $ cat sample_dir/my_new_file.txt
    > First file!

All our work was in a single module which corresponds to a directory.
Modules are independent and have separate set of versions.

In the first step, we created another directory. Let's bind it to a module.

    $ versions module add another another_dir
    > Module another added.

    $ versions module select another
    > Module another selected.

Now we start with a default version as we did before.

For more information about available commands, use the --help flag.

    $ versions --help
    > Simple version control system
    >
    > Usage: versions <COMMAND>
    >
    > Commands:
    > init         Initialize repository
    > module       Module commands
    > version      Version commands
    > list         Show modules and versions
    > completions  Generate shell completions
    > help         Print this message or the help of the given subcommand(s)

    > Options:
    > -h, --help     Print help
    > -V, --version  Print version
