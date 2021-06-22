# FaithType

通过修改字体来删除 Windows 文字渲染的点阵字并禁用 gridfit。

[\[ English \]](Readme.md) [\[ 正體中文 \]](Readme-zh_tw.md) \[ 简体中文 \] [\[ 日本語 \]](Readme-ja.md)

## 简介

自从 Windows 10 的 1703 版本（创造者更新）开始，自带的 TrueType 渲染器能支持纵向抗锯齿了。虽然只有 30 级灰度，但也大幅优化了中文显示的效果。可惜的是，只有寥寥数个字体在特定尺寸下能默认使用这个功能。

通过本工具，你可以给几乎任何字体启用纵向抗锯齿，同时还能移除内嵌的点阵字，让高分屏下文字更加耐看。

## 如何使用本工具？

1. 下载 [Rust 编译器](https://www.rust-lang.org/tools/install)。

2. 下载 FaithType 的源代码。

3. 打开终端（命令提示符或 PowerShell 皆可）：
   ```ps1
   cd path_to_FaithType
   cargo build --release
   cd .\target\release
   ```

4. 阅读使用说明（英文）：
   ```ps1
   .\faithtype.exe --help
   ```

5. 处理字体文件：
   ```ps1
   mkdir "C:\Patched Fonts"
   .\faithtype.exe "C:\Windows\Fonts\simsun.ttc" -o "C:\Patched Fonts\simsun.ttc" --remove-bitmap --remove-hinting --modify-gasp
   ```
   根据你是否要移除或重建 hinting，选择 `--remove-hinting` 或者 `--keep-hinting` 两者其一。

6. **可选：** 如果你打算重建 hinting 指令，在这一步使用 [ttfautohint](https://www.freetype.org/ttfautohint/#download) 进行 hinting 重建。

7. 确认 Windows 可以正常打开和预览修改后的字体文件。

8. 修改注册表：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts]
   "SimSun & NSimSun (TrueType)"="C:\Users\<USERNAME>\AppData\Local\Microsoft\Windows\Fonts\simsun.ttc"
   "宋体 & 新宋体 (TrueType)"="C:\Users\<USERNAME>\AppData\Local\Microsoft\Windows\Fonts\simsun.ttc"
   ```

9. 重启系统。

## FAQ

- **为何要删除点阵字？**

  虽然 OpenType 支持灰阶点阵字，Windows 自带的点阵字只有黑白二值。就算是像素数量够多的高分屏，本应能载入清晰平滑的矢量字，却被点阵拖累而无能为力。

  此外，ClearType 关闭后有些本来不会加载的点阵字会被加载起来。本来不需要理会它们，但现在 ClearType 出了 bug（见下文），所以得删除这些点阵字了。

- **为何要禁用 gridfit？**

  Windows 自带的字体喜欢大量使用 TrueType hinting 来将曲线对齐到像素格点，导致高分屏上文字也显得疙疙瘩瘩。有些人可能还用着 VGA 模拟连接线和未校色的屏幕，却在反驳这种渲染风格有高对比度和锐度。但对我来说，稍微阅读一小会儿严重 gridfit 的文字就会头晕脑胀。

- **为何要修改 `gasp` 表？**

  因为在 Windows 10 的 1703 至 21H1 之间的某个版本，**ClearType 坏掉了**。确切来说，通过 “ClearType 文本调谐器” 也关不掉 LCD 滤波器了。

  LCD 滤波器本来是给液晶显示器在 1:1 缩放比例下使用的。换句话说，在投影仪、电视机、PenTile 屏、可旋转设备、视频录像、屏幕截图、远程会议、PPT 幻灯片、DPI 缩放后的应用程序等等环境下不应该使用 LCD 滤波器。**然而微软 “画图” 的文本框工具画出来的文字竟然有 LCD 滤波的彩边。**

  如果你完全关掉 ClearType，一部分自带字体会变得不清晰，还会出现 bug。解决方案之一就是使用 FaithType 来处理字体文件，通过修改 `gasp` 表，你可以保持 ClearType 关着的状态下向系统请求纵横双向抗锯齿。

## 笔划好细呀！

在旧版 (Mac) OS X 中，TrueType 渲染器会加粗笔划来在低分屏上维持稳定的对比度。但 Windows 并不会做笔划加粗。我的个人经验是易认的文字渲染需要至少 1.3px 笔划宽度，但 Windows 自带的字体都太细了。

1. 把 gamma 从 `0x00000898` 改成 `0x00000708` 可能有用：
   ```reg
   Windows Registry Editor Version 5.00

   [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Avalon.Graphics\DISPLAY1]
   "GammaLevel"=dword:00000708
   ```

2. 另外一个办法是使用 [ttfautohint](https://www.freetype.org/ttfautohint/) 来生成加粗笔划的 hinting 指令。

3. 终极解决方案是去买一台高分屏。

## 常见问题：

- **Microsoft Yahei** (微软雅黑)、**Microsoft Jhenghei** (微軟正黑體)、**Meiryo** (メイリオ)：

  它们的 hinting 指令不支持纵向抗锯齿。你需要移除 hinting。

- **Monotype Courier New**:

  蒙纳的 Courier New 是按照 IBM 打字机的金属活字来设计的，而并不是按照油墨在纸上渗开之后的形状来设计的。总体来说这个字体很垃圾，请改用 “Bitstream Courier 10 Pitch”。

- 4.55 版本之前的 **DynaLab MingLiU** (華康細明體)

  此字体使用 TrueType hinting 来动态组字。请不要移除 hinting。

## 许可证和担保

本软件依 [GPL 许可证](LICENSE)第 3 版或更新版发布。

本软件可能会因我的 bug、微软的 bug 或者第三方应用的 bug 导致操作系统不稳定现象的发生。本软件是本着希望能造福大众的目的而无偿发布的，因此不包含任何形式的担保，也即是说用户需要对使用本软件造成的后果负担责任。

## 鸣谢

- [RemoveBitmapFont](https://github.com/tkumata/RemoveBitmapFont)
- [ttfautohint](https://www.freetype.org/ttfautohint/)
