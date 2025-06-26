# Get the complete list of Visual Studio Code Extensions with version

Rename repo name on GitHub

## Motivation

- Motivation to see how we are doing

## Reason

- For fun, out of curiosity, to learn, to browse the market with their own tools, etc.
  
## Sources

- stackoverflow - How to programmatically retrieve the list of Visual Studio Code extensions on the Marketplace? [![alt text][1]](https://stackoverflow.com/questions/71742538/how-to-programmatically-retrieve-the-list-of-visual-studio-code-extensions-on-th)

## Installed python version local

```bash
> python3 --version
Python 3.11.2
```

## Python virtual environments

- Creation of virtual environments [![alt text][1]](https://docs.python.org/3/library/venv.html)
- How to Create a Python Virtual Environment(Step-by-Step Guide) [![alt text][1]](https://www.geeksforgeeks.org/create-virtual-environment-using-venv-python/)

### create
  
```bash
python3 -m venv .venv
```

### activate - How venvs work [![alt text][1]](https://docs.python.org/3/library/venv.html#how-venvs-workl)

```bash
# source <venv>/bin/activate
source .venv/bin/activate
```

### deactivate virtual environments

```bash
# inside the virtual environments
# (.venv) trapapa@debian$ deactivate
deactivate
```

> [!Note] How does "cat << EOF" work in bash? [![alt text][1]](https://stackoverflow.com/questions/2500436/how-does-cat-eof-work-in-bash)
> <!-- -->
> ```bash
>cat <<EOF > print.sh
>#!/bin/bash
>echo \$PWD
>echo $PWD
>EOF
>```
><!-- -->

## copy code example from article

- 01_test_eof.py
<!-- -->
```bash
cat <<EOF > python/01_test_eof.py
print("Hallo python!")
EOF
```
<!-- -->
- 02_org_from_stackoverflow_article.py
<!-- -->
```bash
cat <<EOF > python/02_org_from_stackoverflow_article.py
# FROM HERE
# https://stackoverflow.com/questions/71742538/how-to-programmatically-retrieve-the-list-of-visual-studio-code-extensions-on-th
import requests
from requests.adapters import HTTPAdapter, Retry


def get_vscode_extensions(max_page=10000, page_size=100,
                          include_versions=True, include_files=True, include_category_and_tags=True, include_shared_accounts=True, include_version_properties=True,
                          exclude_non_validated=False, include_installation_targets=True, include_asset_uri=True, include_statistics=True,
                          include_latest_version_only=False, unpublished=False, include_name_conflict_info=True, api_version='7.2-preview.1', session=None):
    if not session:
        session = requests.session()

    headers = {'Accept': f'application/json; charset=utf-8; api-version={api_version}'}

    flags = 0
    if include_versions:
        flags |= 0x1

    if include_files:
        flags |= 0x2

    if include_category_and_tags:
        flags |= 0x4

    if include_shared_accounts:
        flags |= 0x8

    if include_shared_accounts:
        flags |= 0x8

    if include_version_properties:
        flags |= 0x10

    if exclude_non_validated:
        flags |= 0x20

    if include_installation_targets:
        flags |= 0x40

    if include_asset_uri:
        flags |= 0x80

    if include_statistics:
        flags |= 0x100

    if include_latest_version_only:
        flags |= 0x200

    if unpublished:
        flags |= 0x1000

    if include_name_conflict_info:
        flags |= 0x8000

    for page in range(1, max_page + 1):
        body = {
            "filters": [
                {
                    "criteria": [
                        {
                            "filterType": 8,
                            "value": "Microsoft.VisualStudio.Code"
                        }
                    ],
                    "pageNumber": page,
                    "pageSize": page_size,
                    "sortBy": 0,
                    "sortOrder": 0
                }
            ],
            "assetTypes": [],
            "flags": flags
        }

        r = session.post('https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery', json=body, headers=headers)
        r.raise_for_status()
        response = r.json()

        extensions = response['results'][0]['extensions']
        for extension in extensions:
            yield extension

        if len(extensions) != page_size:
            break


def main():
    retry_strategy = Retry(
        total=3,
        backoff_factor=1,
        status_forcelist=[429, 500, 502, 503, 504],
        allowed_methods=["HEAD", "GET", "OPTIONS"]
    )
    adapter = HTTPAdapter(max_retries=retry_strategy)
    session = requests.Session()
    session.mount("https://", adapter)
    session.mount("http://", adapter)

    for extension in get_vscode_extensions(session=session):
        extension_name = extension['extensionName']
        extension_description = extension['extensionName']
        extensions_versions = extension['versions']
        extensions_statistics = dict({(item['statisticName'], item['value']) for item in extension['statistics']})
        extension_publisher_username = extension['publisher']['publisherName']

        for extension_version_info in extensions_versions:
            extension_version = extension_version_info['version']
            extension_artifact_download_url = f'https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{extension_publisher_username}/vsextensions/{extension_name}/{extension_version}/vspackage'
            print(extension_name, extension_description, extension_version, extension_artifact_download_url, extensions_statistics['install'])


if __name__ == '__main__':
    main()
EOF
```

### install necessary packages

```bash
pip3 install requests
pip3 install requests.adapters 
```
<!-- -->

### Run inside /tmp folder

```bash
/home/trapapa/complete_list_of_visual_studio_code_extensions_with_version/.venv/bin/python /home/trapapa/complete_list_of_visual_studio_code_extensions_with_version/python/02_org_from_stackoverflow_article.py >/tmp/output.txt
```
<!-- -->

#### Grep all vscode-language-pack from output.txt
<!-- -->
```bash
grep -Hrn 'vscode-language-pack' /tmp/output.txt >/tmp/vscode-language-pack.txt
```
<!-- -->

Bash - extract a string between two patterns inside a file

https://regex-generator.olafneumann.org/?sampleText=&flags=i

### grep vscode extention from file

grep -oP 'vsextensions/\K[^/]+' /tmp/output.txt

<!--TODO check se https://github.com/MathiasStadler/rust_ib_async/blob/master/PROJECT_PATH.md -->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email,please -->
[1]: img/link_symbol.svg
