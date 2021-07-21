# FaithType

Modify fonts to remove bitmap and disable gridfit for Windows font rendering.

\[ English \] [\[ 正體中文 \]](Readme-zh_tw.md) [\[ 简体中文 \]](Readme-zh_cn.md) [\[ 日本語 \]](Readme-ja.md)

## Description

Since Windows 10 version 1703 (Creators Update), its built-in TrueType renderer
now supports vertical anti-aliasing. Despite there are only 4 levels of
grayscale shade on each direction, it dramatically improves text rendering,
especially for CJK languages. Sadly, it is only enabled for selected fonts at
selected sizes.

By using this tool, you can enable vertical anti-aliasing for almost any
TrueType outline font, also removing the embedded bitmap, to make the text
look much better on Hi-DPI displays.

## Usage (the automatic way)

1. Download the [latest release](https://github.com/m13253/FaithType/releases/download/latest/FaithType.zip).

2. Extract the downloaded ZIP file.

3. Right click `Auto-Patch-Then-Install.ps1` and select “Run with PowerShell”.

4. Wait for the script to automatically install patched fonts to
   `C:\Patched Fonts`.

5. When it finishes, you will see “Press Enter to exit” at the bottom of the
   window. Please check whether there are any error messages.

6. Restart the system.

If you want to uninstall, right click `Auto-Uninstall.ps1`, select “Run with
PowerShell”, then restart the system. Always uninstall before re-installing.

## Usage (the manual way)

1. Download [the Rust compiler](https://www.rust-lang.org/tools/install).

2. Download the source code of FaithType.

3. Open a terminal (either Command Prompt or PowerShell):
   ```ps1
   cd path_to_FaithType
   cargo build --release
   cd .\target\release
   ```

4. Read how to use the tool:
   ```ps1
   .\faithtype.exe --help
   ```

5. Process the font:
   ```ps1
   mkdir "C:\Patched Fonts"
   .\faithtype.exe "C:\Windows\Fonts\msgothic.ttc" -o "C:\Patched Fonts\msgothic.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msjh.ttc" -o "C:\Patched Fonts\msjh.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msjhbd.ttc" -o "C:\Patched Fonts\msjhbd.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msjhl.ttc" -o "C:\Patched Fonts\msjhl.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msyh.ttc" -o "C:\Patched Fonts\msyh.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msyhbd.ttc" -o "C:\Patched Fonts\msyhbd.ttc"
   .\faithtype.exe "C:\Windows\Fonts\msyhl.ttc" -o "C:\Patched Fonts\msyhl.ttc"
   .\faithtype.exe "C:\Windows\Fonts\simsun.ttc" -o "C:\Patched Fonts\simsun.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothB.ttc" -o "C:\Patched Fonts\YuGothB.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothL.ttc" -o "C:\Patched Fonts\YuGothL.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothM.ttc" -o "C:\Patched Fonts\YuGothM.ttc"
   .\faithtype.exe "C:\Windows\Fonts\YuGothR.ttc" -o "C:\Patched Fonts\YuGothR.ttc"
   ```

6. **Optional:** If you want to regenerate hinting instructions, use
   [ttfautohint](https://www.freetype.org/ttfautohint/#download) at this step.

7. Make sure Windows can open and preview the modified font file.

8. Change the registry:
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "Microsoft JhengHei & Microsoft JhengHei UI (TrueType)"="C:\Patched Fonts\msjh.ttc"
   "Microsoft JhengHei Bold & Microsoft JhengHei UI Bold (TrueType)"="C:\Patched Fonts\msjhbd.ttc"
   "Microsoft JhengHei Light & Microsoft JhengHei UI Light (TrueType)"="C:\Patched Fonts\msjhl.ttc"
   "Microsoft YaHei & Microsoft YaHei UI (TrueType)"="C:\Patched Fonts\msyh.ttc"
   "Microsoft YaHei Bold & Microsoft YaHei UI Bold (TrueType)"="C:\Patched Fonts\msyhbd.ttc"
   "Microsoft YaHei Light & Microsoft YaHei UI Light (TrueType)"="C:\Patched Fonts\msyhl.ttc"
   "MS Gothic & MS UI Gothic & MS PGothic (TrueType)"="C:\Patched Fonts\msgothic.ttc"
   "SimSun & NSimSun (TrueType)"="C:\Patched Fonts\simsun.ttc"
   "Yu Gothic Bold & Yu Gothic UI Semibold & Yu Gothic UI Bold (TrueType)"="C:\Patched Fonts\YuGothB.ttc"
   "Yu Gothic Light & Yu Gothic UI Light (TrueType)"="C:\Patched Fonts\YuGothL.ttc"
   "Yu Gothic Medium & Yu Gothic UI Regular (TrueType)"="C:\Patched Fonts\YuGothM.ttc"
   "Yu Gothic Regular & Yu Gothic UI Semilight (TrueType)"="C:\Patched Fonts\YuGothR.ttc"
   ```

9. Restart the system.

## FAQs

- **Why remove bitmap?**

  Although OpenType supports grayscale bitmap, all Windows built-in bitmap
  fonts only contain black-and-white version. This happens even on Hi-DPI
  displays, where the number of pixels are already enough to produce legible
  anti-aliased text.

  Also, some bitmap fonts only load when ClearType is off. Normally we don't
  care about them. But since current ClearType is broken (see below), we now
  need to remove these bitmaps.

- **Why disable gridfit?**

  Windows built-in fonts tend to use TrueType hinting to heavily gridfit the
  outlines, rendering the text blocky and pixelated even on Hi-DPI screens.
  Some people, who may be using an old VGA connector or an uncalibrated
  display, claims this style maintains color contrast and sharpness. But to me,
  heavily gridfitted text gives me headache after reading for a few minutes.

- **Why patch the `gasp` table?**

  Because **ClearType is now broken** somewhere between Windows 10 version 1703
  and 1903. The LCD filter can no longer be turned off through the “ClearType
  Text Tuner”.
  [\[ Investigations \]](https://github.com/bp2008/BetterClearTypeTuner/wiki/ClearType-Investigations)

  LCD filter is originally designed for LCD screens with 1:1 viewing scale.
  Meaning you should not use LCD filter on projectors, televisions, PenTile
  displays, rotatable displays, video recordings, screenshots, remote meetings,
  slideshows, DPI-scaled applications, etc. **But now, even the Text Tool in
  “Microsoft Paint” can only draw LCD-filtered text.**

  If you turn ClearType off altogether, you also get bugs and lose legibility
  with certain built-in fonts. One solution is to process those fonts with
  FaithType, which patches the `gasp` table to request bidirectional
  anti-aliasing while you can keep ClearType turned off.

## Strokes look too thin!

On older versions of (Mac) OS X, the TrueType renderer widens the strokes to
maintain a stable contrast on Low-DPI displays. Windows can't do stroke
widening. My personal experience with graphics indicates 1.3px is the minimum
width for legible font rendering, but most built-in fonts in Windows are way
too thin.

1. Changing the gamma level from `0x00000898` to `0x00000708` may help:
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Avalon.Graphics\DISPLAY1]
   "GammaLevel"=dword:00000708
   ```

2. Another way is to use [ttfautohint](https://www.freetype.org/ttfautohint/)
   to generate TrueType hinting to widen the strokes.

3. The ultimate solution is to buy a Hi-DPI display.

## Common issues

- **Microsoft Yahei** (微软雅黑), **Microsoft Jhenghei** (微軟正黑體),
  and **Meiryo** (メイリオ):

  Their hinting instructions have issues with vertical anti-aliasing. You need
  to remove hinting.

- **Monotype Courier New**:

  The Monotype Courier New is designed according to the metal letter used in
  IBM typewriters, instead of the real appearance on a paper, where the ink
  spreads and emboldens the strokes.
  Therefore, this Monotype font is garbage, replace it with “Bitstream Courier
  10 Pitch”.

- **DynaLab MingLiU** (華康細明體) before version 4.55, **DynaLab BiauKai** (華康標楷體):

  The font rendering requires TrueType hinting. Don't remove hinting.

## License and warranty

This software is released under the [GPL license](LICENSE), version 3 or
later.

This software may cause instability to your operating system due to bugs on my
side, or on Microsoft's side, or on any third-paty application's side. This
software is released free-of-charge only under the hope that it may be helpful,
thus comes with no warranty in case anything happens during your usage of this
software.

## Acknowledgement

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)
