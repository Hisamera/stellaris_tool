# Stellaris localisation tool
Right now it can scan localisation files from Stellaris, as a single file, or entire folder structure. It can also generate folder structure to mimick that of Stellaris.

This is my first project that I delivered, what you see (as in source code) is only 7 to max 10% of everything that I've written, rest has been deleted.
As horrible as that source code may be, know that it went through heavy refactor process.
If you\`re only interested in the program [here](https://github.com/Hisamera/stellaris_tool/releases) you go.
I\`ll accept any criticism, even horrible ones, and not helpful ones. If you spot a bug (as in, this should work and my tool screams or other way around).

# Roadmap
0.2 -> Proper rendering of [Actor.GetAllianceName], [admiral.GetHerHis], $B$Something, £energy£. So you can see your string and format it without starting stellaris.
0.3 -> First GUI, it WILL be on Vulkan no matter how long it takes me. As I plan on making games in future, this is a must for me. Sorry.

# How to help.
Biggest help would be using it, and proposing additional functionality. As right now I don\`t have conscience to ask for money. As I have a specific plan for version 0.5 you won\`t understand how HUGE help that will be until you see it. So don\`t hesitate to say something to me. It can be through github issue, on Stellaris forum, on [Steam](steamcommunity.com/id/Hisamera), or email (I\`m thinking of a way to fool spam bots, so there is no email right now).

# Help
This is a direct copy and paste of help from program
  Everything that is printed here is subject to change prior to 1.0

  Also please remember that C:\my awesome folder is not synonymus with
  "C:\my awesome folder" (notice the quotes) first one will be parsed as
  ["C:\my", "awesome", "folder"] while the second one as
  ["C:\my awesome folder"]. All paths have to be absolute
  If you\`re new, try "-h -h"

  chainable: Means you can chain arguments so that they work differently together
  if you supply multiple arguments then program will \`consume\` them until nothing is left

  List of possible arguments
  [nothing]
  /h
  \h
  help
  -h    Prints whole documentation (for specific information use -h [argument]), possible arguments.
  overall treat this as an API documentation. If this argument (or help, /h etc)
  is found anywhere in the list every other argument is disregarded.

  chainable: Yes with every argument; Can be used separately: Yes

  requirements: nothing, or a supported argument

  example: -h
  example: -h -a
\-------------
  -g    generate the folder structure that Stellaris the game utilizes

  chainable: NO; Can be used separately: Yes

  requirements: root folder has to point to a valid drive or directory
  It does not have to point to an existing folder or folder structure

  example: folder C:\testing exists but C:\testing\stellaris doesn't
  stellaris_localisation_tool -g "C:\testing\stellaris\newproject"
  creates EVERY folder and subfolder mimicking Stellaris main folder structure.
\-------------
  -a    Scan every .yml file in folder and its subfolders for parsing errors and version checking

  chainable: Yes with -o; Can be used separately: Yes

  requirements: root folder has to point to a valid drive and existing directory

  example: folder C:\testing\localisation exists
  stellaris_localisation_tool -a "C:\testing\localisation"
\-------------
  -o    Overwrites default extension form .yml to whatever you like

  chainable: Yes with -a; Can be used separately: NO

  requirements: -a argument has to be supplied, and it has to be before or after -a argument

  example: folder C:\testing\localisation exists
  stellaris_localisation_tool -a "C:\testing\localisation" -o ""
  (only files with no extensions will be parsed. e.g. file but not file.e)
\-------------
  -s    Scan single file at given path for parsing errors and version checking

  chainable: NO; Can be used separately: Yes

  requirements: file has to exist, file extension has to supplied (.yml, .txt or nothing)

  example: file C:\testing\localisation\test.yml exists
  stellaris_localisation_tool -s "C:\testing\localisation\test.yml"

  example: file C:\testing\localisation\test exists
  stellaris_localisation_tool -s "C:\testing\localisation\test"
\-------------
