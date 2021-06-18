# FaithType

Disable bitmap and gridfit for Windows font rendering.

## Description

Since Windows 10 version 1703 (Creators Update), its builtin TrueType renderer
now supports vertical anti-aliasing. However, it is only enabled for selected
fonts at selected sizes.

By using this tool, you can enable vertical anti-aliasing for almost any
TrueType outline font, also removing the embedded bitmap, to make the text
look much better on HiDPI displays.

## How to use this tool?

1. Download [Rust compiler](https://www.rust-lang.org/tools/install).

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

5. (Optional: If you want to remove hinting, download and use
   [ttfautohint](https://www.freetype.org/ttfautohint/#download) at this step.)

6. Process the font:
   ```reg
   mkdir C:\XXXXXX
   .\faithtype.exe C:\Windows\Fonts\simsun.ttc -o C:\XXXXXX\simsun.ttc --remove-bitmap --remove-hinting --modify-gasp
   ```

7. (Optional: If you want to recreate hinting instructions, use
   [ttfautohint](https://www.freetype.org/ttfautohint/#download) at this step
   instead of Step 5.)

8. Install modified fonts user-wide. Putting them under
   ```
   C:\Users\<USERNAME>\AppData\Local\Microsoft\Windows\Fonts
   ```
   would work

9. Change the registry:
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "SimSun & NSimSun (TrueType)"="C:\Users\<USERNAME>\AppData\Local\Microsoft\Windows\Fonts\simsun.ttc"
   "宋体 & 新宋体 (TrueType)"="C:\Users\<USERNAME>\AppData\Local\Microsoft\Windows\Fonts\simsun.ttc"
   ```

10. Restart the system.

## FAQs

- **Why remove bitmap?**

  Although OpenType supports grayscale bitmap, all Windows built-in bitmap
  fonts only contain black-and-white version. This happens even on HiDPI
  displays, where the number of pixels are enough to produce legible
  anti-aliased text.

  Also, some bitmap fonts only load when ClearType is off. Normally we don't
  care about them. But since current ClearType is broken (see below), we now
  need to remove these bitmaps.

- **Why disable gridfit?**

  Windows built-in fonts tend to use TrueType hinting to heavily gridfit the
  outlines, rendering the text blocky and pixelated even on HiDPI screens. Some
  people, who may be using an old VGA connector or an uncalibrated display,
  claims this style maintains color contrast and sharpness. But to me, heavily
  gridfitted text gives me headache after reading for a few minutes.

- **Why patch the `gasp` table?**

  Because **ClearType is now broken** somewhere between Windows 10 version 1703
  and 21H1. The LCD filter can no longer be turned off through the “ClearType
  Text Tuner”.

  LCD filter is originally designed for LCD screens with 1:1 viewing scale.
  Meaning you should not use LCD filter on projectors, televisions, Pentile
  displays, rotatable displays, video recordings, screenshots, remote meetings,
  slideshows, or DPI-scaled applications. **But now, even the Text Tool in
  “Microsoft Paint” can only draw LCD-filtered text.**

  If you turn ClearType altogether, you also get bugs and lose legibility with
  certain built-in fonts. One solution is to process those fonts with
  FaithType, which patches the `gasp` table to request bidirectional
  anti-aliasing while you can keep ClearType turned off.

## Strokes looks too thin!

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

3. The ultimate solution is to buy a HiDPI display.

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

- **DynaLab MingLiU** (華康新細明體) before version 4.55:

  The font rendering requires TrueType hinting. Don't remove hinting.

## License and warranty

This software is released under the [GPL license](LICENSE), version 3 or
later.

This software may cause instability to your operating system due to bugs on my
side, or on Microsoft's side, or on any third-paty application's side. This
software is released only under the hope that it may be helpful, and comes
with no warranty in case anything happens with your usage of this software.

## Acknowledgement

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)
