# FaithType

Windowsの文字レンダリングにおいて、ビットマップを削除し、gridfitを無効にするようにフォントを修正します。

[\[ English \]](Readme.md) [\[ 正體中文 \]](Readme-zh_tw.md) [\[ 简体中文 \]](Readme-zh_cn.md) \[ 日本語 \]

## ダウンロード

[ダウンロード](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip)

## 説明

Windows 10バージョン1703 (Creators Update) から、内蔵のTrueTypeレンダラーが縦方向のアンチエイリアスを対応するようになりました。各方向に4段階のグレースケールの濃淡しかないにも関わらず、特に日本語の表現力が劇的に向上します。残念なことに、この機能は一部のフォントの特定のサイズに対してのみ有効です。

このツールを使えば、ほとんどのTrueTypeアウトラインフォントに対して縦方向のアンチエイリアス処理を有効にすることができ、ビットマップも削除できるので、高DPIディスプレイでの書体の見栄えが格段に良くなります。

## 比較

下の画像は100%の表示倍率で見る必要があります。

<a href="https://raw.githubusercontent.com/m13253/FaithType/master/img/comparison.svg">![（画像）FaithTypeの使用前後の比較](img/comparison.svg)</a>

**Before：** 新たなWindows 10バージョン21H1のインストール。

**After：** フォントはFaithTypeで修正しました。また、LCDフィルターを無効にしました。

**FreeType：** Fedora Linux 34。ヒンティングとLCDフィルターを無効にしました。

## 使い方（オートマチック）

1. [最新のリリース](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip)をダウンロードします。

2. ZIPファイルを展開します。

3. `Auto-Patch-Then-Install.ps1`を右クリックし、「PowerShellで実行」を選択します。

4. スクリプトが自動的にフォントを処理し、`C:\Patched Fonts`にインストールするのを待ちます。

5. 終了すると、ウィンドウの下部に「Press Enter to exit」と表示されます。エラーメッセージが表示されていないか確認してください。

6. システムを再起動します。

アンインストールする場合は、`Auto-Uninstall.ps1`を右クリックして「PowerShellで実行」を選択します。その後、システムを再起動します。再インストールの前には必ずアンインストールを行ってください。

## 使い方（マニュアル）

1. [最新のリリース](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip)をダウンロードします。

2. ZIPファイルを展開します。

3. ターミナル（コマンドプロンプトまたはPowerShell）を開きます。
   ```ps1
   cd FaithTypeのパス\
   ```

4. 使用方法を読みます（英語）：
   ```ps1
   .\faithtype.exe --help
   ```

5. フォントファイルを処理します：
   ```ps1
   mkdir "C:\Patched Fonts"
   .\faithtype.exe "C:\Windows\Fonts\meiryo.ttc" -o "C:\Patched Fonts\meiryo.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msgothic.ttc" -o "C:\Patched Fonts\msgothic.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msmincho.ttc" -o "C:\Patched Fonts\msmincho.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothB.ttc" -o "C:\Patched Fonts\YuGothB.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothL.ttc" -o "C:\Patched Fonts\YuGothL.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothM.ttc" -o "C:\Patched Fonts\YuGothM.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothR.ttc" -o "C:\Patched Fonts\YuGothR.ttc"
   ```

6. **オプション：** ヒンティングを再生成したい場合は、このステップで[ttfautohint](https://www.freetype.org/ttfautohint/#download)を使います。

7. 修正したフォントファイルを開く可能を確認します。

8. レジストリを設定します：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "Meiryo & Meiryo Italic & Meiryo UI & Meiryo UI Italic (TrueType)"="C:\Patched Fonts\meiryo.ttc"
   "MS Gothic & MS UI Gothic & MS PGothic (TrueType)"="C:\Patched Fonts\msgothic.ttc"
   "MS Mincho & MS PMincho (TrueType)"="C:\Patched Fonts\msmincho.ttc"
   "Yu Gothic Bold & Yu Gothic UI Semibold & Yu Gothic UI Bold (TrueType)"="C:\Patched Fonts\YuGothB.ttc"
   "Yu Gothic Light & Yu Gothic UI Light (TrueType)"="C:\Patched Fonts\YuGothL.ttc"
   "Yu Gothic Medium & Yu Gothic UI Regular (TrueType)"="C:\Patched Fonts\YuGothM.ttc"
   "Yu Gothic Regular & Yu Gothic UI Semilight (TrueType)"="C:\Patched Fonts\YuGothR.ttc"
   ```

9. システムを再起動します。

## FAQ

- **なぜビットマップを削除するのですか？**

  OpenTypeはグレースケールのビットマップを対応しますが、Windowsに内蔵されているビットマップフォントはすべて白黒バージョンしか含んでいません。この問題は、すでに画素数が多いHi-DPIディスプレイでも発生します。

  また、ビットマップフォントの中には、ClearTypeを無効にしたときにしか読み込まれないものがあります。通常は問題ありません。しかし、現在のClearTypeは壊れているので（下記参照）、ビットマップを削除する必要があります。

- **なぜgridfitを無効にするのか？**

  Windowsに内蔵されているフォントは、TrueTypeヒンティングを使用してアウトラインを強いgridfit傾向があり、高解像度の画面でも文字がブロック化てしまいます。古いVGAコネクタやキャリブレーションされていないディスプレイを使用している人の中には、そのスタイルのコントラストとシャープネスが高いと主張する人もいます。しかし、私にとっては、強いgridfitの文字は、数分読むと頭が痛くなります。

- **なぜ`gasp`テーブルをパッチするのか？**

  Windows 10バージョン1703と1903の間のどのバージョンかで**ClearTypeが壊れてしまった**からです。「ClearTypeテキスト チューナー」でLCDフィルタを無効にすることができなくなっています。[\[ 調査 \]](https://github.com/bp2008/BetterClearTypeTuner/wiki/ClearType-Investigations)

  LCDフィルタは、本来1:1の表示倍率で液晶ディスプレイ用に設計されています。つまり、プロジェクター、テレビ、ペンタイル配列のディスプレイ、ピボット対応のディスプレイ、スクリーンキャスト、スクリーンショット、Web会議、スライドショー、DPIスケールが必要の古いアプリなどでは、LCDフィルタを使用してはいけません。**「Microsoftペイント」のテキストツールで描かれた文字も、LCDフィルタがかかっている。**

  また、ClearTypeを完全に無効にすると、不具合が発生したり、特定のフォントが読むにくくなります。解決策の一つは、これらのフォントをFaithTypeで処理することです。FaithTypeは、ClearTypeをオフにしたまま、双方向のアンチエイリアシングを有効にするように`gasp`テーブルをパッチします。

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

  Monotypeは、インクが紙の上に広がった後の形状にはなく、IBMタイプライターの細い金属活字に参考してデザインしました。この理由より、「Bitstream Courier 10 Pitch」に代用してください。

- バージョン4.55以前の**DynaLab MingLiU**（華康細明體）、**DynaLab BiauKai**（華康標楷體）：

  このフォントはTrueTypeヒンティングが必要です。ヒンティングを削除しないでください。

## ライセンスと保証

本ソフトは、[GPLライセンス](LICENSE)（バージョン3以降）に基づいてリリースされています。

本ソフトは、本人、マイクロソフト、または第三者のアプリケーションのバグにより、お使いのOSが不安定になる可能性があります。本ソフトは、少しでもお役に立てることを願って、無償で公開しておりますので、いかなる種類の保証もなく提供する。

## 謝辞

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)
