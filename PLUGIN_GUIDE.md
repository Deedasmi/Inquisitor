# Plugin Creation Guide
This is a WIP guide to help developers create their own inquisitor plugins.

## Prerequisites
Inquisitor requires Rust version 1.26 to compile. A loose goal of the
project is to be compilable on stable Rust. This is not a strict requirement
and plugin authors may use nightly at their own discretion.

## Naming convention
Plugins should prepend agent plugin names with "inquisitor_agent_", and
similiarily receptor plugins with "inquisitor_receptor_". TODO Justify.

Two examples follow:
* inquisitor_agent_alive
* inquisitor_receptor_sync_check

## Semvar
Prior to Inquisitor release 1.0 we recomend the following version convention:
* Major version should be 0. Until inquisitor_lib is 1.0, your plugin cannot be considered stable.
* Minor version should match inquisitor_lib. This clearly signals what version your plugin is for.
* Patch version is what you will use for all development.

After the 1.0 release you may use a more classic interpetation of semvar:
* Major version should match inquisitor_lib. 
* Minor version should increment on schema changes, or significant data
changes. Anything that would break metric or analysis of your plugins responses.
* Patch version is bugs/small changes to data (ie. trimming message)

## API Definition
We define our API as anything that would break a plugin. i.e. changing 
the function signatures required in the next section. Changes to 
Agent/Receptor communication ARE NOT considered API changes as plugins 
should not be using the Status struct in the library anyway. As such, certain methods or
structs within inquisitor_lib may change definition on a patch version. These 
'internal APIs' will be clearly noted in the documentation.

## API Requirements
All plugins must implement either AgentPlugin or ReceptorPlugin outlined
[here](NOT ONLINE YET). This function signature may change in the future with [issue #34511](https://github.com/rust-lang/rust/issues/34511) to allow non-string Errs.
 
Additionally, the type that implements this trait needs to be called "Plugin"
or your plugin must contain a `pub type Plugin = $YOURPLUGIN` alias.
This is because the plugin initialization calls `$YOURPLUGIN::Plugin::new(PathBuf)`.

The given PathBuf should contain the location of all configurations. You should .push()
the name of your configuration file to create the full path. Then you may read your configuration. 
We recomend using the utility function outlined in the next section instead of reading the file manually.

## Utility Functions
Are documented in inquisitor lib documentation NOT ONLINE YET. Of particular note is the 
`read_cfg` function. This function allows you to define a struct that matches your configuration
and simply call `read_cfg::<$YOURCONFIGSTRUCT>(&PathBuf)` to receive a result containing your configuration.

## Logging
Inquisitor uses [env_logger](https://github.com/sebasmagri/env_logger). However, Plugins should not depend on env_logger. Simply use [Log](https://github.com/rust-lang-nursery/log)
and the binaries will filter logs appropriately.

