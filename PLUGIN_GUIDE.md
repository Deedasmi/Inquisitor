# Plugin Creation Guide
This is a WIP guide to help developers create their own inquisitor plugins.

## Prerequisites
* rustc 1.26

## Naming convention
* inquisitor_agent_plugin
* inquisitor_receptor_plugin

## Semvar
Post 1.0:
* Major version is API version
* Minor version should increment on schema changes, or significant data changes
* Patch version is bugs/small changes to data (ie. trimming message)

## API Definition
Anything that would break a plugin. i.e. changing the function signature required in the next section.

## API Requirements
Must have a new function with following signature. Replace AgentPlugin with ReceptorPlugin for receptor plugins.
```
pub fn new(PathBuf) -> Result<impl AgentPlugin, impl std::fmt::Display>;
```
This is nightly-only syntax at the time of writing. You may use normal syntax, and we would recommend using the following syntax until issue #34511 is stabilized.
Otherwise your plugin must be compiled with nightly rust.

```
pub fn new(PathBuf) -> Result<Plugin, String>;
```

Assuming Plugin implements AgentPlugin this definition will work. You may also return a std::io::error, or any other error that implements Display.

TODO explain PathBuf and how to read configs

## Utility Functions
Are documented in inquisitor lib documentation NOT ONLINE YET

## Logging