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
pub fn new(PathBuf) -> Result<T: AgentPlugin, U: Display>;
```
Yes, I know that's not a valid signature. But that's basically what is required.

TODO explain PathBuf and how to read configs

## Utility Functions
Are documented in inquisitor lib documentation NOT ONLINE YET

## Logging