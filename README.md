# Setup

`Setup` is a command line tool to automate and coordinate the execution of programs and scripts.
This tool only works on linux at the moment. Windows support may come sometimes in the future.

## Installing debian package

A debian package is currently only available for the amd64-architecture.

To install the debian package, download the .deb-file and run:

```bash
sudo dpkg -i <setupfilename>
```

## Installing with cargo

To install setup with cargo, clone this repository and run the following command in the root directory of this project:

```bash
cargo install --path .
```

## Schemafile

After installation the project schemafile is available in `/usr/share/setup/projectfileschema.json`.
This file can be used with editors supporting json schema validation for a better experience when creating
the project file.

Example in VS-Code:

Add the following to your `settings.json` file for schema validation.

```json
{
    "json.schemas": [
        {
            "fileMatch": ["<filetomatch>"],
            "url": "/use/share/setup/projectfileschema.json"
        }
    ]
}
```
