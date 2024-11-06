# ymm4_renamer

"ymm4_renamer"は、ゆっくりまんじゅうなどの動く立ち絵をymm4に読み込める形式に変換するCLIです。

## 導入方法
.exe実行ファイルを1つダウンロードするだけで、導入は完了します。  
※ アンチウイルスソフトウェアにダウンロード・実行が弾かれる可能性があります。

## 実行ファイル
|Asset|Note|
|---|---|
|[ymm4_renamer.exe](https://github.com/SL9-1994/ymm4_renamer/releases/latest/download/ymm4_renamer.exe)|x86_64-pc-windows-gnu|

## 使い方

```
> .\ymm4_renamer.exe -h
Usage: ymm4_renamer.exe [OPTIONS] --input-folder <INPUT_FOLDER>

Options:
  -i, --input-folder <INPUT_FOLDER>    Path of the folder where each part of the illustration is stored
  -r, --rename-option <RENAME_OPTION>  Please select the format of your illustration [default: kitsune-yukkuri] [possible values: kitsune-yukkuri]
  -l, --log-level <LOG_LEVEL>          Sets the logging level [default: info] [possible values: error, warn, info, debug, trace]
  -h, --help                           Print help
  -V, --version                        Print version
```

### 基本的なコマンド

```
> .\ymm4_renamer.exe -i <input_folder> -l <log_level> -r <rename_option>

-i <input_folder>: リネームする対象のフォルダ
-l <log_level>: ログレベル (debug, info, warn, errorなど)
-r <rename_option>: 使用するリネームオプション（例: KitsuneYukkuri）
```

## 対応フォーマット
1. [旧きつねゆっくり](http://www.nicotalk.com/charasozai_kt.html)  
2. 追加してほしいフォーマットがあれば、下記のブログ記事に対して要望のコメントを送ってもらえると助かります。  
