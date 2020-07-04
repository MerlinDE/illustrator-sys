# Adobe Illustrator SDK Rust Binding

Download [Adobe Illustrator SDK](https://console.adobe.io/downloads).

Define an `AISDK_ROOT` environment variable that contains the path to your Adobe Illustrator SDK.
Typically the directory structure will look like this:
```
IllustratorSDK
├── docs
├── illustratorapi
    ├── ate
    ├── illustrator
    ├── pica_sp
├── legalnotices
├── Readme.txt
├── samplecode
    ├── ...
    ├── common
    ├── pipl
    ├── ...
├── tools
```


## Getting help

Let Illustrator at least talk a bit about what's going on.

* defaults write com.adobe.CSXS.9 PlayerDebugMode 1
* defaults write com.adobe.CSXS.9 LogLevel 4 (1-6)


* less "/Users/[USERNAME]/Library/Logs/CSXS/CEP9-ILST.log"
* less "/Users/[USERNAME]/Library/Application Support/Adobe/ExtensibilityLibrary/Log/ExtensibilityLibrary.log"