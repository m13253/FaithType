# FaithType

透過修改字型檔來刪除 Windows 文字彩現的點陣字並停用 gridfit。

[\[ English \]](Readme.md) \[ 正體中文 \] [\[ 简体中文 \]](Readme-zh_cn.md) [\[ 日本語 \]](Readme-ja.md)

## 簡介

自從 Windows 10 的 1703 版本（創造者更新），內建的 TrueType 彩現器開始支援縱向反鋸齒了。雖然每個方向只有 4 級灰階，但已大幅提升了中文的呈現效果。可惜只有寥寥幾個字型檔的選定尺寸能夠使用這個功能。

使用本工具，你可以給幾乎任何 TrueType 字型啟用縱向反鋸齒，同時刪除內嵌的點陣字，讓字型在 Hi-DPI 熒幕上顯示更加美觀。

## 比較

以下的影像必須以 100% 縮放比例觀看。

![FaithType 使用前後的比較](img/comparison.svg)

**使用前：** 全新安裝的 Windows 10，21H1 版本。

**使用後：** FaithType 修改後的字型檔，同時關閉 LCD 濾波器。

**FreeType：** Fedora Linux 34，同時關閉 hinting 和 LCD 濾波器。

## 使用方法（自動）

1. 下載[最新釋出版本](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip)。

2. 解壓縮下載的 ZIP 檔案。

3. 右鍵點選 `Auto-Patch-Then-Install.ps1`，選擇「用 PowerShell 執行」。

4. 等待指令碼自動處理字型檔並安裝至 `C:\Patched Fonts`。

5. 完成後，視窗最下方會提示「Press Enter to exit」。請檢查是否有錯誤訊息。

6. 重新啟動系統。

如果需要卸載，請右鍵點選 `Auto-Uninstall.ps1`，選擇「用 PowerShell 執行」，然後重新啟動。如果需要重新安裝，請先卸載再安裝。

## 使用方法（手動）

1. 下載[最新釋出版本](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip)。

2. 解壓縮下載的 ZIP 檔案。

3. 打開終端機（命令提示字元或者 PowerShell 皆可）：
   ```ps1
   cd FaithType之路徑\
   ```

4. 閱讀使用說明（英文）：
   ```ps1
   .\faithtype.exe --help
   ```

5. 處理字型檔：
   ```ps1
   mkdir "C:\Patched Fonts"
   .\faithtype.exe "C:\Windows\Fonts\msjh.ttc" -o "C:\Patched Fonts\msjh.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msjhbd.ttc" -o "C:\Patched Fonts\msjhbd.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msjhl.ttc" -o "C:\Patched Fonts\msjhl.ttc"
   .\faithtype.exe "C:\Windows\Fonts\mingliu.ttc" -o "C:\Patched Fonts\mingliu.ttc"
   .\faithtype.exe "C:\Windows\Fonts\mingliub.ttc" -o "C:\Patched Fonts\mingliub.ttc"
   ```

6. **可省略：** 若要重建 hinting 指令，在這一步使用 [ttfautohint](https://www.freetype.org/ttfautohint/#download)。

7. 確認 Windows 可以正常打開和預覽修改後的檔案。

8. 修改登錄檔：
   ```
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "Microsoft JhengHei & Microsoft JhengHei UI (TrueType)"="C:\Patched Fonts\msjh.ttc"
   "Microsoft JhengHei Bold & Microsoft JhengHei UI Bold (TrueType)"="C:\Patched Fonts\msjhbd.ttc"
   "Microsoft JhengHei Light & Microsoft JhengHei UI Light (TrueType)"="C:\Patched Fonts\msjhl.ttc"
   "MingLiU & PMingLiU & MingLiU_HKSCS (TrueType)"="C:\Patched Fonts\mingliu.ttc"
   "MingLiU-ExtB & PMingLiU-ExtB & MingLiU_HKSCS-ExtB (TrueType)"="C:\Patched Fonts\mingliub.ttc"
   ```

9. 重新啟動系統。

## FAQ

- 為何要刪除點陣字？

  儘管 OpenType 支援灰階點陣字，Windows 內建的所有點陣字型只有黑白兩色。即使是用上足夠能讓反鋸齒文字清晰好認的 Hi-DPI 顯示器也不例外。

  此外，關閉 ClearType 之後，一些原本不會載入的點陣字就會載入。本來不需要理會這件事，但因為 ClearType 現在有 bug（見下文），我們還是得刪除這些點陣字。

- 為何要停用 gridfit？

  Windows 內建的字型喜歡大量使用 TrueType hinting 來將輪廓對齊到像素格點上，導致文字疙疙瘩瘩，即便 Hi-DPI 熒幕也迴天無力。有些人，一邊用着類比 VGA 訊號線和沒校色的熒幕，一邊反駁說這樣的彩現風格對比度強，文字銳利。但對我來說，嚴重 gridfit 的字閱讀幾分鐘便覺頭痛。

- 為何要修補 `gasp` 資料表？

  因為在 Windows 10 的 1703 和 1903 之間的某個版本開始，**ClearType 出現了 bug**。透過「ClearType 文字調整工具」已經關不掉 LCD 濾波器了。[\[ 相關調查 \]](https://github.com/bp2008/BetterClearTypeTuner/wiki/ClearType-Investigations)

  LCD 濾波器本來是設計給 1:1 倍率觀看的液晶顯示器的。也就是說在投影機、電視、PenTile 顯示器、可旋轉熒幕的裝置、影片檔、熒幕擷取檔、遠端會議、簡報投影片、DPI 縮放過的應用程序等場景裡，不應該用 LCD 濾波器。然而，現在 LCD 濾波器完全無法關閉，**甚至「小畫家」的文字工具只能畫出有 LCD 濾波的彩邊文字了。**

  如果你完全關閉 ClearType，一部分內建字型會變得不清晰，還會出現 bug。解決方法之一就是使用 FaithType 來處理這些字型檔，通過修補 `gasp` 資料表來在關閉 ClearType 的情況下仍然開啟縱橫雙向反鋸齒。

## 筆畫太細了！

在旧版本 (Mac) OS X 中，TrueType 彩現器會在低解析度熒幕上加粗筆畫來維持穩定的對比度。Windows 不會做筆畫加粗。我的個人經驗是，好認的文字彩現需要至少 1.3px 寬，而 Windows 內建的字型都太細了。

1. 將 gamma 值從 `0x00000898` 改為 `0x00000708` 能緩解：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Avalon.Graphics\DISPLAY1]
   "GammaLevel"=dword:00000708
   ```

2. 此外可以用 [ttfautohint](https://www.freetype.org/ttfautohint/#download) 來產生加粗筆畫的 TrueType hinting 指令。

3. 終極解決方法是買一台 Hi-DPI 顯示器。

## 常見問題

- **Microsoft Yahei** (微软雅黑)、**Microsoft Jhenghei** (微軟正黑體)、**Meiryo** (メイリオ)：

  它們的 hinting 指令不支援縱向反鋸齒。你需要移除 hinting。

- **Monotype Courier New**：

  蒙納的 Courier New 是參照 IBM 打字機的金屬活字來設計的，而不是參照墨水在紙張上滲透開來的輪廓來設計的。不要用筆畫纖細的這個版本，推薦換成「Bitstream Courier 10 Pitch」。

- 4.55 版本之前的 **DynaLab MingLiU**（華康細明體）、**DynaLab BiauKai**（華康標楷體）：

  這個字型依賴 TrueType hinting 來動態組字。請不要移除 hinting 指令。

## 授權條款和品質擔保

本軟體以 [GPL 授權條款](LICENSE)第三版或者更新版作為授權條款發行。

本軟體可能會因為我的 bug、微軟的 bug 或者第三方應用程式的 bug 導致作業系統工作不穩定。本軟體僅僅是以幫助大衆為目的無償地發行，不包含任何品質擔保，也不對使用本軟體造成的意外負責。

## 鳴謝

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)
