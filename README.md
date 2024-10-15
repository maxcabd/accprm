# accprm
A CLI tool to batch add new accessory entries for Ultimate Ninja Storm CONNECTIONS

Adds entries to MessageInfo, AccessoryParam, and AccessoriesParam.

#  Usage
```
accprm 0.1.0

USAGE:
    accprm --json <JSON> --dir <DIR>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --json <JSON> The path of the .json that will contain the accessory config file
    -d, --dir <DIR>   The path of your data_win32 directory

ARGS:
    <JSON>
    <DIR>
```

Here is the format of the config .json file that's required:
```json
{
    "accessories": [
       {
        "accessory_name": "Madara's Fan",
        "accessory_id": "a_accessory_200",
        "modelcode": "amdf1",
        "icon_filepath": "C:/Desktop/png2xfbin/icon_acc_200.png",
        "head_a": false,
        "head_b": false,
        "face": false,
        "eyes": false,
        "back": true,
        "back_pocket": false,
        "tail": false,
        "arms": false
        },
        {
        "accessory_name": "Assailant's Mask",
        "accessory_id": "a_accessory_201",
        "modelcode": "amdk1",
        "icon_filepath": "C:/Desktop/png2xfbin/icon_acc_201.png",
        "head_a": false,
        "head_b": true,
        "face": true,
        "eyes": false,
        "back": false,
        "back_pocket": false,
        "tail": false,
        "arms": true
      }
    ]
}
```
