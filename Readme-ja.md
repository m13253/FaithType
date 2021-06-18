# FaithType

Windowsの文字レンダリングにおいて、ビットマップを削除し、gridfitを無効にするようにフォントを修正します。

[\[ English \]](Readme.md) [\[ 正體中文 \]](Readme-zh_tw.md) [\[ 简体中文 \]](Readme-zh_cn.md) \[ 日本語 \]

## 説明

Windows 10バージョン1703 (Creators Update) から、内蔵のTrueTypeレンダラーが縦方向のアンチエイリアスを対応するようになりました。グレースケールの濃淡が30段階しかないにも関わらず、特に日本語の表現力が劇的に向上します。残念なことに、この機能は一部のフォントの特定のサイズに対してのみ有効です。

このツールを使えば、ほとんどのTrueTypeアウトラインフォントに対して縦方向のアンチエイリアス処理を有効にすることができ、ビットマップも除去できるので、高DPIディスプレイでのテキストの見栄えが格段に良くなります。

## 使い方は？

1. [Rust言語のコンパイラ](https://www.rust-lang.org/tools/install)をダウンロードしてください。

2. FaithTypeのソースコードをダウンロードしてください。

3. ターミナル（コマンドプロンプトまたはPowerShell）を開きます。
   ```ps1
   cd path_to_FaithType
   cargo build --release
   cd .\target\release
   ```

4. 使用方法を読む（英語）：
   ```ps1
   .\faithtype.exe --help
   ```

5. **オプション：** ヒンティングを完全に除去したい場合は、[ttfautohint](https://www.freetype.org/ttfautohint/#download)をダウンロードして、このステップで `ttfautohint --dehint` を使ってください。

6. フォントファイルを処理します：
   ```ps1
   mkdir C:\XXXXXX
   .\faithtype.exe C:\Windows\Fonts\msgothic.ttc -o C:\XXXXXX\msgothic.ttc --remove-bitmap --remove-hinting --modify-gasp
   ```
   ヒンティングを削除するか維持するかによって、`--remove-hinting` または `--keep-hinting` のいずれかを使用する。

6. **オプション：** ヒンティングを再生成したい場合は、第5歩の代わりにここで[ttfautohint](https://www.freetype.org/ttfautohint/#download)を使う。

7. 修正したフォントをこのユーザーにインストールする。ここにはいいです：
   ```
   C:¥Users¥<ユーザー名>¥AppData¥Local¥Microsoft¥Windows¥Fonts
   ```

8. レジストリを設定してください：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "MS Gothic & MS UI Gothic & MS PGothic (TrueType)"="C:\Users\<ユーザー名>\AppData\Local\Microsoft\Windows\Fonts\msgothic.ttc"
   "ＭＳ ゴシック & MS UI Gothic & ＭＳ Ｐゴシック (TrueType)"="C:\Users\<ユーザー名>\AppData\Local\Microsoft\Windows\Fonts\msgothic.ttc"
   ```

10. システムを再起動してください。

## FAQ

（[英語版](Readme.md)を参照してください。）

## 筆画が細すぎます。

古いバージョンの(Mac) OS XのTrueTypeレンダラーは、低DPIディスプレイでコントラストを維持するために筆画を太くしました。Windowsでは、筆画を太くすることはできません。私の経験では、1.3pxが読みやすい最小限の太さですが、Windowsに内蔵のフォントはあまりにも細すぎます。

1. ガンマ値を `0x00000898` から `0x00000708` に変更するで改善可能：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Avalon.Graphics\DISPLAY1]
   "GammaLevel"=dword:00000708
   ```

2. 他の方法としては、[ttfautohint](https://www.freetype.org/ttfautohint/)を使ってTrueTypeヒンティングを再生成し、筆画を太くします。

3. 究極の解決策は、Hi-DPIモニターを購入することです。

## 一般的な問題

- **Microsoft Yahei**（微软雅黑）、**Microsoft Jhenghei**（微軟正黑體）、そして**Meiryo**（メイリオ）：

  このフォントのヒンティングは、縦方向のアンチエイリアスに不具合があります。ヒンティングを削除する必要があります。

- **Monotype Courier New**：

  Monotypeは、インクが紙の上に広がった後の形状にはなく、IBMタイプライターの細い金属活字に参考してデザインしました。この理由のて、「Bitstream Courier 10 Pitch」に代用してください。

- バージョン4.55以前の**DynaLab MingLiU**（華康細明體）：

  このフォントはTrueTypeヒンティングが必要です。ヒンティングを削除しないでください。

## ライセンスと保証

本ソフトウェアは、[GPLライセンス](LICENSE)（バージョン3以降）に基づいてリリースされています。

本ソフトウェアは、本人、マイクロソフト、または第三者のアプリケーションのバグにより、お使いのOSが不安定になる可能性があります。本ソフトウェアは、少しでもお役に立てることを願って、無償で公開しておりますので、いかなる種類の保証もなく提供する。

## 謝辞

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)