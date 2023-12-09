# Debugging - Launch Configuration

## The following attributes are mandatory for every launch configuration:

- type - the type of debugger to use for this launch configuration. Every installed debug extension introduces a type: node for the built-in Node debugger, for example, or php and go for the PHP and Go extensions.
- request - the request type of this launch configuration. Currently, launch and attach are supported.
- name - the reader-friendly name to appear in the Debug launch configuration dropdown.

## Here are some optional attributes available to all launch configurations:

- presentation - using the order, group, and hidden attributes in the presentation object, you can sort, group, and hide configurations and compounds in the Debug configuration dropdown and in the Debug quick pick.
- preLaunchTask - to launch a task before the start of a debug session, set this attribute to the label of a task specified in tasks.json (in the workspace's .vscode folder). Or, this can be set to ${defaultBuildTask} to use your default build task.
- postDebugTask - to launch a task at the very end of a debug session, set this attribute to the name of a task specified in tasks.json (in the workspace's .vscode folder).
- internalConsoleOptions - this attribute controls the visibility of the Debug Console panel during a debugging session.
- debugServer - for debug extension authors only: this attribute allows you to connect to a specified port instead of launching the debug adapter.
- serverReadyAction - if you want to open a URL in a web browser whenever the program under debugging outputs a specific message to the debug console or integrated terminal. For details see section Automatically open a URI when debugging a server program below.

## Many debuggers support some of the following attributes:

- program - executable or file to run when launching the debugger
- args - arguments passed to the program to debug
- env - environment variables (the value null can be used to "undefine" a variable)
- envFile - path to dotenv file with environment variables
- cwd - current working directory for finding dependencies and other files
- port - port when attaching to a running process
- stopOnEntry - break immediately when the program launches
- console - what kind of console to use, for example, internalConsole, integratedTerminal, or externalTerminal
